//! Model write related functions
#![allow(clippy::expect_used, reason = "Code is designed for testing")]
#![allow(clippy::missing_panics_doc, reason = "Code is designed to panic")]

use alloc::vec::Vec;
use deku::no_std_io::Cursor;
use deku::writer::Writer;
use deku::{DekuContainerWrite, DekuError, DekuWriter};

use crate::FaultyWriteBuffer;

#[inline]
pub fn assert_model_write<Model>(model: &Model, expected_bytes: &[u8])
where
    Model: DekuContainerWrite,
{
    let output = model.to_bytes().expect("Unexpected error");
    assert_eq!(output, expected_bytes, "Bytes written incorrectly");
}

#[inline]
pub fn assert_model_write_ctx<Model, Ctx>(model: Model, expected_bytes: &[u8], ctx: Ctx)
where
    Model: DekuWriter<Ctx>,
    Ctx: Copy,
{
    let mut output = Vec::new();
    let mut cursor = Cursor::new(&mut output);
    let deku_writer = &mut Writer::new(&mut cursor);
    model.to_writer(deku_writer, ctx).expect("Unexpected error");
    deku_writer.finalize().expect("Deku writer finalize");

    assert_eq!(output, expected_bytes, "Bytes written incorrectly");
}

#[inline]
#[must_use]
pub fn assert_model_write_error<Model, Ctx>(
    model: Model,
    ctx: Ctx,
    byte_breaks: u64,
) -> DekuError
where
    Model: DekuWriter<Ctx>,
    Ctx: Copy,
{
    let mut output = FaultyWriteBuffer::new(byte_breaks);
    let mut deku_writer = Writer::new(&mut output);

    model
        .to_writer(&mut deku_writer, ctx)
        .expect_err("Error was expected, data has been written")
}
