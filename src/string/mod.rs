//! Wrapper around String to read and write with various layouts.
use crate::Size;
use alloc::string::String;

#[cfg(feature = "defmt")]
mod defmt_impl;
mod deku_impl;
mod lib_impl;
mod std_impl;

/// Thin wrapper around [`alloc::string::String`] to read and write in various layouts.
///
/// It's designed to define supported layouts easy to read and easy to write without
/// additional functions.
///
/// Example usage of [`StringDeku`]:
///
/// ```rust
/// use deku_string::{Encoding, Size, StringDeku, StringLayout};
///
/// #[derive(Debug, Clone, PartialEq, deku::DekuRead, deku::DekuWrite)]
/// #[deku(endian = "little")]
/// struct SampleModel {
///     #[deku(ctx = "Encoding::Utf8, StringLayout::fixed_length(10)")]
///     fixed_length_string: StringDeku,
///
///     #[deku(ctx = "Encoding::Utf8, StringLayout::ZeroEnded")]
///     ascii_like: StringDeku,
/// }
/// ```
#[derive(Clone, Default, PartialEq, PartialOrd, Eq, Ord)]
pub struct StringDeku(String);

/// Supported string binary layouts.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum StringLayout {
    /// Fixed length string.
    ///
    /// Output will be padded by `null` characters.
    FixedLength {
        /// How many (exact) items should be there.
        size: usize,

        /// If `null` character absence is allowed in the buffer.
        allow_no_null: bool,
    },

    /// String is prefixed by length value (Pascal-like).
    LengthPrefix(Size),

    /// String is `null`-ended (C-like).
    ZeroEnded,
}

impl StringLayout {
    /// Construct fixed length layout with given size and no `null` allowed.
    #[inline]
    #[must_use]
    pub const fn fixed_length(size: usize) -> Self {
        Self::FixedLength {
            size,
            allow_no_null: false,
        }
    }
}

/// String encoding to use.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum Encoding {
    /// UTF-8 encoded string.
    Utf8,

    /// UTF-16 character sequences.
    Utf16,

    /// UTF-32 character sequences.
    Utf32,
}
