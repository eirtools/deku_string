//! Input data for accepted tests

use super::layouts::FIXED_LENGTH;

// ------------------------------------
// ---------- Empty data --------------
// ------------------------------------

// ---------- Empty data (input)
pub const U8_EMPTY_BUF_IN: &[u8; 0] = &[0; 0];
pub const STR_EMPTY_BUF_IN: &[&str; 0] = &[""; 0];

// ---------- Empty data (output)
pub const U8_EMPTY_BUF_OUT: &[u8; FIXED_LENGTH] = &[0; FIXED_LENGTH];
pub const STR_EMPTY_BUF_OUT: &[&str; FIXED_LENGTH] = &[""; FIXED_LENGTH];

// ---------- Empty data (VecDeku<u8> little)
pub const U8_FIXED0_LITTLE_EMPTY_BUF: &[u8; 0] = &[0; 0];
pub const U8_FIXED_LITTLE_EMPTY_BUF: &[u8; FIXED_LENGTH] = &[0; FIXED_LENGTH];
pub const U8_PREFIX_U8_LITTLE_EMPTY_BUF: &[u8; 1] = b"\x00";
pub const U8_PREFIX_U16_LITTLE_EMPTY_BUF: &[u8; 2] = b"\x00\x00";
pub const U8_PREFIX_U32_LITTLE_EMPTY_BUF: &[u8; 4] = b"\x00\x00\x00\x00";
pub const U8_PREFIX_U32_7BIT_LITTLE_EMPTY_BUF: &[u8; 1] = b"\x00";
pub const U8_END_LITTLE_EMPTY_BUF: &[u8; 0] = b"";

// ---------- Empty data (VecDeku<StringDeku> little)
pub const STR_FIXED0_LITTLE_EMPTY_BUF: &[u8; 0] = &[0; 0];
pub const STR_FIXED_LITTLE_EMPTY_BUF: &[u8; FIXED_LENGTH * 2] = &[0; FIXED_LENGTH * 2];
pub const STR_PREFIX_U8_LITTLE_EMPTY_BUF: &[u8; 1] = b"\x00";
pub const STR_PREFIX_U16_LITTLE_EMPTY_BUF: &[u8; 2] = b"\x00\x00";
pub const STR_PREFIX_U32_LITTLE_EMPTY_BUF: &[u8; 4] = b"\x00\x00\x00\x00";
pub const STR_PREFIX_U32_7BIT_LITTLE_EMPTY_BUF: &[u8; 1] = b"\x00";
pub const STR_END_LITTLE_EMPTY_BUF: &[u8; 0] = b"";

// ---------- Empty data (VecDeku<u8> big)
pub const U8_FIXED0_BIG_EMPTY_BUF: &[u8; 0] = &[0; 0];
pub const U8_FIXED_BIG_EMPTY_BUF: &[u8; FIXED_LENGTH] = &[0; FIXED_LENGTH];
pub const U8_PREFIX_U8_BIG_EMPTY_BUF: &[u8; 1] = b"\x00";
pub const U8_PREFIX_U16_BIG_EMPTY_BUF: &[u8; 2] = b"\x00\x00";
pub const U8_PREFIX_U32_BIG_EMPTY_BUF: &[u8; 4] = b"\x00\x00\x00\x00";
pub const U8_PREFIX_U32_7BIT_BIG_EMPTY_BUF: &[u8; 1] = b"\x00";
pub const U8_END_BIG_EMPTY_BUF: &[u8; 0] = b"";

// ---------- Empty data (VecDeku<StringDeku> big)
pub const STR_FIXED0_BIG_EMPTY_BUF: &[u8; 0] = &[0; 0];
pub const STR_FIXED_BIG_EMPTY_BUF: &[u8; FIXED_LENGTH * 2] = &[0; FIXED_LENGTH * 2];
pub const STR_PREFIX_U8_BIG_EMPTY_BUF: &[u8; 1] = b"\x00";
pub const STR_PREFIX_U16_BIG_EMPTY_BUF: &[u8; 2] = b"\x00\x00";
pub const STR_PREFIX_U32_BIG_EMPTY_BUF: &[u8; 4] = b"\x00\x00\x00\x00";
pub const STR_PREFIX_U32_7BIT_BIG_EMPTY_BUF: &[u8; 1] = b"\x00";
pub const STR_END_BIG_EMPTY_BUF: &[u8; 0] = b"";

// -------------%<-------------%<------|------->%------------->%-------------

// ------------------------------------
// ---------- No padding --------------
// ------------------------------------
//
// * `VecLayout::FixedLength(0)` — data is too big

