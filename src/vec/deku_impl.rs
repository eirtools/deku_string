//! Deku-related implementation for [`crate::VecDeku`].
use alloc::vec::Vec;

use deku::ctx::{Endian, Limit};
use deku::reader::Reader;
use deku::writer::Writer;
use deku::{DekuError, DekuReader, DekuWriter, no_std_io};

use crate::common::deku_impl::write_data_fixed_length;
use crate::{
    InternalValue as _, VecDeku, VecLayout, read_size_prefix, write_size_prefix,
};

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
                Limit::from(read_size_prefix(reader, endian, prefix)?)
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
                write_size_prefix(writer, endian, prefix, self.internal_ref().len())?;
                self.internal_ref().to_writer(writer, inner_ctx)
            }

            VecLayout::FixedLength(size) => {
                write_data_fixed_length(writer, self, size, inner_ctx, &T::default())
            }
        }
    }
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
