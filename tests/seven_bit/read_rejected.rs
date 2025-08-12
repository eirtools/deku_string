#![allow(dead_code, unused_macros, unused_imports)]

mod data;
mod macros;

use pastey::paste;
use rstest::rstest;

use deku::DekuReader as _;
use deku::reader::Reader;

use data::read_rejected::*;
use deku_string::{SevenBitU8, SevenBitU16, SevenBitU32, SevenBitU64, SevenBitU128};

#[allow(unused_imports)]
use macros::read_rejected;

use crate::macros::misc::{_match_error, _rejected_check};
use crate::macros::read_rejected::create_test_impl_read_rejected;

create_test_impl_read_rejected!(
    error: incomplete,
    (incomplete_middle),
    (incomplete_end),
);

create_test_impl_read_rejected!(
    error: assertion,
    (overflow),
);
