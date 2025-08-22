use alloc::{borrow::Cow, format, string::String, vec, vec::Vec};

use deku::ctx::{Endian, Limit};
use deku::reader::Reader;
use deku::writer::Writer;
use deku::{DekuError, DekuReader, DekuWriter, no_std_io};

use crate::{Encoding, InternalValue, SevenBitU32, Size, StringDeku, StringLayout};

impl StringDeku {
    pub(self) fn from_reader_with_ctx_impl<R>(
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
            return Ok(StringDeku::from(""));
        }

        match encoding {
            Encoding::Utf8 => {
                read_string(reader, &null_requirement, limit_u8, endian, |buf| {
                    String::from_utf8(buf.to_vec())
                        .map_err(|_| DekuError::Parse("Invalid UTF-8".into()))
                })
            }
            Encoding::Utf16 => {
                read_string(reader, &null_requirement, limit_u16, endian, |buf| {
                    String::from_utf16(buf)
                        .map_err(|_| DekuError::Parse("Invalid UTF-16".into()))
                })
            }
            Encoding::Utf32 => {
                read_string(reader, &null_requirement, limit_u32, endian, |buf| {
                    let mut result: Vec<char> = vec![];
                    buf.iter().try_fold((), |_, value| {
                        let Some(ch) = char::from_u32(*value) else {
                            return Err(DekuError::Parse("Invalid UTF-32".into()));
                        };
                        result.push(ch);
                        Ok(())
                    })?;
                    Ok(result.into_iter().collect())
                })
            }
        }
    }

    pub(self) fn to_writer_impl<W: no_std_io::Write + no_std_io::Seek>(
        &self,
        writer: &mut Writer<W>,
        endian: Endian,
        encoding: Encoding,
        layout: StringLayout,
    ) -> Result<(), DekuError> {
        match encoding {
            Encoding::Utf8 => {
                let mut buf = self.internal_ref().as_bytes().to_vec();
                write_string(writer, endian, layout, &mut buf)
            }
            Encoding::Utf16 => {
                let mut buf = self.internal_ref().encode_utf16().collect::<Vec<u16>>();
                write_string(writer, endian, layout, &mut buf)
            }
            Encoding::Utf32 => {
                let mut buf = self
                    .internal_ref()
                    .chars()
                    .map(|ch| ch.into())
                    .collect::<Vec<u32>>();
                write_string(writer, endian, layout, &mut buf)
            }
        }
    }
}

impl DekuReader<'_, (Endian, Encoding, StringLayout)> for StringDeku {
    /// Read string from reader
    fn from_reader_with_ctx<R: no_std_io::Read + no_std_io::Seek>(
        reader: &mut Reader<R>,
        ctx: (Endian, Encoding, StringLayout),
    ) -> Result<Self, DekuError>
    where
        Self: Sized,
    {
        let (endian, encoding, layout) = ctx;
        Self::from_reader_with_ctx_impl(reader, endian, encoding, layout)
    }
}

impl DekuReader<'_, (Endian, (Encoding, StringLayout))> for StringDeku {
    /// Read string from reader
    fn from_reader_with_ctx<R: no_std_io::Read + no_std_io::Seek>(
        reader: &mut Reader<R>,
        ctx: (Endian, (Encoding, StringLayout)),
    ) -> Result<Self, DekuError>
    where
        Self: Sized,
    {
        let (endian, (encoding, layout)) = ctx;
        Self::from_reader_with_ctx_impl(reader, endian, encoding, layout)
    }
}

impl DekuWriter<(Endian, Encoding, StringLayout)> for StringDeku {
    /// Write string to the writer.
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

/// Read limit and null placement requirements from layout and reader (if prefixed)
///
/// Zero-ended gives another limit kind that based on size, thus result can't be just a size.
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
            Ok((
                null_requirement,
                Limit::from(size),
                Limit::from(size),
                Limit::from(size),
            ))
        }
        StringLayout::ZeroEnded => Ok((
            // zero is already at the end by how deku reads data
            NullRequirement::Accepted,
            Limit::new_until(|v: &u8| *v == 0),
            Limit::new_until(|v: &u16| *v == 0),
            Limit::new_until(|v: &u32| *v == 0),
        )),
        StringLayout::LengthPrefix(prefix) => {
            let size: usize = match prefix {
                Size::U8 => <u8>::from_reader_with_ctx(reader, endian)? as usize,
                Size::U16 => <u16>::from_reader_with_ctx(reader, endian)? as usize,
                Size::U32 => <u32>::from_reader_with_ctx(reader, endian)? as usize,
                Size::U32_7Bit => {
                    let length: u32 =
                        <SevenBitU32>::from_reader_with_ctx(reader, ())?.into();
                    length as usize
                }
            };
            Ok((
                NullRequirement::Rejected,
                Limit::from(size),
                Limit::from(size),
                Limit::from(size),
            ))
        }
    }
}

