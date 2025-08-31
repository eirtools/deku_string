#![allow(
    clippy::tests_outside_test_module,
    reason = "<https://github.com/rust-lang/rust-clippy/issues/11024>"
)]

use deku_string::{Encoding, Size, StringDeku, StringLayout, VecDeku, VecLayout};
use test_utils::{assert_model_read, assert_model_write};

use rstest::rstest;

#[derive(
    Default, Debug, Clone, PartialEq, PartialOrd, deku::DekuRead, deku::DekuWrite,
)]
#[deku(endian = "little")]
struct LayoutsTestModel {
    #[deku(ctx = "VecLayout::FixedLength(10)")]
    fixed: VecDeku<u8>,

    #[deku(ctx = "VecLayout::LengthPrefix(Size::U8)")]
    prefixed_u8: VecDeku<u8>,

    #[deku(ctx = "VecLayout::LengthPrefix(Size::U16)")]
    prefixed_u16: VecDeku<u8>,

    #[deku(ctx = "VecLayout::LengthPrefix(Size::U32)")]
    prefixed_u32: VecDeku<u8>,

    #[deku(ctx = "VecLayout::LengthPrefix(Size::U32), (Encoding::Utf8, \
                  StringLayout::LengthPrefix(Size::U8))")]
    vec_of_strings: VecDeku<StringDeku>,

    #[deku(ctx = "VecLayout::End")]
    end: VecDeku<u8>,
}

const EXPECTED_BYTES: &[u8; 29] = &[
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 4, 116, 101, 115,
    116, 0, 0, 0,
];
const EXPECTED_BYTES_DEFAULT: &[u8; 21] = &[0; 21];

fn sample_model() -> LayoutsTestModel {
    LayoutsTestModel {
        fixed: VecDeku::new(&[0; 10]),
        end: VecDeku::new(&[0; 3]),
        vec_of_strings: VecDeku::new(&[StringDeku::new("test")]),
        ..Default::default()
    }
}

fn default_model() -> LayoutsTestModel {
    LayoutsTestModel {
        fixed: VecDeku::new(&[0; 10]),
        ..Default::default()
    }
}

#[rstest]
#[case::sample(sample_model(), EXPECTED_BYTES)]
#[case::sample(default_model(), EXPECTED_BYTES_DEFAULT)]
fn write_model(#[case] model: LayoutsTestModel, #[case] expected_bytes: &[u8]) {
    assert_model_write(&model, expected_bytes);
}

#[rstest]
#[case::sample(EXPECTED_BYTES, sample_model())]
#[case::sample(EXPECTED_BYTES_DEFAULT, default_model())]
fn read_model(#[case] bytes: &[u8], #[case] expected_model: LayoutsTestModel) {
    assert_model_read(bytes, &expected_model);
}
