#![allow(dead_code, unused_macros, unused_imports, reason = "tests")]
#![allow(
    clippy::tests_outside_test_module,
    reason = "<https://github.com/rust-lang/rust-clippy/issues/11024>"
)]

mod data;
mod macros;

use pastey::paste;
use rstest::rstest;

use deku::DekuReader as _;
use deku::ctx::Endian;
use deku::reader::Reader;

use data::layouts::*;
use data::read_rejected::*;
use deku_string::{Encoding, StringDeku};

use macros::misc::*;
use macros::read_rejected::*;

// --------------------------------------
// ---------- TESTS: Parse --------------
// --------------------------------------

// ---------- Parse
create_test_impl_read_rejected! {
    fixed_force_zero,
    error: parse,
    (invalid_utf),
}

create_test_impl_read_rejected! {
    fixed_allow_no_zero,
    error: parse,
    (invalid_utf),
}

create_test_impl_read_rejected! {
    prefix_u8,
    error: parse,
    (invalid_utf),
}

create_test_impl_read_rejected! {
    prefix_u16,
    error: parse,
    (invalid_utf),
}

create_test_impl_read_rejected! {
    prefix_u32,
    error: parse,
    (invalid_utf),
}

create_test_impl_read_rejected! {
    prefix_u32_7bit,
    error: parse,
    (invalid_utf),
}

create_test_impl_read_rejected! {
    zero_ended,
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

create_test_impl_read_rejected! {
    prefix_u32_7bit,
    error: assertion,
    (zero_in_middle),
    (invalid_size_encoding)
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
    prefix_u32_7bit,
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
