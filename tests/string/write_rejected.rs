//! Rejected write tests
mod data;
mod macros;

use pastey::paste;
use rstest::rstest;

use deku::DekuWriter as _;
use deku::ctx::Endian;
use deku::writer::Writer;

use deku_string::{Encoding, StringDeku};

use data::invalid_io::InvalidBufferType;
use data::layouts::*;
use data::write_rejected::*;

#[allow(unused_imports)]
use macros::write_rejected;

create_test_impl_write_rejected! {
    fixed_force_zero,
    error: assertion,
    (no_zero_inside),
    (zero_inside_str),
    (too_big_data),
}

create_test_impl_write_rejected! {
    fixed_allow_no_zero,
    error: assertion,
    (zero_inside_str),
    (too_big_data),
}

create_test_impl_write_rejected! {
    prefix_u8,
    error: assertion,
    (zero_inside_str),
    (too_big_data),
}

create_test_impl_write_rejected! {
    prefix_u16,
    error: assertion,
    (zero_inside_str),
    (too_big_data),
}

create_test_impl_write_rejected! {
    prefix_u32,
    error: assertion,
    (zero_inside_str),
    // it's not worth a while to test this due requirements to save 4GB string in heap
    // (too_big_data),
}

create_test_impl_write_rejected! {
    zero_ended,
    error: assertion,
    (zero_inside_str),
}

// -------

create_test_impl_write_rejected! {
    fixed_force_zero,
    error: io,
    (data),
}

create_test_impl_write_rejected! {
    fixed_allow_no_zero,
    error: io,
    (data),
}

create_test_impl_write_rejected! {
    prefix_u8,
    error: io,
    (data),
    (prefix),
}

create_test_impl_write_rejected! {
    prefix_u16,
    error: io,
    (data),
    (prefix),
}

create_test_impl_write_rejected! {
    prefix_u32,
    error: io,
    (data),
    (prefix),
}

create_test_impl_write_rejected! {
    zero_ended,
    error: io,
    (data),
    (suffix),
}
