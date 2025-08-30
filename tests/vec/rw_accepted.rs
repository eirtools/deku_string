#![allow(dead_code, unused_macros, unused_imports, reason = "tests")]
#![allow(
    clippy::tests_outside_test_module,
    reason = "<https://github.com/rust-lang/rust-clippy/issues/11024>"
)]

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
use deku_string::{StringDeku, VecDeku};

use macros::accepted::*;
use macros::misc::*;

create_test_impl_rw_accepted! {
    fixed0,
    all_data,
    (empty, in_data_in),
}

create_test_impl_rw_accepted! {
    fixed,
    all_data,
    (empty, in_data_out),
    (no_padding, in_data_in),
    (padded_data, in_data_out),
}

create_test_impl_rw_accepted! {
    prefix_u8,
    data: u8,
    (empty, in_data_in),
    (no_padding, in_data_in),
    (len_255, in_data_in),
}

create_test_impl_rw_accepted! {
    prefix_u8,
    data: str,
    (empty, in_data_in),
    (no_padding, in_data_in),
}

create_test_impl_rw_accepted! {
    prefix_u16,
    all_data,
    (empty, in_data_in),
    (no_padding, in_data_in),
}

create_test_impl_rw_accepted! {
    prefix_u32,
    all_data,
    (empty, in_data_in),
    (no_padding, in_data_in),
}

create_test_impl_rw_accepted! {
    prefix_u32_7bit,
    all_data,
    (empty, in_data_in),
    (no_padding, in_data_in),
}

create_test_impl_rw_accepted! {
    end,
    all_data,
    (empty, in_data_in),
    (no_padding, in_data_in),
}
