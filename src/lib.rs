#![deny(missing_docs)]
#![deny(missing_debug_implementations)]

//!
//! Simple wrapper around String to implement `DekuReader` and `DekuWriter`
//!
//! This implementation requires following context:
//!  * Endian — little or big
//!  * Encoding — UTF-8, UTF-16 or UTF-32
//!  * `StringLayout` — how string is represented in binary
//!
//! Pick your favorite combination
//!
//! Example usage od `StringDeku` with UTF-8 strings:
//!
//! ```rust,ignore
//! use ::deku_string::{StringDeku, Encoding, StringLayout, Size};
//!
//! #[derive(Debug, Clone, PartialEq, ::deku::DekuRead, ::deku::DekuWrite)]
//! #[deku(endian = "little")]
//! struct SampleModel {
//!     // fixed length buffer, null  character is required to be inside
//!     // "012345678\x00" is allowed
//!     // "0123456789" is NOT allowed
//!     #[deku(ctx = "Encoding::Utf8, StringLayout::fixed_length(10)")]
//!     fixed_value1: StringDeku,
//!
//!     // fixed length buffer, null byte is allowed to be inside,
//!     // both "012345678\x00" and "0123456789" are allowed
//!     #[deku(ctx = "Encoding::Utf8, StringLayout::FixedLength{size: 10, allow_no_null: true}")]
//!     fixed_value2: StringDeku,
//!
//!     // length (1 byte) then string, null character is NOT allowed inside
//!     // b"\0x501234"
//!     #[deku(ctx = "Encoding::Utf8, StringLayout::LengthPrefix(Size::U8)")]
//!     prefixed_u8: StringDeku,
//!
//!     // length (2 byte) then string, null character is NOT allowed inside
//!     // b"\0x5\x0001234"
//!     #[deku(ctx = "Encoding::Utf8, StringLayout::LengthPrefix(Size::U16)")]
//!     prefixed_u16: StringDeku,
//!
//!     // length (4 byte) then string, null character is NOT allowed inside
//!     // b"\0x5\x00\x00\x0001234"
//!     #[deku(ctx = "Encoding::Utf8, StringLayout::LengthPrefix(Size::U32)")]
//!     prefixed_u32: StringDeku,
//!
//!     // String with null byte at the end
//!     // b"012345\x00"
//!     #[deku(ctx = "Encoding::Utf8, StringLayout::ZeroEnded")]
//!     zero_ended: StringDeku,
//! }
//! ```

mod common;
mod seven_bit;
mod string;

pub(crate) use common::InternalValue;
pub(crate) use common::serde_impl::serde_shim_implementation;
pub(crate) use common::std_impl::std_shim_implementation;

pub use seven_bit::{SevenBitU8, SevenBitU16, SevenBitU32, SevenBitU64, SevenBitU128};
pub use string::{Encoding, StringDeku, StringLayout};

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

    /// u16, 7bit encoded
    U32_7Bit,
}