/// Common implementation to read String from stream.
///
/// Read data from reader, check null character presence
/// and placement and converts to a string.
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

    match null_requirement {
        NullRequirement::Accepted => {}
        NullRequirement::Required => {
            if first_null == buf.len() {
                return Err(DekuError::Assertion(Cow::from(
                    "Null must be present in the buffer",
                )));
            }
        }
        NullRequirement::Rejected => {
            if first_null != buf.len() {
                return Err(DekuError::Assertion(Cow::from(
                    "Null must be present in the buffer",
                )));
            }
        }
    }

    convert(&buf[..first_null]).map(Into::into)
}

/// Common implementation to write Vec<u8> and Vec<u16>
fn write_string<W, T>(
    writer: &mut Writer<W>,
    endian: Endian,
    layout: StringLayout,
    buf: &mut Vec<T>,
) -> Result<(), DekuError>
where
    W: no_std_io::Write + no_std_io::Seek,
    T: Default + Clone + PartialEq + DekuWriter<Endian>,
{
    let zero = T::default();
    // don't write shady strings with null character in the middle

    let first_null: usize = buf.iter().position(|x| *x == zero).unwrap_or(buf.len());
    if first_null != buf.len() {
        return Err(DekuError::Assertion(Cow::from(
            "Null MUST NOT be present in the binary representation",
        )));
    }

    match layout {
        StringLayout::LengthPrefix(prefix_size) => {
            write_string_length_prefix(writer, endian, buf, prefix_size)
        }
        StringLayout::ZeroEnded => {
            buf.to_writer(writer, endian)?;
            zero.to_writer(writer, endian)
        }

        StringLayout::FixedLength {
            size,
            allow_no_null,
        } => write_string_fixed_length(
            writer,
            endian,
            buf,
            size,
            allow_no_null,
            first_null,
            zero,
        ),
    }
}

fn write_string_length_prefix<W, T>(
    writer: &mut Writer<W>,
    endian: Endian,
    buf: &mut Vec<T>,
    prefix_size: Size,
) -> Result<(), DekuError>
where
    W: no_std_io::Write + no_std_io::Seek,
    T: Default + Clone + PartialEq + DekuWriter<Endian>,
{
    let max_size: usize = match prefix_size {
        Size::U8 => u8::MAX as usize,
        Size::U16 => u16::MAX as usize,
        Size::U32 => u32::MAX as usize,
        Size::U32_7Bit => u32::MAX as usize,
    };

    if buf.len() > max_size {
        return Err(DekuError::Assertion(Cow::from(format!(
            "Encoded string length cannot exceed {max_size} bytes"
        ))));
    }

    // buffer len is not above corresponding type size,
    // so truncation is safe
    #[allow(clippy::cast_possible_truncation)]
    match prefix_size {
        Size::U8 => ((buf.len() & 0xFF) as u8).to_writer(writer, endian),
        Size::U16 => (buf.len() as u16).to_writer(writer, endian),
        Size::U32 => (buf.len() as u32).to_writer(writer, endian),
        Size::U32_7Bit => {
            let length: SevenBitU32 = (buf.len() as u32).into();
            (length).to_writer(writer, ())
        }
    }?;

    buf.to_writer(writer, endian)
}

fn write_string_fixed_length<W, T>(
    writer: &mut Writer<W>,
    endian: Endian,
    buf: &mut Vec<T>,
    size: usize,
    allow_no_null: bool,
    first_null: usize,
    zero: T,
) -> Result<(), DekuError>
where
    W: no_std_io::Write + no_std_io::Seek,
    T: Default + Clone + PartialEq + DekuWriter<Endian>,
{
    if buf.len() > size {
        return Err(DekuError::Assertion(Cow::from(format!(
            "Encoded string length cannot exceed {size} elements"
        ))));
    }

    if !allow_no_null && first_null == size {
        return Err(DekuError::Assertion(Cow::from(
            "String fills whole output buffer, while Null character must be written",
        )));
    }

    buf.to_writer(writer, endian)?;

    for _ in buf.len()..size {
        zero.to_writer(writer, endian)?;
    }

    Ok(())
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
