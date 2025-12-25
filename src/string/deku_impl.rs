//! Deku-related implementation for [`crate::StringDeku`].
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

use deku::ctx::{Endian, Limit};
use deku::reader::Reader;
use deku::writer::Writer;
use deku::{DekuError, DekuReader, DekuWriter, no_std_io};

use crate::common::deku_impl::write_data_fixed_length;
use crate::{
    Encoding, InternalValue as _, StringDeku, StringLayout, read_size_prefix,
    write_size_prefix,
};

impl StringDeku {
    /// Read from reader with context
    #[inline]
    fn from_reader_impl<R>(
        reader: &mut Reader<R>,
        endian: Endian,
        encoding: Encoding,
        layout: StringLayout,
    ) -> Result<Self, DekuError>
    where
        R: no_std_io::Read + no_std_io::Seek,
    {
        let (null_requirement, limit_u8, limit_u16, limit_u32): ReadRequirements =
            read_requirements(reader, endian, layout)?;

        // other limits are the same, but have different types.
        // This won't ever match for zero-ended strings
        if limit_u8 == Limit::Count(0) {
            // if requested length is 0, skip the data
            return Ok(Self::from(""));
        }

        match encoding {
            Encoding::Utf8 => {
                read_string(reader, &null_requirement, limit_u8, endian, convert_utf_8)
            }
            Encoding::Utf16 => read_string(
                reader,
                &null_requirement,
                limit_u16,
                endian,
                convert_utf_16,
            ),
            Encoding::Utf32 => read_string(
                reader,
                &null_requirement,
                limit_u32,
                endian,
                convert_utf_32,
            ),
        }
    }

    /// Write to a reader with context
    #[inline]
    fn to_writer_impl<W: no_std_io::Write + no_std_io::Seek>(
        &self,
        writer: &mut Writer<W>,
        endian: Endian,
        encoding: Encoding,
        layout: StringLayout,
    ) -> Result<(), DekuError> {
        match encoding {
            Encoding::Utf8 => {
                let buf = self.internal_ref().as_bytes().to_vec();
                write_string(writer, endian, layout, &buf)
            }
            Encoding::Utf16 => {
                let buf = self.internal_ref().encode_utf16().collect::<Vec<u16>>();
                write_string(writer, endian, layout, &buf)
            }
            Encoding::Utf32 => {
                let buf = self
                    .internal_ref()
                    .chars()
                    .map(Into::into)
                    .collect::<Vec<u32>>();
                write_string(writer, endian, layout, &buf)
            }
        }
    }
}

/// Convert UTF-8 data (native endian) to string
#[inline]
fn convert_utf_8(buf: &[u8]) -> Result<String, DekuError> {
    #[allow(clippy::map_err_ignore, reason = "Deku doesn't support custom errors")]
    String::from_utf8(buf.to_vec())
        .map_err(|_| DekuError::Parse("Invalid UTF-8".into()))
}

/// Convert UTF-16 data (native endian) to string
fn convert_utf_16(buf: &[u16]) -> Result<String, DekuError> {
    #[allow(clippy::map_err_ignore, reason = "Deku doesn't support custom errors")]
    String::from_utf16(buf).map_err(|_| DekuError::Parse("Invalid UTF-16".into()))
}

/// Convert UTF-32 data (native endian) to string
#[inline]
fn convert_utf_32(buf: &[u32]) -> Result<String, DekuError> {
    let mut result: Vec<char> = vec![];
    buf.iter().try_fold((), |(), value| {
        let Some(ch) = char::from_u32(*value) else {
            return Err(DekuError::Parse("Invalid UTF-32".into()));
        };
        result.push(ch);
        Ok(())
    })?;
    Ok(result.into_iter().collect())
}

impl DekuReader<'_, (Endian, Encoding, StringLayout)> for StringDeku {
    /// Read string from reader
    #[inline]
    fn from_reader_with_ctx<R: no_std_io::Read + no_std_io::Seek>(
        reader: &mut Reader<R>,
        ctx: (Endian, Encoding, StringLayout),
    ) -> Result<Self, DekuError>
    where
        Self: Sized,
    {
        let (endian, encoding, layout) = ctx;
        Self::from_reader_impl(reader, endian, encoding, layout)
    }
}

impl DekuReader<'_, (Endian, (Encoding, StringLayout))> for StringDeku {
    /// Read string from reader
    #[inline]
    fn from_reader_with_ctx<R: no_std_io::Read + no_std_io::Seek>(
        reader: &mut Reader<R>,
        ctx: (Endian, (Encoding, StringLayout)),
    ) -> Result<Self, DekuError>
    where
        Self: Sized,
    {
        let (endian, (encoding, layout)) = ctx;
        Self::from_reader_impl(reader, endian, encoding, layout)
    }
}

impl DekuWriter<(Endian, Encoding, StringLayout)> for StringDeku {
    /// Write string to the writer.
    #[inline]
    fn to_writer<W: no_std_io::Write + no_std_io::Seek>(
        &self,
        writer: &mut Writer<W>,
        ctx: (Endian, Encoding, StringLayout),
    ) -> Result<(), DekuError> {
        let (endian, encoding, layout) = ctx;
        self.to_writer_impl(writer, endian, encoding, layout)
    }
}

