//!
//! Integration test with Deku derives and example how to use.
use ::deku_string::{Encoding, Size, StringDeku, StringLayout};
use deku::{DekuContainerRead, DekuContainerWrite};
use rstest::rstest;

#[derive(
    Default, Debug, Clone, PartialEq, PartialOrd, ::deku::DekuRead, ::deku::DekuWrite,
)]
#[deku(endian = "little")]
struct SampleModel {
    // fixed length buffer, null  character is required to be inside
    // "012345678\x00" is allowed
    // "0123456789" is NOT allowed
    //
    // byte length with empty string is 10
    #[deku(ctx = "Encoding::Utf8, StringLayout::fixed_length(10)")]
    utf8_fixed_value_force_zero: StringDeku,

    // fixed length buffer, null byte is allowed to be inside,
    // both "012345678\x00" and "0123456789" are allowed
    //
    // byte length with empty string is 10
    #[deku(ctx = "Encoding::Utf8, StringLayout::FixedLength{size: 10, \
                  allow_no_null: true}")]
    utf8_fixed_value: StringDeku,

    // length (1 byte) then string, null character is NOT allowed inside
    // b"\0x501234"
    //
    // byte length with empty string is 1
    #[deku(ctx = "Encoding::Utf8, StringLayout::LengthPrefix(Size::U8)")]
    utf8_prefixed_u8: StringDeku,

    // length (2 byte) then string, null character is NOT allowed inside
    // b"\0x5\x0001234"
    //
    // byte length with empty string is 2
    #[deku(ctx = "Encoding::Utf8, StringLayout::LengthPrefix(Size::U16)")]
    utf8_prefixed_u16: StringDeku,

    // length (4 byte) then string, null character is NOT allowed inside
    // b"\0x5\x00\x00\x0001234"
    //
    // byte length with empty string is 4
    #[deku(ctx = "Encoding::Utf8, StringLayout::LengthPrefix(Size::U32)")]
    utf8_prefixed_u32: StringDeku,

    // variable 32-bit (7-bit encoded) length then string, null character is NOT allowed inside
    // b"\0x501234"
    //
    // byte length with empty string is 1
    #[deku(ctx = "Encoding::Utf8, StringLayout::LengthPrefix(Size::U32_7Bit)")]
    utf8_prefixed_u32_7bit: StringDeku,

    // String with null byte at the end
    // b"012345\x00"
    //
    // byte length with empty string is 1
    #[deku(ctx = "Encoding::Utf8, StringLayout::ZeroEnded")]
    utf8_zero_ended: StringDeku,

    // fixed length buffer, null  character is required to be inside
    // "012345678\x00" is allowed
    // "0123456789" is NOT allowed
    //
    // byte length with empty string is 20
    #[deku(ctx = "Encoding::Utf16, StringLayout::fixed_length(10)")]
    utf16_fixed_value_force_zero: StringDeku,

    // fixed length buffer, null byte is allowed to be inside,
    // both "012345678\x00" and "0123456789" are allowed
    //
    // byte length with empty string is 20
    #[deku(ctx = "Encoding::Utf16, StringLayout::FixedLength{size: 10, \
                  allow_no_null: true}")]
    utf16_fixed_value: StringDeku,

    // length (1 byte) then string, null character is NOT allowed inside
    // b"\0x501234"
    //
    // byte length with empty string is 1
    #[deku(ctx = "Encoding::Utf16, StringLayout::LengthPrefix(Size::U8)")]
    utf16_prefixed_u8: StringDeku,

    // length (2 byte) then string, null character is NOT allowed inside
    // b"\0x5\x0001234"
    //
    // byte length with empty string is 2
    #[deku(ctx = "Encoding::Utf16, StringLayout::LengthPrefix(Size::U16)")]
    utf16_prefixed_u16: StringDeku,

    // length (4 byte) then string, null character is NOT allowed inside
    // b"\0x5\x00\x00\x0001234"
    //
    // byte length with empty string is 4
    #[deku(ctx = "Encoding::Utf16, StringLayout::LengthPrefix(Size::U32)")]
    utf16_prefixed_u32: StringDeku,

    // String with null byte at the end
    // b"012345\x00"
    //
    // byte length with empty string is 1
    #[deku(ctx = "Encoding::Utf16, StringLayout::ZeroEnded")]
    utf16_zero_ended: StringDeku,
}

const EXPECTED_BYTES: &[u8; 78] = &[0; 78];

#[rstest]
fn write_model() {
    let model = SampleModel::default();

    match model.to_bytes() {
        Ok(value) => assert_eq!(value, EXPECTED_BYTES),
        Err(value) => panic!("Got unexpected error {value:#?}"),
    }
}

#[rstest]
fn read_model() {
    let expected_model = SampleModel::default();

    match SampleModel::from_bytes((EXPECTED_BYTES, 0)) {
        Ok(((rest, size_left), value)) => {
            assert_eq!(value, expected_model);
            assert_eq!(size_left, 0);
            assert_eq!(rest.len(), 0);
        }
        Err(value) => panic!("Got unexpected error {value:#?}"),
    }
}
