#![no_std]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//! Utility crate for [`Deku`] that provides convenient support for serializing and
//! deserializing String and Vec in a variety of binary formats.
//!
//! Some notable features:
//!
//! * StringDeku supports UTF-8, UTF-16 and UTF-32 encodings.
//! * Multiple layout formats, such as .Net, Pascal and zero-ended.
//! * Little and Big Endian support.
//! * Dynamic read and write without additional temporary structs and operations.
//! * No need to specify custom reader and writer functions.
//! * Compatible with Deku's derive macros and custom readers/writers.
//! * Supports `serde` and `defmt`.
//!
//! Read specific struct for example usage.
//!
//! [`Deku`]: https://docs.rs/deku

extern crate alloc;
#[cfg(test)]
extern crate std;

mod common;
mod seven_bit;
mod string;
mod vec;

pub(crate) use common::InternalValue;
pub(crate) use common::deku_impl::{read_size_prefix, write_size_prefix};
pub(crate) use common::serde_impl::serde_shim_implementation;
pub(crate) use common::std_impl::std_shim_implementation;

pub use seven_bit::{SevenBitU8, SevenBitU16, SevenBitU32, SevenBitU64, SevenBitU128};
pub use string::{Encoding, StringDeku, StringLayout};
pub use vec::{VecDeku, VecLayout};

/// Size encoding for length-prefixed [`StringDeku`] and [`VecDeku`]
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum Size {
    /// 1 byte to encode length.
    U8,

    /// 2 bytes to encode length.
    U16,

    /// 4 bytes to encode length.
    U32,

    /// 7bit encoded u32 to encode length (.Net style).
    ///
    /// See [`crate::SevenBitU32`] for details.
    U32_7Bit,
}
