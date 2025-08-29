//! Deku implementation for `VecDeku`
use alloc::borrow::Cow;
use alloc::format;
use alloc::vec::Vec;

use deku::ctx::{Endian, Limit};
use deku::reader::Reader;
use deku::writer::Writer;
use deku::{DekuError, DekuReader, DekuWriter, no_std_io};

use crate::{InternalValue as _, SevenBitU32, Size, VecDeku, VecLayout};

impl<T: Clone> VecDeku<T> {
    /// Read data from reader
    #[inline]
    fn from_reader_impl<'a, R, Ctx>(
        reader: &mut Reader<R>,
        endian: Endian,
        layout: VecLayout,
        inner_ctx: Ctx,
    ) -> Result<Self, DekuError>
    where
        Ctx: Copy,
        T: Clone + DekuReader<'a, Ctx>,
        R: no_std_io::Read + no_std_io::Seek,
    {
        let limit = match layout {
            VecLayout::FixedLength(size) => Limit::from(size),
            VecLayout::LengthPrefix(prefix) => {
                Limit::from(read_size(reader, endian, prefix)?)
            }
            VecLayout::End => Limit::End,
        };
        <Vec<T>>::from_reader_with_ctx(reader, (limit, inner_ctx)).map(Into::into)
    }

    /// Read data to writer
    #[inline]
    fn to_writer_impl<W, Ctx>(
        &self,
        writer: &mut Writer<W>,
        endian: Endian,
        layout: VecLayout,
        inner_ctx: Ctx,
    ) -> Result<(), DekuError>
    where
        Ctx: Copy,
        T: Clone + Default + DekuWriter<Ctx>,
        W: no_std_io::Write + no_std_io::Seek,
    {
        match layout {
            VecLayout::End => self.internal_ref().to_writer(writer, inner_ctx),
            VecLayout::LengthPrefix(prefix) => {
                write_length_prefix(self, writer, endian, prefix, inner_ctx)
            }

            VecLayout::FixedLength(size) => {
                write_fixed_length(self, writer, size, inner_ctx)
            }
        }
    }
}

/// Read expected size from stream
#[inline]
fn read_size<R>(
    reader: &mut Reader<R>,
    endian: Endian,
    prefix: Size,
) -> Result<usize, DekuError>
where
    R: no_std_io::Read + no_std_io::Seek,
{
    Ok(match prefix {
        Size::U8 => <u8>::from_reader_with_ctx(reader, endian)? as usize,
        Size::U16 => <u16>::from_reader_with_ctx(reader, endian)? as usize,
        Size::U32 => <u32>::from_reader_with_ctx(reader, endian)? as usize,
        Size::U32_7Bit => {
            let size: u32 = <SevenBitU32>::from_reader_with_ctx(reader, ())?.into();
            size as usize
        }
    })
}

/// Write length-prefixed data into stream
#[inline]
fn write_length_prefix<T, W, Ctx>(
    data: &VecDeku<T>,
    writer: &mut Writer<W>,
    endian: Endian,
    prefix: Size,
    inner_ctx: Ctx,
) -> Result<(), DekuError>
where
    Ctx: Copy,
    T: Clone + Default + DekuWriter<Ctx>,
    W: no_std_io::Write + no_std_io::Seek,
{
    let max_size: usize = match prefix {
        Size::U8 => u8::MAX as usize,
        Size::U16 => u16::MAX as usize,
        Size::U32 | Size::U32_7Bit => u32::MAX as usize,
    };

    let len = data.internal_ref().len();

    if len > max_size {
        return Err(DekuError::Assertion(Cow::from(format!(
            "Encoded string length cannot exceed {max_size} bytes"
        ))));
    }

    // buffer len is not above corresponding type size,
    // so truncation is safe

    match prefix {
        Size::U8 =>
        {
            #[allow(clippy::cast_possible_truncation)]
            (len as u8).to_writer(writer, endian)
        }
        Size::U16 =>
        {
            #[allow(clippy::cast_possible_truncation)]
            (len as u16).to_writer(writer, endian)
        }
        Size::U32 =>
        {
            #[allow(clippy::cast_possible_truncation)]
            (len as u32).to_writer(writer, endian)
        }
        Size::U32_7Bit => {
            #[allow(clippy::cast_possible_truncation)]
            let size = SevenBitU32::new(len as u32);
            size.to_writer(writer, ())
        }
    }?;

    data.internal_ref().to_writer(writer, inner_ctx)
}

