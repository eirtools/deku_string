//!
//! Rejected read tests for UTF-8 strings
//!
mod layouts;
mod test_gen_macro;

use pastey::paste;
use rstest::rstest;

use deku::ctx::Endian;
use deku::reader::Reader;
use deku::DekuReader as _;

use deku_string::{Encoding, StringDeku};
use layouts::*;

const SMALL_BUFFER: &[u8; 0] = &[];

const FIXED_INVALID_UTF: &[u8; FIXED_LENGTH] = b"invalid test case  \xe2\x28\xa1\x00";

const FIXED_NO_ZERO_INSIDE: &[u8; FIXED_LENGTH] = b"invalid test case zero?";

const PREFIX_U8_ZERO_IN_MIDDLE: &[u8; 3] = b"\x02\x00a";
const PREFIX_U8_INVALID_UTF: &[u8; 8] = b"\x07utf-\xe2\x28\xa1";

const PREFIX_U8_SHORT_LEN: &[u8; 1] = b"\x01";
const PREFIX_U8_SHORT_DATA: &[u8; 2] = b"\x02a";

const PREFIX_U16_LITTLE_ZERO_IN_MIDDLE: &[u8; 4] = b"\x02\x00\x00a";
const PREFIX_U16_BIG_ZERO_IN_MIDDLE: &[u8; 4] = b"\x00\x02\x00a";

const PREFIX_U16_LITTLE_INVALID_UTF: &[u8; 9] = b"\x07\x00utf-\xe2\x28\xa1";
const PREFIX_U16_BIG_INVALID_UTF: &[u8; 9] = b"\x00\x07utf-\xe2\x28\xa1";

const PREFIX_U16_LITTLE_SHORT_LEN: &[u8; 2] = b"\x01\x00";
const PREFIX_U16_BIG_SHORT_LEN: &[u8; 2] = b"\x01\x00";

const PREFIX_U16_LITTLE_SHORT_DATA: &[u8; 3] = b"\x02\x00a";
const PREFIX_U16_BIG_SHORT_DATA: &[u8; 3] = b"\x02\x00a";

const PREFIX_U32_LITTLE_ZERO_IN_MIDDLE: &[u8; 6] = b"\x02\x00\x00\x00\x00a";
const PREFIX_U32_BIG_ZERO_IN_MIDDLE: &[u8; 6] = b"\x00\x00\x00\x02\x00a";

const PREFIX_U32_LITTLE_INVALID_UTF: &[u8; 11] = b"\x07\x00\x00\x00utf-\xe2\x28\xa1";
const PREFIX_U32_BIG_INVALID_UTF: &[u8; 11] = b"\x00\x00\x00\x07utf-\xe2\x28\xa1";

const PREFIX_U32_LITTLE_SHORT_LEN: &[u8; 4] = b"\x01\x00\x00\x00";
const PREFIX_U32_BIG_SHORT_LEN: &[u8; 4] = b"\x01\x00\x00\x00";

const PREFIX_U32_LITTLE_SHORT_DATA: &[u8; 5] = b"\x02\x00\x00\x00a";
const PREFIX_U32_BIG_SHORT_DATA: &[u8; 5] = b"\x02\x00\x00\x00a";

const ZERO_ENDED_INVALID: &[u8; 17] = b"invalid test case";

create_test_impl_read_rejected! {
    fixed_force_zero,
    endian: little,
    encoding: utf_8,
    LAYOUT_FIXED_FORCE_ZERO,
    deku::DekuError::Assertion(_),
    (not_zero_ended, FIXED_NO_ZERO_INSIDE),
}

create_test_impl_read_rejected! {
    fixed_force_zero_small_buf,
    endian: little,
    encoding: utf_8,
    LAYOUT_FIXED_FORCE_ZERO,
    deku::DekuError::Incomplete(deku::error::NeedSize {..}),
    (small_buffer, SMALL_BUFFER),
}

