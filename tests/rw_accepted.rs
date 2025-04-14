//!
//! Accepted read and write tests for UTF-8 & UTF-16 strings
//!
mod data;
mod test_gen_macro;

use pastey::paste;
use rstest::rstest;

use deku::ctx::Endian;
use deku::reader::Reader;
use deku::writer::Writer;
use deku::{no_std_io, DekuReader as _, DekuWriter as _};

use data::accepted::*;
use data::layouts::*;
use deku_string::{Encoding, StringDeku};

// --------------------------------------
// ---------- TESTS: Parse --------------
// --------------------------------------

create_test_impl_rw_accepted! {
    fixed_force_zero,
    all_encodings,
    (empty, in_str_in),
    (valid, in_str_in),
    (zero_in_middle, in_str_out)
}

create_test_impl_rw_accepted! {
    fixed_allow_no_zero,
    all_encodings,
    (empty, in_str_in),
    (valid, in_str_in),
    (zero_in_middle, in_str_out),
    (no_zero_inside, in_str_in)
}

create_test_impl_rw_accepted! {
    prefix_u8,
    all_encodings,
    (empty, in_str_in),
    (valid, in_str_in)
}

create_test_impl_rw_accepted! {
    prefix_u16,
    all_encodings,
    (empty, in_str_in),
    (valid, in_str_in)
}

create_test_impl_rw_accepted! {
    prefix_u32,
    all_encodings,
    (empty, in_str_in),
    (valid, in_str_in)
}

create_test_impl_rw_accepted! {
    zero_ended,
    all_encodings,
    (empty, in_str_in),
    (valid, in_str_in)
}
