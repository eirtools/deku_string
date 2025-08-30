#![allow(
    clippy::tests_outside_test_module,
    reason = "<https://github.com/rust-lang/rust-clippy/issues/11024>"
)]

use ::deku_string::{Size, VecDeku, VecLayout};
use deku::{DekuContainerRead as _, DekuContainerWrite as _};
use rstest::{fixture, rstest};

#[derive(
    Default, Debug, Clone, PartialEq, PartialOrd, ::deku::DekuRead, ::deku::DekuWrite,
)]
#[deku(endian = "little")]
struct SampleModel {
    // fixed length buffer
    //
    // Minimal byte length is 10 (size of the buffer)
    #[deku(ctx = "VecLayout::FixedLength(10)")]
    fixed: VecDeku<u8>,

    // length (1 byte) then data
    //
    // Minimal byte length is 1
    #[deku(ctx = "VecLayout::LengthPrefix(Size::U8)")]
    prefixed_u8: VecDeku<u8>,

    // length (2 byte) then data
    //
    // Minimal byte length is 2
    #[deku(ctx = "VecLayout::LengthPrefix(Size::U16)")]
    prefixed_u16: VecDeku<u8>,

    // length (4 byte) then data
    //
    // Minimal byte length is 4
    #[deku(ctx = "VecLayout::LengthPrefix(Size::U32)")]
    prefixed_u32: VecDeku<u8>,

    // Variable-length array, till the end of buffer
    //
    // Minimal byte length is 0
    #[deku(ctx = "VecLayout::End")]
    end: VecDeku<u8>,
}

const EXPECTED_BYTES: &[u8; 20] = &[0; 20];
const EXPECTED_BYTES_DEFAULT: &[u8; 17] = &[0; 17];

#[fixture]
pub fn sample_model() -> SampleModel {
    SampleModel {
        fixed: VecDeku::new(&[0; 10]),
        end: VecDeku::new(&[0; 3]),
        ..Default::default()
    }
}

#[fixture]
pub fn default_model() -> SampleModel {
    SampleModel {
        fixed: VecDeku::new(&[0; 10]),
        ..Default::default()
    }
}

#[rstest]
#[case::sample(sample_model, EXPECTED_BYTES)]
#[case::sample(default_model, EXPECTED_BYTES_DEFAULT)]
fn write_model(#[case] model: fn() -> SampleModel, #[case] bytes: &[u8]) {
    let result = model().to_bytes().expect("Unexpected error");
    assert_eq!(result, bytes);
}

#[rstest]
#[case::sample(sample_model, EXPECTED_BYTES)]
#[case::sample(default_model, EXPECTED_BYTES_DEFAULT)]
fn read_model(#[case] model: fn() -> SampleModel, #[case] bytes: &[u8]) {
    let ((rest, size_left), value) =
        SampleModel::from_bytes((bytes, 0)).expect("Unexpected error");

    assert_eq!(value, model());
    assert_eq!(size_left, 0);
    assert_eq!(rest.len(), 0);
}
