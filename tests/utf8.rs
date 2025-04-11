//!
//! Accepted read and write tests for UTF-8 strings
//!
mod test_gen_macro;
use pastey::paste;
use rstest::rstest;

use deku::ctx::Endian;
use deku::reader::Reader;
use deku::writer::Writer;
use deku::{no_std_io, DekuReader as _, DekuWriter as _};

use deku_string::{Encoding, Layout, Size, StringDeku};

const FIXED_LENGTH: usize = 23;

const FIXED_EMPTY: &[u8; FIXED_LENGTH] = &[0; FIXED_LENGTH];
const FIXED_EMPTY_STR: &str = "";

const FIXED_ZERO_INSIDE: &[u8; FIXED_LENGTH] = b"valid test case w zero\x00";
const FIXED_ZERO_INSIDE_STR: &str = "valid test case w zero";

const FIXED_ZERO_MIDDLE: &[u8; FIXED_LENGTH] = b"valid test\x00with garbage";
const FIXED_ZERO_MIDDLE_STR: &str = "valid test";
const FIXED_ZERO_MIDDLE_WRITE: &[u8; FIXED_LENGTH] =
    b"valid test\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";

// This is readable if absence of zero byte is allowed
const FIXED_NO_ZERO_INSIDE: &[u8; FIXED_LENGTH] = b"valid test case no zero";
const FIXED_NO_ZERO_INSIDE_STR: &str = "valid test case no zero";
const FIXED_NO_ZERO_INSIDE_WRITE: &[u8; FIXED_LENGTH] = b"valid test case no zero";

const PREFIX_U8_EMPTY: &[u8; 1] = b"\x00";
const PREFIX_U8_EMPTY_STR: &str = "";

const PREFIX_U8_VALID: &[u8; 16] = b"\x0Fvalid test case";
const PREFIX_U8_VALID_STR: &str = "valid test case";

const PREFIX_U16_LITTLE_EMPTY: &[u8; 2] = b"\x00\x00";
const PREFIX_U16_LITTLE_EMPTY_STR: &str = "";

const PREFIX_U16_LITTLE_VALID: &[u8; 17] = b"\x0F\x00valid test case";
const PREFIX_U16_LITTLE_VALID_STR: &str = "valid test case";

const PREFIX_U32_LITTLE_EMPTY: &[u8; 4] = b"\x00\x00\x00\x00";
const PREFIX_U32_LITTLE_EMPTY_STR: &str = "";

const PREFIX_U32_LITTLE_VALID: &[u8; 19] = b"\x0F\x00\x00\x00valid test case";
const PREFIX_U32_LITTLE_VALID_STR: &str = "valid test case";

const PREFIX_U16_BIG_EMPTY: &[u8; 2] = b"\x00\x00";
const PREFIX_U16_BIG_EMPTY_STR: &str = "";

const PREFIX_U16_BIG_VALID: &[u8; 17] = b"\x00\x0Fvalid test case";
const PREFIX_U16_BIG_VALID_STR: &str = "valid test case";

const PREFIX_U32_BIG_EMPTY: &[u8; 4] = b"\x00\x00\x00\x00";
const PREFIX_U32_BIG_EMPTY_STR: &str = "";

const PREFIX_U32_BIG_VALID: &[u8; 19] = b"\x00\x00\x00\x0Fvalid test case";
const PREFIX_U32_BIG_VALID_STR: &str = "valid test case";

const ZERO_ENDED_EMPTY: &[u8; 1] = b"\x00";
const ZERO_ENDED_EMPTY_STR: &str = "";

const ZERO_ENDED_VALID: &[u8; 16] = b"valid test case\x00";
const ZERO_ENDED_VALID_STR: &str = "valid test case";

create_test_impl_rw_accepted! {
    fixed_force_zero,
    endian: little,
    encoding: utf_8,
    Layout::FixedLength {
        size: FIXED_LENGTH,
        allow_no_null:false,
    },
    (empty, FIXED_EMPTY, FIXED_EMPTY_STR, FIXED_EMPTY),
    (zero_at_end, FIXED_ZERO_INSIDE, FIXED_ZERO_INSIDE_STR, FIXED_ZERO_INSIDE),
    (zero_in_middle, FIXED_ZERO_MIDDLE, FIXED_ZERO_MIDDLE_STR, FIXED_ZERO_MIDDLE_WRITE)
}

create_test_impl_rw_accepted! {
    fixed_force_zero,
    endian: big,
    encoding: utf_8,
    Layout::FixedLength {
        size: FIXED_LENGTH,
        allow_no_null:false,
    },
    (empty, FIXED_EMPTY, FIXED_EMPTY_STR, FIXED_EMPTY),
    (zero_at_end, FIXED_ZERO_INSIDE, FIXED_ZERO_INSIDE_STR, FIXED_ZERO_INSIDE),
    (zero_in_middle, FIXED_ZERO_MIDDLE, FIXED_ZERO_MIDDLE_STR, FIXED_ZERO_MIDDLE_WRITE),
}

