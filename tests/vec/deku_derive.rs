use ::deku_string::{Size, VecDeku, VecLayout};
use deku::{DekuContainerRead, DekuContainerWrite};
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
    SampleModel::default()
}

#[rstest]
fn write_model(sample_model: SampleModel) {
    match sample_model.to_bytes() {
        Ok(value) => assert_eq!(value, EXPECTED_BYTES),
        Err(value) => panic!("Got unexpected error {value:#?}"),
    }
}

#[rstest]
fn read_model(sample_model: SampleModel) {
    match SampleModel::from_bytes((EXPECTED_BYTES, 0)) {
        Ok(((rest, size_left), value)) => {
            assert_eq!(value, sample_model);
            assert_eq!(size_left, 0);
            assert_eq!(rest.len(), 0);
        }
        Err(value) => panic!("Got unexpected error {value:#?}"),
    }
}

#[rstest]
fn write_model_default(default_model: SampleModel) {
    match default_model.to_bytes() {
        Ok(value) => assert_eq!(value, EXPECTED_BYTES_DEFAULT),
        Err(value) => panic!("Got unexpected error {value:#?}"),
    }
}

#[rstest]
fn read_model_default(default_model: SampleModel) {
    match SampleModel::from_bytes((EXPECTED_BYTES_DEFAULT, 0)) {
        Ok(((rest, size_left), value)) => {
            assert_ne!(value, default_model);
            assert_eq!(size_left, 0);
            assert_eq!(rest.len(), 0);
        }
        Err(value) => panic!("Got unexpected error {value:#?}"),
    }
}
