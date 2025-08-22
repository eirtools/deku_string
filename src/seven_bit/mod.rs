//! Read/write 7-bit int as described here:
//!
//! * Read: https://learn.microsoft.com/en-us/dotnet/api/system.io.binaryreader.read7bitencodedint?view=net-9.0
//! * Write: https://learn.microsoft.com/en-us/dotnet/api/system.io.binarywriter.write7bitencodedint?view=net-9.0

mod deku_impl;
mod lib_impl;

/// Variable length 7-bit encoded u8
#[derive(Clone, Default, PartialEq, PartialOrd, Eq, Ord)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SevenBitU8(pub(crate) u8);

/// Variable length 7-bit encoded u16
#[derive(Clone, Default, PartialEq, PartialOrd, Eq, Ord)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SevenBitU16(pub(crate) u16);

/// Variable length 7-bit encoded u32
#[derive(Clone, Default, PartialEq, PartialOrd, Eq, Ord)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SevenBitU32(pub(crate) u32);

/// Variable length 7-bit encoded u64
#[derive(Clone, Default, PartialEq, PartialOrd, Eq, Ord)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SevenBitU64(pub(crate) u64);

/// Variable length 7-bit encoded u128
#[derive(Clone, Default, PartialEq, PartialOrd, Eq, Ord)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SevenBitU128(pub(crate) u128);
