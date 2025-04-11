#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//!
//! Simple wrapper around String to implement DekuReader and DekuWriter
//!
//! This implementation requires following context:
//!  * Endian — little or big
//!  * Encoding — UTF-8 or UTF-16
//!  * Layout — how string is represented in binary
//!
//! Pick your favorite combination
//!
//! Example usage od StringDeku with UTF-8 strings:
//!
//! ```rust,ignore
//! use ::deku_string::{StringDeku, Encoding, Layout, Size};
//!
//! #[derive(Debug, Clone, PartialEq, ::deku::DekuRead, ::deku::DekuWrite)]
//! #[deku(endian = "little")]
//! struct SampleModel {
//!     // fixed length buffer, null  character is required to be inside
//!     // "012345678\x00" is allowed
//!     // "0123456789" is NOT allowed
//!     #[deku(ctx = "Encoding::Utf8, Layout::fixed_length(10)")]
//!     fixed_value1: StringDeku,
//!
//!     // fixed length buffer, null byte is allowed to be inside,
//!     // both "012345678\x00" and "0123456789" are allowed
//!     #[deku(ctx = "Encoding::Utf8, Layout::FixedLength{size: 10, allow_no_null: true}")]
//!     fixed_value2: StringDeku,
//!
//!     // length (1 byte) then string, null character is NOT allowed inside
//!     // b"\0x501234"
//!     #[deku(ctx = "Encoding::Utf8, Layout::LengthPrefix(Size::U8)")]
//!     prefixed_u8: StringDeku,
//!
//!     // length (2 byte) then string, null character is NOT allowed inside
//!     // b"\0x5\x0001234"
//!     #[deku(ctx = "Encoding::Utf8, Layout::LengthPrefix(Size::U16)")]
//!     prefixed_u16: StringDeku,
//!
//!     // length (4 byte) then string, null character is NOT allowed inside
//!     // b"\0x5\x00\x00\x0001234"
//!     #[deku(ctx = "Encoding::Utf8, Layout::LengthPrefix(Size::U32)")]
//!     prefixed_u32: StringDeku,
//!
//!     // String with null byte at the end
//!     // b"012345\x00"
//!     #[deku(ctx = "Encoding::Utf8, Layout::ZeroEnded")]
//!     zero_ended: StringDeku,
//! }
//! ```
//!

mod deku_impl;
mod std_impl;

/// Simple wrapper around String to implement DekuReader and DekuWriter
#[derive(Clone, Default, PartialEq, PartialOrd, Eq, Ord)]
pub struct StringDeku(pub(crate) String);

/// String variant to read and write
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum Layout {
    /// Fixed length string
    FixedLength {
        /// How many items should be there.
        size: usize,

        /// If null character absence is allowed in the buffer.
        allow_no_null: bool,
    },

    /// String is prefixed by length value
    LengthPrefix(Size),

    /// String is zero-ended. 1 byte for Ascii and UTF-8, 2 bytes for UTF-16
    ZeroEnded,
}

impl Layout {
    /// Construct fixed length variant with given size and no null isn't allowed.
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

/// Length prefix size
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum Size {
    /// One byte
    U8,

    /// 2 bytes
    U16,

    /// 4 bytes
    U32,
}
