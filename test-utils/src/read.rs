//! Model read related functions
#![allow(clippy::expect_used, reason = "Code is designed for testing")]
#![allow(clippy::missing_panics_doc, reason = "Code is designed to panic")]

use core::fmt::Debug;
use deku::no_std_io::Cursor;
use deku::reader::Reader;
use deku::{DekuContainerRead, DekuError, DekuReader};

#[inline]
pub fn assert_model_read<'a, Model>(bytes: &'a [u8], expected_model: &Model)
where
    Model: DekuContainerRead<'a> + PartialEq + Debug,
{
    let ((rest, size_left), value) =
        Model::from_bytes((bytes, 0)).expect("Unexpected error");

    assert_eq!(value, *expected_model, "Bytes read incorrectly");
    assert_eq!(size_left, 0, "There's leftovers");
    assert_eq!(rest.len(), 0, "There's leftovers");
}

#[inline]
pub fn assert_model_read_ctx<'a, Model, Ctx>(
    bytes: &'a [u8],
    expected_model: &Model,
    ctx: Ctx,
) where
    Model: DekuReader<'a, Ctx> + PartialEq + Debug,
{
    let mut cursor = Cursor::new(bytes);
    let mut reader = Reader::new(&mut cursor);

    let model =
        Model::from_reader_with_ctx(&mut reader, ctx).expect("Unexpected error");
    assert_eq!(model, *expected_model, "Bytes read incorrectly");
}

#[inline]
#[must_use]
pub fn assert_model_read_error<'a, Model, Ctx>(bytes: &'a [u8], ctx: Ctx) -> DekuError
where
    Model: DekuReader<'a, Ctx> + PartialEq + Debug,
    Ctx: Copy,
{
    let mut cursor = Cursor::new(bytes);
    let mut deku_reader = Reader::new(&mut cursor);

    Model::from_reader_with_ctx(&mut deku_reader, ctx)
        .expect_err("Error was expected, data has been read")
}
