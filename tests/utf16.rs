//!
//! Accepted read and write tests for UTF-16 strings
//!
mod test_gen_macro;
use pastey::paste;
use rstest::rstest;

use deku::ctx::Endian;
use deku::reader::Reader;
use deku::writer::Writer;
use deku::{no_std_io, DekuReader as _, DekuWriter as _};

use deku_string::{Encoding, Layout, Size, StringDeku};

/// constraint for reader
const FIXED_LENGTH: usize = 23;
/// Real data size
const FIXED_LENGTH_DATA: usize = FIXED_LENGTH * 2;

const FIXED_ZERO_INSIDE_STR: &str = "valid test case w zero";
const FIXED_LITTLE_ZERO_INSIDE: &[u8; FIXED_LENGTH_DATA] = b"v\x00a\x00l\x00i\x00d\x00 \x00t\x00e\x00s\x00t\x00 \x00c\x00a\x00s\x00e\x00 \x00w\x00 \x00z\x00e\x00r\x00o\x00\x00\x00";
const FIXED_BIG_ZERO_INSIDE: &[u8; FIXED_LENGTH_DATA] = b"\x00v\x00a\x00l\x00i\x00d\x00 \x00t\x00e\x00s\x00t\x00 \x00c\x00a\x00s\x00e\x00 \x00w\x00 \x00z\x00e\x00r\x00o\x00\x00";

const FIXED_ZERO_MIDDLE_STR: &str = "valid test";
const FIXED_LITTLE_ZERO_MIDDLE: &[u8; FIXED_LENGTH_DATA] = b"v\x00a\x00l\x00i\x00d\x00 \x00t\x00e\x00s\x00t\x00\x00\x00w\x00i\x00t\x00h\x00 \x00g\x00a\x00r\x00b\x00a\x00g\x00e\x00";
const FIXED_BIG_ZERO_MIDDLE: &[u8; FIXED_LENGTH_DATA] = b"\x00v\x00a\x00l\x00i\x00d\x00 \x00t\x00e\x00s\x00t\x00\x00\x00w\x00i\x00t\x00h\x00 \x00g\x00a\x00r\x00b\x00a\x00g\x00e";

const FIXED_LITTLE_ZERO_MIDDLE_WRITE: &[u8; FIXED_LENGTH_DATA] =
    b"v\x00a\x00l\x00i\x00d\x00 \x00t\x00e\x00s\x00t\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";
const FIXED_BIG_ZERO_MIDDLE_WRITE: &[u8; FIXED_LENGTH_DATA] =
    b"\x00v\x00a\x00l\x00i\x00d\x00 \x00t\x00e\x00s\x00t\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";

const FIXED_NO_ZERO_INSIDE_STR: &str = "valid test case no zero";
const FIXED_LITTLE_NO_ZERO_INSIDE: &[u8; FIXED_LENGTH_DATA] = b"v\x00a\x00l\x00i\x00d\x00 \x00t\x00e\x00s\x00t\x00 \x00c\x00a\x00s\x00e\x00 \x00n\x00o\x00 \x00z\x00e\x00r\x00o\x00";
const FIXED_BIG_NO_ZERO_INSIDE: &[u8; FIXED_LENGTH_DATA] = b"\x00v\x00a\x00l\x00i\x00d\x00 \x00t\x00e\x00s\x00t\x00 \x00c\x00a\x00s\x00e\x00 \x00n\x00o\x00 \x00z\x00e\x00r\x00o";

const PREFIX_U8_EMPTY_STR: &str = "";
const PREFIX_U8_EMPTY: &[u8; 1] = b"\x00";

const PREFIX_U8_VALID_STR: &str = "valid test case";
const PREFIX_U8_LITTLE_VALID: &[u8; 31] =
    b"\x0Fv\x00a\x00l\x00i\x00d\x00 \x00t\x00e\x00s\x00t\x00 \x00c\x00a\x00s\x00e\x00";
const PREFIX_U8_BIG_VALID: &[u8; 31] =
    b"\x0F\x00v\x00a\x00l\x00i\x00d\x00 \x00t\x00e\x00s\x00t\x00 \x00c\x00a\x00s\x00e";

const PREFIX_U16_LITTLE_EMPTY_STR: &str = "";
const PREFIX_U16_LITTLE_EMPTY: &[u8; 2] = b"\x00\x00";

