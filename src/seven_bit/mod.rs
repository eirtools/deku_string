//! Read/write 7-bit int as described here:
//!
//! * Read: https://learn.microsoft.com/en-us/dotnet/api/system.io.binaryreader.read7bitencodedint?view=net-9.0
//! * Write: https://learn.microsoft.com/en-us/dotnet/api/system.io.binarywriter.write7bitencodedint?view=net-9.0

mod deku_impl;
mod lib_impl;

/// 7-bit u32
#[derive(Clone, Default, PartialEq, PartialOrd, Eq, Ord)]
pub struct SevenBitU32(pub(crate) u32);

/// 7-bit u64
#[derive(Clone, Default, PartialEq, PartialOrd, Eq, Ord)]
pub struct SevenBitU64(pub(crate) u64);

/// 7-bit u128
#[derive(Clone, Default, PartialEq, PartialOrd, Eq, Ord)]
pub struct SevenBitU128(pub(crate) u128);
