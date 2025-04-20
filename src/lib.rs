#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//!
//! Simple wrapper around String and Vec<T> implementing `DekuReader` and `DekuWriter`.
//!
//! This implementation guarantees:
//!  * Predictable reads and writes
//!  * For fixed length layouts it will be always read and written with that size (if data is valid)
//!  * No forgotten update()
//!  * Suitable for nested structures like this: `Encrypted<Compressed<FixedLengthVec<PascalString>>>`
//!
//! ```rust,ignore
//! use ::deku_string::{Encoding, Size, StringDeku, StringLayout, VecLayout};
//!
//! #[derive(Debug, Clone, PartialEq, ::deku::DekuRead, ::deku::DekuWrite)]
//! #[deku(endian = "little")]
//! struct SampleModel {
//!     #[deku(ctx = "Encoding::Utf8, StringLayout::fixed_length(10)")]
//!     fixed_string: StringDeku,
//!
//!     #[deku(ctx = "VecLayout::LengthPrefix(Size::U32), (Encoding::Utf8, StringLayout::ZeroEnded)")]
//!     prefixed_u32_vec: VecDeku<StringDeku>,
//! }
//! ```

mod deku_impl;
mod deku_impl_vec;
mod std_impl;

///
/// Thin wrapper around String to read and write various layouts.
/// This looks as a simple wrapper over String, `StringDeku` structure is built
/// to be able of reading and writing of various common binary layouts.
///
/// * Fixed Length Layout represents a fixed amount of elements (bytes or words depending on encoding)
///   to read and write
/// * Length prefixed layout represent Pascal-like strings
/// * Zero ended — C-like strings.
///
/// How it's different from using `deku` directly?
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

/// Thin wrapper around Vec<T> to read and write various layouts.
/// This looks as a simple wrapper over Vec<T>, `VecDeku` structure is built
/// to be able of reading and writing of various common binary layouts.
///
/// * Fixed Length Layout represents a fixed amount of elements (bytes or words depending on encoding)
///   to read and write.
/// * Length prefixed layout represent count and then
///
/// How it's different from using `deku` directly?
///
/// * It's convenient way to create and maintain models:
/// * no readers and writers,
/// * no running `update` function, which may be forgotten.
/// * Fixed length is actually fixed, so if developer asked size 10, it will be read and written exactly 10.
/// * Easy to write complicated fields like this: `Encrypted<Compressed<FixedLengthVec<PascalString>>>`
#[derive(Debug, Clone, Default, PartialEq, PartialOrd)]
pub struct VecDeku<T>(pub Vec<T>)
where
    T: Sized + Clone + PartialEq + PartialOrd;

///
///  Supported string binary layouts
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

    /// String is zero-ended. 1 byte for Ascii and UTF-8, 2 bytes for UTF-16
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

///
///  Supported string binary layouts
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum VecLayout {
    /// Fixed length string
    FixedLength(
        /// How many (exact) items should be in vector.
        usize,
    ),

    /// String is prefixed by length value
    LengthPrefix(Size),

    /// Read until the reader end
    End,
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
