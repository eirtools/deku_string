#![allow(
    clippy::tests_outside_test_module,
    reason = "<https://github.com/rust-lang/rust-clippy/issues/11024>"
)]

use deku::{DekuContainerRead as _, DekuContainerWrite as _};
use deku_string::{Encoding, Size, StringDeku, StringLayout};
use rstest::rstest;

#[derive(
    Default, Debug, Clone, PartialEq, PartialOrd, deku::DekuRead, deku::DekuWrite,
)]
#[deku(endian = "little")]
struct LayoutsTestModel {
    #[deku(ctx = "Encoding::Utf8, StringLayout::fixed_length(10)")]
    utf8_fixed_value_force_zero: StringDeku,

    #[deku(ctx = "Encoding::Utf8, StringLayout::FixedLength{size: 10, \
                  allow_no_null: true}")]
    utf8_fixed_value: StringDeku,

    #[deku(ctx = "Encoding::Utf8, StringLayout::LengthPrefix(Size::U8)")]
    utf8_prefixed_u8: StringDeku,

    #[deku(ctx = "Encoding::Utf8, StringLayout::LengthPrefix(Size::U16)")]
    utf8_prefixed_u16: StringDeku,

    #[deku(ctx = "Encoding::Utf8, StringLayout::LengthPrefix(Size::U32)")]
    utf8_prefixed_u32: StringDeku,

    #[deku(ctx = "Encoding::Utf8, StringLayout::LengthPrefix(Size::U32_7Bit)")]
    utf8_prefixed_u32_7bit: StringDeku,

    #[deku(ctx = "Encoding::Utf8, StringLayout::ZeroEnded")]
    utf8_zero_ended: StringDeku,

    #[deku(ctx = "Encoding::Utf16, StringLayout::fixed_length(10)")]
    utf16_fixed_value_force_zero: StringDeku,

    #[deku(ctx = "Encoding::Utf16, StringLayout::FixedLength{size: 10, \
                  allow_no_null: true}")]
    utf16_fixed_value: StringDeku,

    #[deku(ctx = "Encoding::Utf16, StringLayout::LengthPrefix(Size::U8)")]
    utf16_prefixed_u8: StringDeku,

    #[deku(ctx = "Encoding::Utf16, StringLayout::LengthPrefix(Size::U16)")]
    utf16_prefixed_u16: StringDeku,

    #[deku(ctx = "Encoding::Utf16, StringLayout::LengthPrefix(Size::U32)")]
    utf16_prefixed_u32: StringDeku,

    #[deku(ctx = "Encoding::Utf16, StringLayout::ZeroEnded")]
    utf16_zero_ended: StringDeku,
}

const EXPECTED_BYTES: &[u8; 78] = &[0; 78];

#[rstest]
fn write_model() {
    let model = LayoutsTestModel::default();

    let value = model.to_bytes().expect("Unexpected error");
    assert_eq!(value, EXPECTED_BYTES);
}

#[rstest]
fn read_model() {
    let expected_model = LayoutsTestModel::default();

    let ((rest, size_left), value) =
        LayoutsTestModel::from_bytes((EXPECTED_BYTES, 0)).expect("Unexpected error");

    assert_eq!(value, expected_model);
    assert_eq!(size_left, 0);
    assert_eq!(rest.len(), 0);
}
