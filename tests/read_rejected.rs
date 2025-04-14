//!
//! Rejected read tests for UTF-8 & UTF-16 strings
//!
mod data;
mod test_gen_macro;

use pastey::paste;
use rstest::rstest;

use deku::ctx::Endian;
use deku::reader::Reader;
use deku::DekuReader as _;

use data::layouts::*;
use data::rejected::*;
use deku_string::{Encoding, StringDeku};

// --------------------------------------
// ---------- TESTS: Parse --------------
// --------------------------------------

// ---------- Parse
create_test_impl_read_rejected! {
    error: parse,
    (invalid_utf),
}

// --------------------------------------
// ---------- TESTS: Assertion ----------
// --------------------------------------

create_test_impl_read_rejected! {
    fixed_force_zero,
    error: assertion,
    (no_zero_inside),
}

create_test_impl_read_rejected! {
    prefix_u8,
    error: assertion,
    (zero_in_middle),
}

create_test_impl_read_rejected! {
    prefix_u16,
    error: assertion,
    (zero_in_middle),
}

create_test_impl_read_rejected! {
    prefix_u32,
    error: assertion,
    (zero_in_middle),
}

// ---------- Incomplete
create_test_impl_read_rejected! {
    fixed_force_zero,
    error: incomplete,
    (small_buffer),
}

create_test_impl_read_rejected! {
    fixed_allow_no_zero,
    error: incomplete,
    (small_buffer),
}

create_test_impl_read_rejected! {
    prefix_u8,
    error: incomplete,
    (small_buffer),
    (short_len),
    (short_data),
}

create_test_impl_read_rejected! {
    prefix_u16,
    error: incomplete,
    (small_buffer),
    (short_len),
    (short_data),
}

create_test_impl_read_rejected! {
    prefix_u32,
    error: incomplete,
    (small_buffer),
    (short_len),
    (short_data),
}

create_test_impl_read_rejected! {
    zero_ended,
    error: incomplete,
    (small_buffer),
    (no_zero_inside),
}
