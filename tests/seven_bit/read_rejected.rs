mod data;
mod macros;

use pastey::paste;
use rstest::rstest;

use deku::DekuReader as _;
use deku::reader::Reader;

use data::read_rejected::*;
use deku_string::{SevenBitU32, SevenBitU64, SevenBitU128};

#[allow(unused_imports)]
use macros::read_rejected;

create_test_impl_read_rejected!(
    error: incomplete,
    (incomplete_middle),
    (incomplete_end),
);

create_test_impl_read_rejected!(
    error: assertion,
    (overflow),
);