const PREFIX_U16_LITTLE_VALID_STR: &str = "valid test case";
const PREFIX_U16_LITTLE_VALID: &[u8; 32] =
    b"\x0F\x00v\x00a\x00l\x00i\x00d\x00 \x00t\x00e\x00s\x00t\x00 \x00c\x00a\x00s\x00e\x00";

const PREFIX_U32_LITTLE_EMPTY_STR: &str = "";
const PREFIX_U32_LITTLE_EMPTY: &[u8; 4] = b"\x00\x00\x00\x00";

const PREFIX_U32_LITTLE_VALID_STR: &str = "valid test case";
const PREFIX_U32_LITTLE_VALID: &[u8; 34] =
    b"\x0F\x00\x00\x00v\x00a\x00l\x00i\x00d\x00 \x00t\x00e\x00s\x00t\x00 \x00c\x00a\x00s\x00e\x00";

const PREFIX_U16_BIG_EMPTY_STR: &str = "";
const PREFIX_U16_BIG_EMPTY: &[u8; 2] = b"\x00\x00";

const PREFIX_U16_BIG_VALID_STR: &str = "valid test case";
const PREFIX_U16_BIG_VALID: &[u8; 32] =
    b"\x00\x0F\x00v\x00a\x00l\x00i\x00d\x00 \x00t\x00e\x00s\x00t\x00 \x00c\x00a\x00s\x00e";

const PREFIX_U32_BIG_EMPTY_STR: &str = "";
const PREFIX_U32_BIG_EMPTY: &[u8; 4] = b"\x00\x00\x00\x00";

const PREFIX_U32_BIG_VALID_STR: &str = "valid test case";
const PREFIX_U32_BIG_VALID: &[u8; 34] =
    b"\x00\x00\x00\x0F\x00v\x00a\x00l\x00i\x00d\x00 \x00t\x00e\x00s\x00t\x00 \x00c\x00a\x00s\x00e";

const ZERO_ENDED_EMPTY_STR: &str = "";
const ZERO_ENDED_EMPTY: &[u8; 2] = b"\x00\x00";

const ZERO_ENDED_VALID_STR: &str = "valid test case";
const ZERO_ENDED_LITTLE_VALID: &[u8; 32] =
    b"v\x00a\x00l\x00i\x00d\x00 \x00t\x00e\x00s\x00t\x00 \x00c\x00a\x00s\x00e\x00\x00\x00";
const ZERO_ENDED_BIG_VALID: &[u8; 32] =
    b"\x00v\x00a\x00l\x00i\x00d\x00 \x00t\x00e\x00s\x00t\x00 \x00c\x00a\x00s\x00e\x00\x00";

create_test_impl_rw_accepted! {
    fixed_force_zero,
    endian: little,
    encoding: utf_16,
    Layout::FixedLength {
        size: FIXED_LENGTH ,
        allow_no_null:false,
    },
    (zero_at_end, FIXED_LITTLE_ZERO_INSIDE, FIXED_ZERO_INSIDE_STR, FIXED_LITTLE_ZERO_INSIDE),
    (zero_in_middle, FIXED_LITTLE_ZERO_MIDDLE, FIXED_ZERO_MIDDLE_STR, FIXED_LITTLE_ZERO_MIDDLE_WRITE)
}

create_test_impl_rw_accepted! {
    fixed_force_zero,
    endian: big,
    encoding: utf_16,
    Layout::FixedLength {
        size: FIXED_LENGTH,
        allow_no_null:false,
    },
    (zero_at_end, FIXED_BIG_ZERO_INSIDE, FIXED_ZERO_INSIDE_STR, FIXED_BIG_ZERO_INSIDE),
    (zero_in_middle, FIXED_BIG_ZERO_MIDDLE, FIXED_ZERO_MIDDLE_STR, FIXED_BIG_ZERO_MIDDLE_WRITE)
}

