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
use deku_string::{StringDeku, VecDeku};

use macros::misc::*;
use macros::read_rejected::create_test_impl_read_rejected;

create_test_impl_read_rejected!(fixed, (data),);

create_test_impl_read_rejected!(prefix_u8, (prefix), (data),);

create_test_impl_read_rejected!(prefix_u16, (prefix), (data),);

create_test_impl_read_rejected!(prefix_u32, (prefix), (data),);

create_test_impl_read_rejected!(prefix_u32_7bit, (prefix), (data),);
