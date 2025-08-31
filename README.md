# Easy out-of-the-box read and write strings in various formats using Deku

[![Crates.io](https://img.shields.io/crates/v/deku_string.svg)](https://crates.io/crates/deku_string)
[![Docs.rs](https://docs.rs/deku_string/badge.svg)](https://docs.rs/deku_string)

**`deku_string`** is an utility crate for [Deku](https://docs.rs/deku) that provides convenient support
for serializing and deserializing String and Vec in a variety of binary formats.

It handles UTF-8, UTF-16 and UTF-32 encoded strings and is suitable for parsing complex layouts.

### ✨ Features

* String support UTF-8, UTF-16 and UTF-32 support.
* Multiple layout formats, such as .Net, Pascal and zero-ended.
* Little and Big Endian support.
* Dynamic read and write without additional temporary structs and operations.
* No need to specify custom reader and writer.
* Compatible with Deku's derive macros and custom readers/writers.
* Supports serde via `serde` feature and `defmt` for embedded programming.

### Supported plenty layout formats

* **Fixed-length strings**
  * With or without zero-termination
* **Pascal-style strings**
  * Length-prefixed strings having length size `u8`, `u16`, or `u32`
* **.NET length prefix**
  * Length-prefixed strings having length size `u32` 7-bit encoded (like in .NET)
* **C-style strings**
  * Zero-terminated strings

### 📦 Usage

```rust
#[derive(Default, Debug, Clone, PartialEq, PartialOrd, deku::DekuRead, deku::DekuWrite)]
#[deku(endian = "little")]
struct SampleModel {
    #[deku(ctx = "Encoding::Utf8, StringLayout::fixed_length(10)")]
    fixed_value: StringDeku,

    // Vector of strings, each of them have their own layout.
    #[deku(ctx = "VecLayout::LengthPrefix(Size::U32_7Bit), (Encoding::Utf8, StringLayout::LengthPrefix(Size::U8))")]
    vec_of_strings: VecDeku<StringDeku>

    // Read u8 into vector to the end.
    #[deku(ctx = "VecLayout::FixedSize(10)")]
    vec: VecDeku<u8>,
}
```

### Contributing

Contributions are welcome!

Whether you're fixing a bug, improving performance, adding support for new string formats, or just refining docs — your pull requests and suggestions are greatly appreciated.
