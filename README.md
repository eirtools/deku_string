# Easy out-of-the-box read and write strings in various formats using Deku

[![Crates.io](https://img.shields.io/crates/v/deku_string.svg)](https://crates.io/crates/deku_string)
[![Docs.rs](https://docs.rs/deku_string/badge.svg)](https://docs.rs/deku_string)

**`deku_string`** is an utility crate for [Deku](https://docs.rs/deku) that provides convenient support
for serializing and deserializing strings in a variety of binary formats.
It handles both UTF-8 and UTF-16 encoded strings and is suitable for parsing various string layouts.

### ✨ Features

* UTF-8 and UTF-16 support.
* Multiple string encoding and layout formats.
* Little and Big Endian support.
* Dynamic read and write without additional temporary structs and operations.
* No need to specify custom reader and writer.
* Compatible with Deku's derive macros and custom readers/writers.
* Supports serde via `serde` feature

### Supported Layout Formats

* **Fixed-length strings**
  * With or without zero-termination
* **Pascal-style strings**
  * Length-prefixed strings having length size `u8`, `u16`, or `u32`
* **C-style strings**
  * Zero-terminated strings

### 📦 Usage

See full example in [`tests/deku_derive.rs`](./tests/deku_derive.rs)

```rust
#[derive(Default, Debug, Clone, PartialEq, PartialOrd, ::deku::DekuRead, ::deku::DekuWrite)]
#[deku(endian = "little")]
struct SampleModel {
    // fixed length buffer, null  character is required to be inside
    // "012345678\x00" is allowed
    // "0123456789" is NOT allowed
    #[deku(ctx = "Encoding::Utf8, StringLayout::fixed_length(10)")]
    fixed_value: StringDeku,

    // length (1 byte) then string, null character is NOT allowed inside
    // b"\0x501234"
    #[deku(ctx = "Encoding::Utf8, StringLayout::LengthPrefix(Size::U8)")]
    prefixed_u8: StringDeku,

    // String with null byte at the end
    // b"012345\x00"
    #[deku(ctx = "Encoding::Utf8, StringLayout::ZeroEnded")]
    zero_ended: StringDeku,
}
```

### 🤝 Contributing

Contributions are warmly welcome!

Whether you're fixing a bug, improving performance, adding support for new string formats, or just refining docs — your pull requests and suggestions are greatly appreciated.