// ---------- No padding (additional consts)
pub const U8_NO_PADDING_BUF_LEN: usize = 3;
pub const STR_NO_PADDING_VALUE: &str = "value";
pub const STR_ENCODED_VALUE_NO_PADDING_LEN: usize = 7;

// ---------- No padding (input)
pub const U8_NO_PADDING_BUF_IN: &[u8; U8_NO_PADDING_BUF_LEN] = b"val";
pub const STR_NO_PADDING_BUF_IN: &[&str; FIXED_LENGTH] =
    &[STR_NO_PADDING_VALUE; FIXED_LENGTH];

// ---------- No padding (VecDeku<u8> little)
pub const U8_FIXED_LITTLE_NO_PADDING_BUF: &[u8; FIXED_LENGTH] = b"val";
pub const U8_PREFIX_U8_LITTLE_NO_PADDING_BUF: &[u8; 1 + U8_NO_PADDING_BUF_LEN] =
    b"\x03val";
pub const U8_PREFIX_U16_LITTLE_NO_PADDING_BUF: &[u8; 2 + U8_NO_PADDING_BUF_LEN] =
    b"\x03\x00val";
pub const U8_PREFIX_U32_LITTLE_NO_PADDING_BUF: &[u8; 4 + U8_NO_PADDING_BUF_LEN] =
    b"\x03\x00\x00\x00val";
pub const U8_PREFIX_U32_7BIT_LITTLE_NO_PADDING_BUF: &[u8; 1 + U8_NO_PADDING_BUF_LEN] =
    b"\x03val";
pub const U8_END_LITTLE_NO_PADDING_BUF: &[u8; U8_NO_PADDING_BUF_LEN] = b"val";

// ---------- No padding (VecDeku<StringDeku> little)
pub const STR_FIXED_LITTLE_NO_PADDING_BUF: &[u8; FIXED_LENGTH
     * STR_ENCODED_VALUE_NO_PADDING_LEN] = b"\x05\x00value\x05\x00value\x05\x00value";
pub const STR_PREFIX_U8_LITTLE_NO_PADDING_BUF: &[u8; 1 + FIXED_LENGTH
    * STR_ENCODED_VALUE_NO_PADDING_LEN] =
    b"\x03\x05\x00value\x05\x00value\x05\x00value";
pub const STR_PREFIX_U16_LITTLE_NO_PADDING_BUF:
    &[u8; 2 + FIXED_LENGTH * STR_ENCODED_VALUE_NO_PADDING_LEN] =
    b"\x03\x00\x05\x00value\x05\x00value\x05\x00value";
pub const STR_PREFIX_U32_LITTLE_NO_PADDING_BUF:
    &[u8; 4 + FIXED_LENGTH * STR_ENCODED_VALUE_NO_PADDING_LEN] =
    b"\x03\x00\x00\x00\x05\x00value\x05\x00value\x05\x00value";
pub const STR_PREFIX_U32_7BIT_LITTLE_NO_PADDING_BUF:
    &[u8; 1 + FIXED_LENGTH * STR_ENCODED_VALUE_NO_PADDING_LEN] =
    b"\x03\x05\x00value\x05\x00value\x05\x00value";
pub const STR_END_LITTLE_NO_PADDING_BUF: &[u8; FIXED_LENGTH
     * STR_ENCODED_VALUE_NO_PADDING_LEN] = b"\x05\x00value\x05\x00value\x05\x00value";

// ---------- No padding (VecDeku<u8> big)
pub const U8_FIXED_BIG_NO_PADDING_BUF: &[u8; FIXED_LENGTH] = b"val";
pub const U8_PREFIX_U8_BIG_NO_PADDING_BUF: &[u8; 4] = b"\x03val";
pub const U8_PREFIX_U16_BIG_NO_PADDING_BUF: &[u8; 5] = b"\x00\x03val";
pub const U8_PREFIX_U32_BIG_NO_PADDING_BUF: &[u8; 7] = b"\x00\x00\x00\x03val";
pub const U8_PREFIX_U32_7BIT_BIG_NO_PADDING_BUF: &[u8; 4] = b"\x03val";
pub const U8_END_BIG_NO_PADDING_BUF: &[u8; 3] = b"val";

// ---------- No padding (VecDeku<StringDeku> big)
pub const STR_FIXED_BIG_NO_PADDING_BUF: &[u8; FIXED_LENGTH
     * STR_ENCODED_VALUE_NO_PADDING_LEN] = b"\x00\x05value\x00\x05value\x00\x05value";
pub const STR_PREFIX_U8_BIG_NO_PADDING_BUF: &[u8; 1 + FIXED_LENGTH
    * STR_ENCODED_VALUE_NO_PADDING_LEN] =
    b"\x03\x00\x05value\x00\x05value\x00\x05value";
