//!
//! Integration test with Deku derives and example how to use.

use ::deku_string::{SevenBitU32, SevenBitU64, SevenBitU128};
use deku::{DekuContainerRead, DekuContainerWrite};
use rstest::rstest;

#[derive(Default, Debug, Clone, PartialEq, PartialOrd)] // usual stuff
#[derive(::deku::DekuRead, ::deku::DekuWrite)] // deku
struct SampleModel {
    seven_bit_u32: SevenBitU32,
    seven_bit_u64: SevenBitU64,
    seven_bit_u128: SevenBitU128,
}

const EXPECTED_BYTES: &[u8; 3] = &[0; 3];

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
            assert_eq!(rest, &[]);
        }
        Err(value) => panic!("Got unexpected error {value:#?}"),
    }
}
