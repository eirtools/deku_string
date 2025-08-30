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
use deku::writer::Writer;

use data::write_rejected::*;
use deku_string::{SevenBitU8, SevenBitU16, SevenBitU32, SevenBitU64, SevenBitU128};

use io_rejected::InvalidBufferType;

use crate::macros::misc::{_match_error, _rejected_check};
use crate::macros::write_rejected::create_test_impl_write_rejected;

create_test_impl_write_rejected!((data), (data_end),);