/// Write fixed-length buffer into stream
#[inline]
fn write_fixed_length<T, W, Ctx>(
    data: &VecDeku<T>,
    writer: &mut Writer<W>,
    size: usize,
    inner_ctx: Ctx,
) -> Result<(), DekuError>
where
    Ctx: Copy,
    T: Clone + Default + DekuWriter<Ctx>,
    W: no_std_io::Write + no_std_io::Seek,
{
    let len = data.internal_ref().len();
    if len > size {
        return Err(DekuError::Assertion(Cow::from(format!(
            "Encoded string length cannot exceed {size} bytes"
        ))));
    }

    data.internal_ref().to_writer(writer, inner_ctx)?;

    let default = T::default();
    for _ in len..size {
        default.to_writer(writer, inner_ctx)?;
    }

    Ok(())
}

impl<'a, V> DekuReader<'a, (Endian, VecLayout)> for VecDeku<V>
where
    V: Clone + DekuReader<'a, Endian>,
{
    #[inline]
    fn from_reader_with_ctx<R: no_std_io::Read + no_std_io::Seek>(
        reader: &mut Reader<R>,
        ctx: (Endian, VecLayout),
    ) -> Result<Self, DekuError> {
        Self::from_reader_impl(reader, ctx.0, ctx.1, ctx.0)
    }
}

impl<'a, V, Ctx> DekuReader<'a, (Endian, VecLayout, Ctx)> for VecDeku<V>
where
    Ctx: Copy,
    V: Clone + DekuReader<'a, (Endian, Ctx)>,
{
    #[inline]
    fn from_reader_with_ctx<R: no_std_io::Read + no_std_io::Seek>(
        reader: &mut Reader<R>,
        ctx: (Endian, VecLayout, Ctx),
    ) -> Result<Self, DekuError> {
        Self::from_reader_impl(reader, ctx.0, ctx.1, (ctx.0, ctx.2))
    }
}

impl<'a, V, Ctx> DekuReader<'a, (Endian, (VecLayout, Ctx))> for VecDeku<V>
where
    Ctx: Copy,
    V: Clone + DekuReader<'a, (Endian, Ctx)>,
{
    #[inline]
    fn from_reader_with_ctx<R: no_std_io::Read + no_std_io::Seek>(
        reader: &mut Reader<R>,
        ctx: (Endian, (VecLayout, Ctx)),
    ) -> Result<Self, DekuError> {
        Self::from_reader_impl(reader, ctx.0, ctx.1.0, (ctx.0, ctx.1.1))
    }
}

impl<V> DekuWriter<(Endian, VecLayout)> for VecDeku<V>
where
    V: Clone + Default + DekuWriter<Endian>,
{
    #[inline]
    fn to_writer<W: no_std_io::Write + no_std_io::Seek>(
        &self,
        writer: &mut Writer<W>,
        ctx: (Endian, VecLayout),
    ) -> Result<(), DekuError> {
        self.to_writer_impl(writer, ctx.0, ctx.1, ctx.0)
    }
}

impl<V, Ctx> DekuWriter<(Endian, VecLayout, Ctx)> for VecDeku<V>
where
    Ctx: Copy,
    V: Clone + Default + DekuWriter<(Endian, Ctx)>,
{
    #[inline]
    fn to_writer<W: no_std_io::Write + no_std_io::Seek>(
        &self,
        writer: &mut Writer<W>,
        ctx: (Endian, VecLayout, Ctx),
    ) -> Result<(), DekuError> {
        self.to_writer_impl(writer, ctx.0, ctx.1, (ctx.0, ctx.2))
    }
}

impl<V, Ctx> DekuWriter<(Endian, (VecLayout, Ctx))> for VecDeku<V>
where
    Ctx: Copy,
    V: Clone + Default + DekuWriter<(Endian, Ctx)>,
{
    #[inline]
    fn to_writer<W: no_std_io::Write + no_std_io::Seek>(
        &self,
        writer: &mut Writer<W>,
        ctx: (Endian, (VecLayout, Ctx)),
    ) -> Result<(), DekuError> {
        self.to_writer_impl(writer, ctx.0, ctx.1.0, (ctx.0, ctx.1.1))
    }
}