create_test_impl_read_rejected! {
    fixed_force_zero_utf_parse,
    endian: little,
    encoding: utf_8,
    LAYOUT_FIXED_FORCE_ZERO,
    deku::DekuError::Parse(_),
    (invalid_utf, FIXED_INVALID_UTF),
}

create_test_impl_read_rejected! {
    fixed_allow_zero_utf_parse,
    endian: little,
    encoding: utf_8,
    LAYOUT_FIXED_ALLOW_NO_ZERO,
    deku::DekuError::Parse(_),
    (invalid_utf, FIXED_INVALID_UTF),
}

create_test_impl_read_rejected! {
    fixed_force_zero,
    endian: big,
    encoding: utf_8,
    LAYOUT_FIXED_FORCE_ZERO,
    deku::DekuError::Assertion(_),
    (not_zero_ended, FIXED_NO_ZERO_INSIDE),
}

create_test_impl_read_rejected! {
    fixed_force_zero_small_buf,
    endian: big,
    encoding: utf_8,
    LAYOUT_FIXED_FORCE_ZERO,
    deku::DekuError::Incomplete(deku::error::NeedSize {..}),
    (small_buffer, SMALL_BUFFER),
}

create_test_impl_read_rejected! {
    prefix_u8,
    endian: little,
    encoding: utf_8,
    LAYOUT_PREFIX_U8,
    deku::DekuError::Incomplete(deku::error::NeedSize {..}),
    (small_buffer, SMALL_BUFFER),
    (short_len, PREFIX_U8_SHORT_LEN),
    (short_data, PREFIX_U8_SHORT_DATA),
}

create_test_impl_read_rejected! {
    prefix_u8,
    endian: big,
    encoding: utf_8,
    LAYOUT_PREFIX_U8,
    deku::DekuError::Incomplete(deku::error::NeedSize {..}),
    (small_buffer, SMALL_BUFFER),
    (short_len, PREFIX_U8_SHORT_LEN),
    (short_data, PREFIX_U8_SHORT_DATA),
}

create_test_impl_read_rejected! {
    prefix_u8_zero,
    endian: little,
    encoding: utf_8,
    LAYOUT_PREFIX_U8,
    deku::DekuError::Assertion(_),
    (zero_in_middle, PREFIX_U8_ZERO_IN_MIDDLE),
}

create_test_impl_read_rejected! {
    prefix_u8_zero,
    endian: big,
    encoding: utf_8,
    LAYOUT_PREFIX_U8,
    deku::DekuError::Assertion(_),
    (zero_in_middle, PREFIX_U8_ZERO_IN_MIDDLE),
}

create_test_impl_read_rejected! {
    prefix_u8_utf_parse,
    endian: little,
    encoding: utf_8,
    LAYOUT_PREFIX_U8,
    deku::DekuError::Parse(_),
    (invalid_utf, PREFIX_U8_INVALID_UTF),
}

create_test_impl_read_rejected! {
    prefix_u8_utf_parse,
    endian: big,
    encoding: utf_8,
    LAYOUT_PREFIX_U8,
    deku::DekuError::Parse(_),
    (invalid_utf, PREFIX_U8_INVALID_UTF),
}

create_test_impl_read_rejected! {
    prefix_u16,
    endian: little,
    encoding: utf_8,
    LAYOUT_PREFIX_U16,
    deku::DekuError::Incomplete(deku::error::NeedSize {..}),
    (small_buffer, SMALL_BUFFER),
    (short_len, PREFIX_U16_LITTLE_SHORT_LEN),
    (short_data, PREFIX_U16_LITTLE_SHORT_DATA),
}

create_test_impl_read_rejected! {
    prefix_u16,
    endian: big,
    encoding: utf_8,
    LAYOUT_PREFIX_U16,
    deku::DekuError::Incomplete(deku::error::NeedSize {..}),
    (small_buffer, SMALL_BUFFER),
    (short_len, PREFIX_U16_BIG_SHORT_LEN),
    (short_data, PREFIX_U16_BIG_SHORT_DATA),
}

