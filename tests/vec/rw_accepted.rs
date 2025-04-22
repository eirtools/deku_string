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

#[allow(unused_imports)]
use macros::accepted;

create_test_impl_rw_accepted! {
    fixed0,
    all_data,
    (empty, in_data_in),
}

create_test_impl_rw_accepted! {
    fixed,
    all_data,
    (full, in_data_in),
    (partial, in_data_out),
}

create_test_impl_rw_accepted! {
    prefix_u8,
    all_data,
    (empty, in_data_in),
    (full, in_data_in),
}

create_test_impl_rw_accepted! {
    prefix_u16,
    all_data,
    (empty, in_data_in),
    (full, in_data_in),
}

create_test_impl_rw_accepted! {
    prefix_u32,
    all_data,
    (empty, in_data_in),
    (full, in_data_in),
}

create_test_impl_rw_accepted! {
    end,
    all_data,
    (empty, in_data_in),
    (full, in_data_in),
}
