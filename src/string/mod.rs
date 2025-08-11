use crate::Size;

mod deku_impl;
mod lib_impl;
mod std_impl;

///
/// Simple wrapper around String to read and write various layouts.
/// This looks as a simple wrapper over String, `StringDeku` structure is built
/// to be able of reading and writing of various common binary layouts.
///
/// For example,
///
/// * Fixed Length Layout represents a fixed amount of elements (bytes or words depending on encoding)
///   to read and write
/// * Length prefixed layout represent Pascal-like strings
/// * Zero ended — C-like strings.
///
/// How it's different from using Deku directly?
///
/// * It's convenient way to create and maintain models:
///    * no readers and writers,
///    * no running `update` function, which may be forgotten
/// * `StringDeku` implementation checks if there was no unexpected zero characters in the middle,
///   which could be missed during decoding, resulting in a probably dangerous values.
///
/// While content is hidden, `to_string`, `into` and equality functions and operators provide
/// convenient way to make operations easier.
#[derive(Clone, Default, PartialEq, PartialOrd, Eq, Ord)]
pub struct StringDeku(pub(crate) String);

/// String variant to read and write
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum StringLayout {
    /// Fixed length string
    FixedLength {
        /// How many (exact) items should be there.
        size: usize,

        /// If null character absence is allowed in the buffer.
        allow_no_null: bool,
    },

    /// String is prefixed by length value
    LengthPrefix(Size),

    /// String is zero-ended. 1 byte for ASCII and UTF-8, 2 bytes for UTF-16
    ZeroEnded,
}

impl StringLayout {
    /// Construct fixed length variant with given size and no null isn't allowed.
    #[must_use]
    pub fn fixed_length(size: usize) -> Self {
        Self::FixedLength {
            size,
            allow_no_null: false,
        }
    }
}

/// Encoding to use for read and write
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum Encoding {
    /// UTF-8 encoded string
    Utf8,

    /// UTF-16 character sequences
    Utf16,
}