pub const STR_PREFIX_U16_BIG_NO_PADDING_BUF: &[u8; 2 + FIXED_LENGTH
    * STR_ENCODED_VALUE_NO_PADDING_LEN] =
    b"\x00\x03\x00\x05value\x00\x05value\x00\x05value";
pub const STR_PREFIX_U32_BIG_NO_PADDING_BUF: &[u8; 4 + FIXED_LENGTH
    * STR_ENCODED_VALUE_NO_PADDING_LEN] =
    b"\x00\x00\x00\x03\x00\x05value\x00\x05value\x00\x05value";
pub const STR_PREFIX_U32_7BIT_BIG_NO_PADDING_BUF:
    &[u8; 1 + FIXED_LENGTH * STR_ENCODED_VALUE_NO_PADDING_LEN] =
    b"\x03\x00\x05value\x00\x05value\x00\x05value";
pub const STR_END_BIG_NO_PADDING_BUF: &[u8; FIXED_LENGTH
     * STR_ENCODED_VALUE_NO_PADDING_LEN] = b"\x00\x05value\x00\x05value\x00\x05value";

// -------------%<-------------%<------|------->%------------->%-------------

// -------------------------------------
// ------------ Padded data ------------
// -------------------------------------
//
// * `VecLayout::LengthPrefix` — no padding
// * `VecLayout::End` — no padding

// ---------- Padded data (additional consts)
pub const U8_PADDED_DATA_BUF_LEN: usize = 2;
pub const STR_PADDED_DATA_DATA: &str = "value";
pub const STR_PADDED_DATA_DATA_LEN: usize = 7;

// ---------- Padded data (input)
pub const U8_PADDED_DATA_BUF_IN: &[u8; U8_PADDED_DATA_BUF_LEN] = b"va";
pub const STR_PADDED_DATA_BUF_IN: &[&str; 2] =
    &[STR_PADDED_DATA_DATA, STR_PADDED_DATA_DATA];

// ---------- Padded data (output)
pub const U8_PADDED_DATA_BUF_OUT: &[u8; 3] = b"va\x00";
pub const STR_PADDED_DATA_BUF_OUT: &[&str; FIXED_LENGTH] =
    &[STR_PADDED_DATA_DATA, STR_PADDED_DATA_DATA, ""];

// ---------- Padded data (VecDeku<u8> little)
pub const U8_FIXED_LITTLE_PADDED_DATA_BUF: &[u8; FIXED_LENGTH] = b"va\x00";

// ---------- Padded data (VecDeku<StringDeku> little)
pub const STR_FIXED_LITTLE_PADDED_DATA_BUF: &[u8; 2 * STR_PADDED_DATA_DATA_LEN + 2] =
    b"\x05\x00value\x05\x00value\x00\x00";

// ---------- Padded data (VecDeku<u8> big)
pub const U8_FIXED_BIG_PADDED_DATA_BUF: &[u8; FIXED_LENGTH] = b"va\x00";

// ---------- Padded data (VecDeku<StringDeku> big)
pub const STR_FIXED_BIG_PADDED_DATA_BUF: &[u8; 2 * STR_PADDED_DATA_DATA_LEN + 2] =
    b"\x00\x05value\x00\x05value\x00\x00";

// -------------%<-------------%<------|------->%------------->%-------------

// ------------------------------------
// ------------ Length 255 ------------
// ------------------------------------
//
// Same as valid case but algorithm boundary for `VecLayout::PrefixLength(Size::U8)`
//
// * `VecLayout::FixedLength` — too big data.
// * `VecLayout::End` — same as "valid" case.
// * `VecLayout::PrefixLength` (not u8) — same as "valid" case.

// ---------- Length 255 (additional consts)
const UTF_8_255_DATA: usize = 255;

// ---------- Length 255 (input)
pub(crate) const U8_LEN_255_BUF_IN: &[u8; UTF_8_255_DATA] =
    include_bytes!("../../data/data_255");

// ---------- Length 255 (VecDeku<u8> little)
pub(crate) const U8_PREFIX_U8_LITTLE_LEN_255_BUF: &[u8; 1 + UTF_8_255_DATA] =
    include_bytes!("../../data/utf8_u8_little_data_255");

// ---------- Length 255 (VecDeku<u8> big)
pub(crate) const U8_PREFIX_U8_BIG_LEN_255_BUF: &[u8; 1 + UTF_8_255_DATA] =
    include_bytes!("../../data/utf8_u8_big_data_255");

// -------------%<-------------%<------|------->%------------->%-------------
