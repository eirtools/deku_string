#![allow(dead_code, unused_macros, unused_imports, reason = "tests")]
#![allow(
    clippy::tests_outside_test_module,
    reason = "<https://github.com/rust-lang/rust-clippy/issues/11024>"
)]

mod data;
mod macros;

use pastey::paste;
use rstest::rstest;

use deku::reader::Reader;
use deku::writer::Writer;
use deku::{DekuReader as _, DekuWriter as _, no_std_io};

use data::accepted::*;
use deku_string::{SevenBitU8, SevenBitU16, SevenBitU32, SevenBitU64, SevenBitU128};

use crate::macros::accepted::{
    create_test_impl_read_accepted, create_test_impl_rw_accepted,
    create_test_impl_write_accepted,
};

create_test_impl_rw_accepted! {
    (zero),
    (less_128),
    (eq_127),
    (greater_127),
    (max),
}
