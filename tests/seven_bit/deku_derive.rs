//!
//! Integration test with Deku derives and example how to use.

use ::deku_string::{SevenBitU8, SevenBitU16, SevenBitU32, SevenBitU64, SevenBitU128};
use deku::{DekuContainerRead as _, DekuContainerWrite as _};
use rstest::rstest;

#[derive(Default, Debug, Clone, PartialEq, PartialOrd)] // usual stuff
#[derive(::deku::DekuRead, ::deku::DekuWrite)] // deku
struct SampleModel {
    value_u8: SevenBitU8,
    value_u16: SevenBitU16,
    value_u32: SevenBitU32,
    value_u64: SevenBitU64,
    value_u128: SevenBitU128,
}

const EXPECTED_BYTES: &[u8; 5] = &[0; 5];

#[rstest]
fn write_model() {
    let model = SampleModel::default();

    let value = model.to_bytes().expect("Unexpected error");
    assert_eq!(value, EXPECTED_BYTES);
}

#[rstest]
fn read_model() {
    let expected_model = SampleModel::default();

    let ((rest, size_left), value) =
        SampleModel::from_bytes((EXPECTED_BYTES, 0)).expect("Unexpected error");

    assert_eq!(value, expected_model);
    assert_eq!(size_left, 0);
    assert_eq!(rest.len(), 0);
}
