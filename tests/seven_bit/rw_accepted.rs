mod data;
mod macros;

use pastey::paste;
use rstest::rstest;

use deku::reader::Reader;
use deku::writer::Writer;
use deku::{DekuReader as _, DekuWriter as _, no_std_io};

use data::accepted::*;
use deku_string::{SevenBitU32, SevenBitU64, SevenBitU128};

#[allow(unused_imports)]
use macros::accepted;

create_test_impl_rw_accepted! {
    (zero),
    (less_128),
    (eq_127),
    (greater_127),
    (max),
}
