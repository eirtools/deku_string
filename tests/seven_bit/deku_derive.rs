#![allow(
    clippy::tests_outside_test_module,
    reason = "<https://github.com/rust-lang/rust-clippy/issues/11024>"
)]

use deku::{DekuContainerRead as _, DekuContainerWrite as _};
use deku_string::{SevenBitU8, SevenBitU16, SevenBitU32, SevenBitU64, SevenBitU128};
use rstest::rstest;

#[derive(Default, Debug, Clone, PartialEq, PartialOrd)] // usual stuff
#[derive(::deku::DekuRead, ::deku::DekuWrite)] // deku
struct LayoutsTestModel {
    uint8: SevenBitU8,
    uint16: SevenBitU16,
    uint32: SevenBitU32,
    uint64: SevenBitU64,
    uint128: SevenBitU128,
}

const EXPECTED_BYTES: &[u8; 5] = &[0; 5];

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
