#![allow(dead_code, unused_macros, unused_imports, reason = "tests")]
#![allow(
    clippy::tests_outside_test_module,
    reason = "<https://github.com/rust-lang/rust-clippy/issues/11024>"
)]

mod data;
mod macros;

use pastey::paste;
use rstest::rstest;

use deku::DekuWriter as _;
use deku::ctx::Endian;
use deku::writer::Writer;

use data::layouts::*;
use data::write_rejected::*;

use deku_string::{StringDeku, VecDeku};
use test_utils::assert_model_write_error;

use macros::misc::*;
use macros::write_rejected::{
    _create_test_impl_write_rejected_size, create_test_impl_write_rejected,
};

create_test_impl_write_rejected!(
    fixed,
    error: io,
    (data),
    (less_data),
    (less_rest),
);

create_test_impl_write_rejected!(
    prefix_u8,
    error: io,
    (prefix),
    (data),
);

create_test_impl_write_rejected!(
    prefix_u16,
    error: io,
    (prefix),
    (data),
);

create_test_impl_write_rejected!(
    prefix_u32,
    error: io,
    (prefix),
    (data),
);

create_test_impl_write_rejected!(
    prefix_u32_7bit,
    error: io,
    (prefix),
    (data),
);

create_test_impl_write_rejected!(
    end,
    error: io,
    (data),
);

create_test_impl_write_rejected!(
    fixed0,
    data: u8,
    error: assertion,
    (too_big_data),
);

create_test_impl_write_rejected!(
    fixed,
    data: u8,
    error: assertion,
    (too_big_data),
);

create_test_impl_write_rejected!(
    prefix_u8,
    data: u8,
    error: assertion,
    (too_big_data),
);