create_test_impl_rw_accepted! {
    fixed_allow_no_zero,
    endian: little,
    encoding: utf_8,
    Layout::FixedLength {
        size: FIXED_LENGTH,
        allow_no_null:true,
    },
    (empty, FIXED_EMPTY, FIXED_EMPTY_STR, FIXED_EMPTY),
    (zero_at_end, FIXED_ZERO_INSIDE, FIXED_ZERO_INSIDE_STR, FIXED_ZERO_INSIDE),
    (zero_in_middle, FIXED_ZERO_MIDDLE, FIXED_ZERO_MIDDLE_STR, FIXED_ZERO_MIDDLE_WRITE),
    (not_zero_ended, FIXED_NO_ZERO_INSIDE, FIXED_NO_ZERO_INSIDE_STR, FIXED_NO_ZERO_INSIDE_WRITE),
}

create_test_impl_rw_accepted! {
    fixed_allow_no_zero,
    endian: big,
    encoding: utf_8,
    Layout::FixedLength {
        size: FIXED_LENGTH,
        allow_no_null:true,
    },
    (empty, FIXED_EMPTY, FIXED_EMPTY_STR, FIXED_EMPTY),
    (zero_at_end, FIXED_ZERO_INSIDE, FIXED_ZERO_INSIDE_STR, FIXED_ZERO_INSIDE),
    (zero_in_middle, FIXED_ZERO_MIDDLE, FIXED_ZERO_MIDDLE_STR, FIXED_ZERO_MIDDLE_WRITE),
    (not_zero_ended, FIXED_NO_ZERO_INSIDE, FIXED_NO_ZERO_INSIDE_STR, FIXED_NO_ZERO_INSIDE_WRITE),
}

create_test_impl_rw_accepted! {
    prefix_u8,
    endian: little,
    encoding: utf_8,
    Layout::LengthPrefix(Size::U8),
    (empty, PREFIX_U8_EMPTY, PREFIX_U8_EMPTY_STR),
    (valid, PREFIX_U8_VALID, PREFIX_U8_VALID_STR),
}

create_test_impl_rw_accepted! {
    prefix_u8,
    endian: big,
    encoding: utf_8,
    Layout::LengthPrefix(Size::U8),
    (empty, PREFIX_U8_EMPTY, PREFIX_U8_EMPTY_STR),
    (valid, PREFIX_U8_VALID, PREFIX_U8_VALID_STR),
}
create_test_impl_rw_accepted! {
    prefix_u16,
    endian: little,
    encoding: utf_8,
    Layout::LengthPrefix(Size::U16),
    (empty, PREFIX_U16_LITTLE_EMPTY, PREFIX_U16_LITTLE_EMPTY_STR),
    (valid, PREFIX_U16_LITTLE_VALID, PREFIX_U16_LITTLE_VALID_STR),
}

create_test_impl_rw_accepted! {
    prefix_u16,
    endian: big,
    encoding: utf_8,
    Layout::LengthPrefix(Size::U16),
    (empty, PREFIX_U16_BIG_EMPTY, PREFIX_U16_BIG_EMPTY_STR),
    (valid, PREFIX_U16_BIG_VALID, PREFIX_U16_BIG_VALID_STR),
}

create_test_impl_rw_accepted! {
    prefix_u32,
    endian: little,
    encoding: utf_8,
    Layout::LengthPrefix(Size::U32),
    (empty, PREFIX_U32_LITTLE_EMPTY, PREFIX_U32_LITTLE_EMPTY_STR),
    (valid, PREFIX_U32_LITTLE_VALID, PREFIX_U32_LITTLE_VALID_STR),
}

create_test_impl_rw_accepted! {
    prefix_u32,
    endian: big,
    encoding: utf_8,
    Layout::LengthPrefix(Size::U32),
    (empty, PREFIX_U32_BIG_EMPTY, PREFIX_U32_BIG_EMPTY_STR),
    (valid, PREFIX_U32_BIG_VALID, PREFIX_U32_BIG_VALID_STR),
}

create_test_impl_rw_accepted! {
    zero_ended,
    endian: little,
    encoding: utf_8,
    Layout::ZeroEnded,
    (empty, ZERO_ENDED_EMPTY, ZERO_ENDED_EMPTY_STR),
    (valid, ZERO_ENDED_VALID, ZERO_ENDED_VALID_STR),
}

create_test_impl_rw_accepted! {
    zero_ended,
    endian: big,
    encoding: utf_8,
    Layout::ZeroEnded,
    (empty, ZERO_ENDED_EMPTY, ZERO_ENDED_EMPTY_STR),
    (valid, ZERO_ENDED_VALID, ZERO_ENDED_VALID_STR),
}
