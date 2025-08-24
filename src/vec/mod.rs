mod deku_impl;
mod lib_impl;
#[cfg(feature = "serde")]
mod serde_impl;
mod std_impl;

use alloc::vec::Vec;

use crate::Size;

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
#[derive(Clone, Default, PartialEq, PartialOrd)]
pub struct VecDeku<T>(Vec<T>)
where
    T: Sized + Clone + PartialEq + PartialOrd;

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
