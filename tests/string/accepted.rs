#![allow(dead_code, unused_macros, unused_imports, reason = "tests")]
#![allow(
    clippy::tests_outside_test_module,
    reason = "<https://github.com/rust-lang/rust-clippy/issues/11024>"
)]

//! Accepted read and write tests for UTF-8 & UTF-16 strings
mod data;
mod macros;

use pastey::paste;
use rstest::rstest;

use deku::ctx::Endian;
use deku::reader::Reader;
use deku::writer::Writer;
use deku::{DekuReader as _, DekuWriter as _, no_std_io};

use data::accepted::*;
use data::layouts::*;
use deku_string::{Encoding, StringDeku, StringLayout};

use test_utils::{assert_model_read_ctx, assert_model_write_ctx};

use macros::accepted::*;
use macros::misc::*;

// --------------------------------------
// ---------- TESTS: Parse --------------
// --------------------------------------

create_test_impl_rw_accepted! {
    fixed_force_zero,
    all_encodings,
    (empty, in_data_in),
    (valid, in_data_in),
    (zero_in_middle, in_data_out),
}

create_test_impl_rw_accepted! {
    fixed_allow_no_zero,
    all_encodings,
    (empty, in_data_in),
    (valid, in_data_in),
    (zero_in_middle, in_data_out),
    (no_null_inside, in_data_in),
}

create_test_impl_rw_accepted! {
    prefix_u8,
    all_encodings,
    (empty, in_data_in),
    (valid, in_data_in),
    (len_255, in_data_in),
}

create_test_impl_rw_accepted! {
    prefix_u16,
    all_encodings,
    (empty, in_data_in),
    (valid, in_data_in),
}

create_test_impl_rw_accepted! {
    prefix_u32,
    all_encodings,
    (empty, in_data_in),
    (valid, in_data_in),
}

create_test_impl_rw_accepted! {
    prefix_u32_7bit,
    all_encodings,
    (empty, in_data_in),
    (valid, in_data_in),
}

create_test_impl_rw_accepted! {
    zero_ended,
    all_encodings,
    (empty, in_data_in),
    (valid, in_data_in),
}
