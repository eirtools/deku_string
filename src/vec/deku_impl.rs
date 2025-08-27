//! Deku implementation for `VecDeku`
use alloc::borrow::Cow;
use alloc::format;
use alloc::vec::Vec;

use deku::ctx::{Endian, Limit};
use deku::reader::Reader;
use deku::writer::Writer;
use deku::{DekuError, DekuReader, DekuWriter, no_std_io};

use crate::{SevenBitU32, Size, VecDeku, VecLayout};

impl<T: Clone> VecDeku<T> {
    /// Read data from reader
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
                let size = match prefix {
                    Size::U8 => <u8>::from_reader_with_ctx(reader, endian)? as usize,
                    Size::U16 => <u16>::from_reader_with_ctx(reader, endian)? as usize,
                    Size::U32 => <u32>::from_reader_with_ctx(reader, endian)? as usize,
                    Size::U32_7Bit => {
                        let size: u32 =
                            <SevenBitU32>::from_reader_with_ctx(reader, ())?.into();
                        size as usize
                    }
                };
                Limit::from(size)
            }
            VecLayout::End => Limit::End,
        };
        <Vec<T>>::from_reader_with_ctx(reader, (limit, inner_ctx)).map(Into::into)
    }

    /// Read data to writer
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
            VecLayout::End => self.0.to_writer(writer, inner_ctx),
            VecLayout::LengthPrefix(prefix) => {
                let max_size: usize = match prefix {
                    Size::U8 => u8::MAX as usize,
                    Size::U16 => u16::MAX as usize,
                    Size::U32 | Size::U32_7Bit => u32::MAX as usize,
                };

                if self.0.len() > max_size {
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
                        (self.0.len() as u8).to_writer(writer, endian)
                    }
                    Size::U16 =>
                    {
                        #[allow(clippy::cast_possible_truncation)]
                        (self.0.len() as u16).to_writer(writer, endian)
                    }
                    Size::U32 =>
                    {
                        #[allow(clippy::cast_possible_truncation)]
                        (self.0.len() as u32).to_writer(writer, endian)
                    }
                    Size::U32_7Bit => {
                        #[allow(clippy::cast_possible_truncation)]
                        let size: SevenBitU32 = (self.0.len() as u32).into();
                        size.to_writer(writer, ())
                    }
                }?;

                self.0.to_writer(writer, inner_ctx)
            }

            VecLayout::FixedLength(size) => {
                let len = self.0.len();
                if len > size {
                    return Err(DekuError::Assertion(Cow::from(format!(
                        "Encoded string length cannot exceed {size} bytes"
                    ))));
                }
                self.0.to_writer(writer, inner_ctx)?;
                let default = T::default();
                for _ in len..size {
                    default.to_writer(writer, inner_ctx)?;
                }
                Ok(())
            }
        }
    }
}

impl<'a, V> DekuReader<'a, (Endian, VecLayout)> for VecDeku<V>
where
    V: Clone + DekuReader<'a, Endian>,
{
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
    fn to_writer<W: no_std_io::Write + no_std_io::Seek>(
        &self,
        writer: &mut Writer<W>,
        ctx: (Endian, (VecLayout, Ctx)),
    ) -> Result<(), DekuError> {
        self.to_writer_impl(writer, ctx.0, ctx.1.0, (ctx.0, ctx.1.1))
    }
}
