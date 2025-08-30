//! Wrapper around `Vec<T>` to read and write with various layouts.
mod deku_impl;
mod lib_impl;
#[cfg(feature = "serde")]
mod serde_impl;
mod std_impl;

use alloc::vec::Vec;

use crate::Size;

/// Thin wrapper around [`alloc::vec::Vec<T>`] to read and write in various layouts.
///
/// It designed to make it easy to create complicated fields like this:
///   `encrypted_compressed_items: Encrypted<Compressed<FixedLengthVec<PascalString>>>`
///
/// Example usage of [`VecDeku`]:
///
/// ```rust
/// use deku_string::{Encoding, Size, StringDeku, StringLayout, VecDeku, VecLayout};
///
/// #[derive(Debug, Clone, PartialEq, deku::DekuRead, deku::DekuWrite)]
/// #[deku(endian = "little")]
/// struct SampleModel {
///     #[deku(ctx = "VecLayout::FixedLength(10)")]
///     fixed_length_vec: VecDeku<u8>,
///
///     #[deku(ctx = "VecLayout::LengthPrefix(Size::U32), (Encoding::Utf8, \
///                   StringLayout::LengthPrefix(Size::U8))")]
///     vec_of_strings: VecDeku<StringDeku>,
/// }
/// ```
#[derive(Clone, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct VecDeku<T>(Vec<T>)
where
    T: Sized + Clone;

/// Supported vec binary layouts.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum VecLayout {
    /// Fixed length data.
    ///
    /// If a user writes less items, [`Default::default()`] value will be used.
    ///
    /// While it's possible to embed custom value here, it would have serious
    /// limitations.
    FixedLength(
        /// How many (exact) items should be in vector.
        usize,
    ),

    /// String is prefixed by length value
    LengthPrefix(Size),

    /// Read until the reader end
    End,
}