create_test_impl_read_rejected! {
    prefix_u16_zero,
    endian: little,
    encoding: utf_8,
    LAYOUT_PREFIX_U16,
    deku::DekuError::Assertion(_),
    (zero_in_middle, PREFIX_U16_LITTLE_ZERO_IN_MIDDLE),
}

create_test_impl_read_rejected! {
    prefix_u16_zero,
    endian: big,
    encoding: utf_8,
    LAYOUT_PREFIX_U16,
    deku::DekuError::Assertion(_),
    (zero_in_middle, PREFIX_U16_BIG_ZERO_IN_MIDDLE),
}

create_test_impl_read_rejected! {
    prefix_u16_utf_parse,
    endian: little,
    encoding: utf_8,
    LAYOUT_PREFIX_U16,
    deku::DekuError::Parse(_),
    (invalid_utf, PREFIX_U16_LITTLE_INVALID_UTF),
}

create_test_impl_read_rejected! {
    prefix_u16_utf_parse,
    endian: big,
    encoding: utf_8,
    LAYOUT_PREFIX_U16,
    deku::DekuError::Parse(_),
    (invalid_utf, PREFIX_U16_BIG_INVALID_UTF),
}

create_test_impl_read_rejected! {
    prefix_u32,
    endian: little,
    encoding: utf_8,
    LAYOUT_PREFIX_U32,
    deku::DekuError::Incomplete(deku::error::NeedSize {..}),
    (small_buffer, SMALL_BUFFER),
    (short_len, PREFIX_U32_LITTLE_SHORT_LEN),
    (short_data, PREFIX_U32_LITTLE_SHORT_DATA),
}

create_test_impl_read_rejected! {
    prefix_u32,
    endian: big,
    encoding: utf_8,
    LAYOUT_PREFIX_U32,
    deku::DekuError::Incomplete(deku::error::NeedSize {..}),
    (small_buffer, SMALL_BUFFER),
    (short_len, PREFIX_U32_BIG_SHORT_LEN),
    (short_data, PREFIX_U32_BIG_SHORT_DATA),
}

create_test_impl_read_rejected! {
    prefix_u32_zero,
    endian: little,
    encoding: utf_8,
    LAYOUT_PREFIX_U32,
    deku::DekuError::Assertion(_),
    (zero_in_middle, PREFIX_U32_LITTLE_ZERO_IN_MIDDLE),
}

create_test_impl_read_rejected! {
    prefix_u32_zero,
    endian: big,
    encoding: utf_8,
    LAYOUT_PREFIX_U32,
    deku::DekuError::Assertion(_),
    (zero_in_middle, PREFIX_U32_BIG_ZERO_IN_MIDDLE),
}

create_test_impl_read_rejected! {
    prefix_u32_utf_parse,
    endian: little,
    encoding: utf_8,
    LAYOUT_PREFIX_U32,
    deku::DekuError::Parse(_),
    (invalid_utf, PREFIX_U32_LITTLE_INVALID_UTF),
}

create_test_impl_read_rejected! {
    prefix_u32_utf_parse,
    endian: big,
    encoding: utf_8,
    LAYOUT_PREFIX_U32,
    deku::DekuError::Parse(_),
    (invalid_utf, PREFIX_U32_BIG_INVALID_UTF),
}

create_test_impl_read_rejected! {
    zero_ended,
    endian: little,
    encoding: utf_8,
    LAYOUT_ZERO_ENDED,
    deku::DekuError::Incomplete(deku::error::NeedSize {..}),
    (small_buffer, SMALL_BUFFER),
    (not_zero_ended, ZERO_ENDED_INVALID),
}

create_test_impl_read_rejected! {
    zero_ended,
    endian: big,
    encoding: utf_8,
    LAYOUT_ZERO_ENDED,
    deku::DekuError::Incomplete(deku::error::NeedSize {..}),
    (small_buffer, SMALL_BUFFER),
    (not_zero_ended, ZERO_ENDED_INVALID),
}
