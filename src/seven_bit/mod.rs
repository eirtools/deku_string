//! Read/write 7-bit int as described in MSDN.
#![cfg_attr(
    feature = "defmt",
    allow(clippy::missing_asserts_for_indexing, reason = "Defmt impl")
)]

mod deku_impl;
mod lib_impl;

/// Variable length 7-bit encoded u8.
///
/// MSDN links for .NET 9.0 documentation, implementation is based on:
/// * [`BinaryReader.read()`]
/// * [`BinaryWriter.write()`]
///
/// <div class="warning">Endianness isn't supported per reference documentation</div>
///
/// Example usage of [`SevenBitU8`]:
///
/// ```rust
/// use deku_string::{Encoding, SevenBitU8, Size, StringDeku, StringLayout};
///
/// #[derive(Debug, Clone, PartialEq, deku::DekuRead, deku::DekuWrite)]
/// struct SampleModel {
///     // 7-bit encoded u8
///     value: SevenBitU8,
/// }
/// ```
///
/// [`BinaryReader.read()`]: https://learn.microsoft.com/en-us/dotnet/api/system.io.binaryreader.read7bitencodedint?view=net-9.0
/// [`BinaryWriter.write()`]: https://learn.microsoft.com/en-us/dotnet/api/system.io.binarywriter.write7bitencodedint?view=net-9.0
#[derive(Clone, Default, PartialEq, PartialOrd, Eq, Ord)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SevenBitU8(u8);

/// Variable length 7-bit encoded u16.
///
/// MSDN links for .NET 9.0 documentation, implementation is based on:
/// * [`BinaryReader.read()`]
/// * [`BinaryWriter.write()`]
///
/// <div class="warning">Endianness isn't supported per reference documentation</div>
///
/// Example usage of [`SevenBitU16`]:
///
/// ```rust
/// use deku_string::{Encoding, SevenBitU16, Size, StringDeku, StringLayout};
///
/// #[derive(Debug, Clone, PartialEq, deku::DekuRead, deku::DekuWrite)]
/// struct SampleModel {
///     // 7-bit encoded u16
///     value: SevenBitU16,
/// }
/// ```
///
/// [`BinaryReader.read()`]: https://learn.microsoft.com/en-us/dotnet/api/system.io.binaryreader.read7bitencodedint?view=net-9.0
/// [`BinaryWriter.write()`]: https://learn.microsoft.com/en-us/dotnet/api/system.io.binarywriter.write7bitencodedint?view=net-9.0
#[derive(Clone, Default, PartialEq, PartialOrd, Eq, Ord)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SevenBitU16(u16);

/// Variable length 7-bit encoded u32.
///
/// MSDN links for .NET 9.0 documentation, implementation is based on:
/// * [`BinaryReader.read()`]
/// * [`BinaryWriter.write()`]
///
/// <div class="warning">Endianness isn't supported per reference documentation</div>
///
/// Example usage of [`SevenBitU32`]:
///
/// ```rust
/// use deku_string::{Encoding, SevenBitU32, Size, StringDeku, StringLayout};
///
/// #[derive(Debug, Clone, PartialEq, deku::DekuRead, deku::DekuWrite)]
/// struct SampleModel {
///     // 7-bit encoded u32
///     value: SevenBitU32,
/// }
/// ```
///
/// [`BinaryReader.read()`]: https://learn.microsoft.com/en-us/dotnet/api/system.io.binaryreader.read7bitencodedint?view=net-9.0
/// [`BinaryWriter.write()`]: https://learn.microsoft.com/en-us/dotnet/api/system.io.binarywriter.write7bitencodedint?view=net-9.0
#[derive(Clone, Default, PartialEq, PartialOrd, Eq, Ord)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SevenBitU32(u32);

/// Variable length 7-bit encoded u64.
///
/// MSDN links for .NET 9.0 documentation, implementation is based on:
/// * [`BinaryReader.read()`]
/// * [`BinaryWriter.write()`]
///
/// <div class="warning">Endianness isn't supported per reference documentation</div>
///
/// Example usage of [`SevenBitU64`]:
///
/// ```rust
/// use deku_string::{Encoding, SevenBitU64, Size, StringDeku, StringLayout};
///
/// #[derive(Debug, Clone, PartialEq, deku::DekuRead, deku::DekuWrite)]
/// struct SampleModel {
///     // 7-bit encoded u64
///     value: SevenBitU64,
/// }
/// ```
///
/// [`BinaryReader.read()`]: https://learn.microsoft.com/en-us/dotnet/api/system.io.binaryreader.read7bitencodedint?view=net-9.0
/// [`BinaryWriter.write()`]: https://learn.microsoft.com/en-us/dotnet/api/system.io.binarywriter.write7bitencodedint?view=net-9.0
#[derive(Clone, Default, PartialEq, PartialOrd, Eq, Ord)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SevenBitU64(u64);

/// Variable length 7-bit encoded u128.
///
/// MSDN links for .NET 9.0 documentation, implementation is based on:
/// * [`BinaryReader.read()`]
/// * [`BinaryWriter.write()`]
///
/// <div class="warning">Endianness isn't supported per reference documentation</div>
///
/// Example usage of [`SevenBitU128`]:
///
/// ```rust
/// use deku_string::{Encoding, SevenBitU128, Size, StringDeku, StringLayout};
///
/// #[derive(Debug, Clone, PartialEq, deku::DekuRead, deku::DekuWrite)]
/// struct SampleModel {
///     // 7-bit encoded u128
///     value: SevenBitU128,
/// }
/// ```
///
/// [`BinaryReader.read()`]: https://learn.microsoft.com/en-us/dotnet/api/system.io.binaryreader.read7bitencodedint?view=net-9.0
/// [`BinaryWriter.write()`]: https://learn.microsoft.com/en-us/dotnet/api/system.io.binarywriter.write7bitencodedint?view=net-9.0
#[derive(Clone, Default, PartialEq, PartialOrd, Eq, Ord)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SevenBitU128(u128);