impl DekuWriter<(Endian, (Encoding, StringLayout))> for StringDeku {
    /// Write string to the writer.
    #[inline]
    fn to_writer<W: no_std_io::Write + no_std_io::Seek>(
        &self,
        writer: &mut Writer<W>,
        ctx: (Endian, (Encoding, StringLayout)),
    ) -> Result<(), DekuError> {
        let (endian, (encoding, layout)) = ctx;
        self.to_writer_impl(writer, endian, encoding, layout)
    }
}

/// Read Requirements tuple.
///
/// Type is defined for convenience.
/// There's no option to define it just with size only.
type ReadRequirements = (
    NullRequirement,
    Limit<u8, fn(&u8) -> bool>,
    Limit<u16, fn(&u16) -> bool>,
    Limit<u32, fn(&u32) -> bool>,
);

/// Read limit and null placement requirements from layout and reader (if
/// prefixed)
///
/// Zero-ended gives another limit kind that based on size, thus result can't be
/// just a size.
#[inline]
fn read_requirements<R: no_std_io::Read + no_std_io::Seek>(
    reader: &mut Reader<R>,
    endian: Endian,
    layout: StringLayout,
) -> Result<ReadRequirements, DekuError> {
    match layout {
        StringLayout::FixedLength {
            size,
            allow_no_null,
        } => {
            let null_requirement = if allow_no_null {
                NullRequirement::Accepted
            } else {
                NullRequirement::Required
            };

            Ok(size_requirement(null_requirement, size))
        }

        StringLayout::ZeroEnded => Ok((
            // zero is already at the end by how deku reads data
            NullRequirement::Accepted,
            Limit::new_until(|int: &u8| *int == 0),
            Limit::new_until(|int: &u16| *int == 0),
            Limit::new_until(|int: &u32| *int == 0),
        )),
        StringLayout::LengthPrefix(prefix) => Ok(size_requirement(
            NullRequirement::Rejected,
            read_size_prefix(reader, endian, prefix)?,
        )),
    }
}

/// Create read requirements based on size.
#[inline]
#[must_use]
fn size_requirement(
    null_requirement: NullRequirement,
    size: usize,
) -> ReadRequirements {
    (
        null_requirement,
        Limit::from(size),
        Limit::from(size),
        Limit::from(size),
    )
}

/// Common implementation to read String from stream.
///
/// Read data from reader, check null character presence
/// and placement and converts to a string.
#[inline]
fn read_string<'a, R, T>(
    reader: &mut Reader<R>,
    null_requirement: &NullRequirement,
    limit: Limit<T, fn(&T) -> bool>,
    endian: Endian,
    convert: fn(&[T]) -> Result<String, DekuError>,
) -> Result<StringDeku, DekuError>
where
    R: no_std_io::Read + no_std_io::Seek,
    T: Default + Clone + PartialEq + DekuReader<'a, Endian>,
{
    let zero = T::default();
    let buf = <Vec<T>>::from_reader_with_ctx(reader, (limit, endian))?;

    let first_null = buf.iter().position(|x| *x == zero).unwrap_or(buf.len());

    match *null_requirement {
        NullRequirement::Accepted => {}
        NullRequirement::Required => {
            if first_null == buf.len() {
                return Err(DekuError::Assertion(
                    "Null must be present in the buffer".into(),
                ));
            }
        }
        NullRequirement::Rejected => {
            if first_null != buf.len() {
                return Err(DekuError::Assertion(
                    "Null must be present in the buffer".into(),
                ));
            }
        }
    }
    #[allow(clippy::indexing_slicing, reason = "Checked indexing")]
    convert(&buf[..first_null]).map(Into::into)
}

/// Common implementation to write Vec<T> where T is u8, u16 or u32
#[inline]
fn write_string<W, T>(
    writer: &mut Writer<W>,
    endian: Endian,
    layout: StringLayout,
    buf: &Vec<T>,
) -> Result<(), DekuError>
where
    W: no_std_io::Write + no_std_io::Seek,
    T: Default + Clone + PartialEq + DekuWriter<Endian>,
{
    let zero = T::default();
    // don't write shady strings with null character in the middle

    let first_null: usize = buf.iter().position(|x| *x == zero).unwrap_or(buf.len());
    if first_null != buf.len() {
        return Err(DekuError::Assertion(
            "Null MUST NOT be present in the binary representation".into(),
        ));
    }

    match layout {
        StringLayout::LengthPrefix(prefix_size) => {
            write_size_prefix(writer, endian, prefix_size, buf.len())?;
            buf.to_writer(writer, endian)
        }
        StringLayout::ZeroEnded => {
            buf.to_writer(writer, endian)?;
            zero.to_writer(writer, endian)
        }

        StringLayout::FixedLength {
            size,
            allow_no_null,
        } => {
            if !allow_no_null && first_null == size {
                return Err(DekuError::Assertion(
                    "String fills whole output buffer, while Null character must be \
                     written"
                        .into(),
                ));
            }

            write_data_fixed_length(writer, buf, size, endian, &zero)
        }
    }
}

/// Requirement for null character presence
#[derive(Debug, PartialEq, PartialOrd)]
enum NullRequirement {
    /// Null character is required to be somewhere in a buffer
    Required,

    /// Null character is accepted to be or not to be in a buffer
    Accepted,

    /// Null character is no accepted to be in a buffer
    Rejected,
}
