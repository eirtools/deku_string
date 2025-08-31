//! Common methods to implement Deku interfaces for buffers in this lib.

use alloc::borrow::Cow;
use alloc::format;

use deku::ctx::Endian;
use deku::reader::Reader;
use deku::writer::Writer;
use deku::{DekuError, DekuReader as _, DekuWriter, no_std_io};

use crate::{SevenBitU32, Size};

/// Read expected size from stream
#[allow(clippy::as_conversions, reason = "Assume that usize > u32")]
#[inline]
pub(crate) fn read_size_prefix<R>(
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
#[allow(clippy::as_conversions, reason = "Assume that usize > u32")]
#[inline]
pub(crate) fn write_size_prefix<W>(
    writer: &mut Writer<W>,
    endian: Endian,
    prefix: Size,
    length: usize,
) -> Result<(), DekuError>
where
    W: no_std_io::Write + no_std_io::Seek,
{
    let max_size: usize = match prefix {
        Size::U8 => u8::MAX as usize,
        Size::U16 => u16::MAX as usize,
        Size::U32 | Size::U32_7Bit => u32::MAX as usize,
    };

    if length > max_size {
        return Err(DekuError::Assertion(Cow::from(format!(
            "Encoded data length cannot exceed {max_size} bytes"
        ))));
    }

    #[allow(clippy::cast_possible_truncation, reason = "Checked above")]
    match prefix {
        Size::U8 => (length as u8).to_writer(writer, endian),
        Size::U16 => (length as u16).to_writer(writer, endian),
        Size::U32 => (length as u32).to_writer(writer, endian),
        Size::U32_7Bit => {
            let size = SevenBitU32::new(length as u32);
            size.to_writer(writer, ())
        }
    }
}

/// Write fixed-length buffer into stream
#[inline]
pub(crate) fn write_data_fixed_length<T, W, Ctx>(
    writer: &mut Writer<W>,
    data: &[T],
    size: usize,
    inner_ctx: Ctx,
    default: &T,
) -> Result<(), DekuError>
where
    Ctx: Copy,
    T: Clone + Default + DekuWriter<Ctx>,
    W: no_std_io::Write + no_std_io::Seek,
{
    let len = data.len();
    if len > size {
        return Err(DekuError::Assertion(Cow::from(format!(
            "Encoded data length cannot exceed {size} elements"
        ))));
    }

    data.to_writer(writer, inner_ctx)?;

    for _ in len..size {
        default.to_writer(writer, inner_ctx)?;
    }

    Ok(())
}
