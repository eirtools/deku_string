mod data;
mod macros;

use pastey::paste;
use rstest::rstest;

use deku::DekuWriter as _;
use deku::writer::Writer;

use data::write_rejected::*;
use deku_string::{SevenBitU32, SevenBitU64, SevenBitU128};

#[allow(unused_imports)]
use macros::write_rejected;

use io_rejected::InvalidBufferType;

create_test_impl_write_rejected!((data), (data_end),);