create_test_impl_rw_accepted! {
    fixed_allow_no_zero,
    endian: little,
    encoding: utf_16,
    Layout::FixedLength {
        size: FIXED_LENGTH,
        allow_no_null:true,
    },
    (zero_at_end, FIXED_LITTLE_ZERO_INSIDE, FIXED_ZERO_INSIDE_STR, FIXED_LITTLE_ZERO_INSIDE),
    (zero_in_middle, FIXED_LITTLE_ZERO_MIDDLE, FIXED_ZERO_MIDDLE_STR, FIXED_LITTLE_ZERO_MIDDLE_WRITE),
    (not_zero_ended, FIXED_LITTLE_NO_ZERO_INSIDE, FIXED_NO_ZERO_INSIDE_STR, FIXED_LITTLE_NO_ZERO_INSIDE),
}

create_test_impl_rw_accepted! {
    fixed_allow_no_zero,
    endian: big,
    encoding: utf_16,
    Layout::FixedLength {
        size: FIXED_LENGTH,
        allow_no_null:true,
    },
    (zero_at_end, FIXED_BIG_ZERO_INSIDE, FIXED_ZERO_INSIDE_STR, FIXED_BIG_ZERO_INSIDE),
    (zero_in_middle, FIXED_BIG_ZERO_MIDDLE, FIXED_ZERO_MIDDLE_STR, FIXED_BIG_ZERO_MIDDLE_WRITE),
    (not_zero_ended, FIXED_BIG_NO_ZERO_INSIDE, FIXED_NO_ZERO_INSIDE_STR, FIXED_BIG_NO_ZERO_INSIDE),
}

create_test_impl_rw_accepted! {
    prefix_u8,
    endian: little,
    encoding: utf_16,
    Layout::LengthPrefix(Size::U8),
    (empty, PREFIX_U8_EMPTY, PREFIX_U8_EMPTY_STR),
    (valid, PREFIX_U8_LITTLE_VALID, PREFIX_U8_VALID_STR),
}

create_test_impl_rw_accepted! {
    prefix_u8,
    endian: big,
    encoding: utf_16,
    Layout::LengthPrefix(Size::U8),
    (empty, PREFIX_U8_EMPTY, PREFIX_U8_EMPTY_STR),
    (valid, PREFIX_U8_BIG_VALID, PREFIX_U8_VALID_STR),
}

create_test_impl_rw_accepted! {
    prefix_u16,
    endian: little,
    encoding: utf_16,
    Layout::LengthPrefix(Size::U16),
    (empty, PREFIX_U16_LITTLE_EMPTY, PREFIX_U16_LITTLE_EMPTY_STR),
    (valid, PREFIX_U16_LITTLE_VALID, PREFIX_U16_LITTLE_VALID_STR),
}

create_test_impl_rw_accepted! {
    prefix_u16,
    endian: big,
    encoding: utf_16,
    Layout::LengthPrefix(Size::U16),
    (empty, PREFIX_U16_BIG_EMPTY, PREFIX_U16_BIG_EMPTY_STR),
    (valid, PREFIX_U16_BIG_VALID, PREFIX_U16_BIG_VALID_STR),
}

create_test_impl_rw_accepted! {
    prefix_u32,
    endian: little,
    encoding: utf_16,
    Layout::LengthPrefix(Size::U32),
    (empty, PREFIX_U32_LITTLE_EMPTY, PREFIX_U32_LITTLE_EMPTY_STR),
    (valid, PREFIX_U32_LITTLE_VALID, PREFIX_U32_LITTLE_VALID_STR),
}

create_test_impl_rw_accepted! {
    prefix_u32,
    endian: big,
    encoding: utf_16,
    Layout::LengthPrefix(Size::U32),
    (empty, PREFIX_U32_BIG_EMPTY, PREFIX_U32_BIG_EMPTY_STR),
    (valid, PREFIX_U32_BIG_VALID, PREFIX_U32_BIG_VALID_STR),
}

create_test_impl_rw_accepted! {
    zero_ended,
    endian: little,
    encoding: utf_16,
    Layout::ZeroEnded,
    (empty, ZERO_ENDED_EMPTY, ZERO_ENDED_EMPTY_STR),
    (valid, ZERO_ENDED_LITTLE_VALID, ZERO_ENDED_VALID_STR),
}

create_test_impl_rw_accepted! {
    zero_ended,
    endian: big,
    encoding: utf_16,
    Layout::ZeroEnded,
    (empty, ZERO_ENDED_EMPTY, ZERO_ENDED_EMPTY_STR),
    (valid, ZERO_ENDED_BIG_VALID, ZERO_ENDED_VALID_STR),
}
