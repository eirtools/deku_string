//! Input data for accepted tests

use super::layouts::FIXED_LENGTH;

// --------------------------------------------
// ---------- Empty data (EMPTY) --------------
// --------------------------------------------

// FIXED case is covered in PARTIAL

pub const U8_EMPTY_BUF: &[u8; 0] = &[0; 0];

// actual data may not be empty, but it must never appear in the output
pub const STR_EMPTY_DATA: &str = "value";
pub const STR_EMPTY_BUF: &[&str; 0] = &[STR_EMPTY_DATA; 0];

pub const U8_FIXED0_EMPTY_LITTLE_BUF: &[u8; 0] = &[0; 0];
pub const U8_PREFIX_U8_EMPTY_LITTLE_BUF: &[u8; 1] = b"\x00";
pub const U8_PREFIX_U16_EMPTY_LITTLE_BUF: &[u8; 2] = b"\x00\x00";
pub const U8_PREFIX_U32_EMPTY_LITTLE_BUF: &[u8; 4] = b"\x00\x00\x00\x00";
pub const U8_PREFIX_U32_7BIT_EMPTY_LITTLE_BUF: &[u8; 1] = b"\x00";
pub const U8_END_EMPTY_LITTLE_BUF: &[u8; 0] = b"";

pub const STR_FIXED0_EMPTY_LITTLE_BUF: &[u8; 0] = &[0; 0];
pub const STR_PREFIX_U8_EMPTY_LITTLE_BUF: &[u8; 1] = b"\x00";
pub const STR_PREFIX_U16_EMPTY_LITTLE_BUF: &[u8; 2] = b"\x00\x00";
pub const STR_PREFIX_U32_EMPTY_LITTLE_BUF: &[u8; 4] = b"\x00\x00\x00\x00";
pub const STR_PREFIX_U32_7BIT_EMPTY_LITTLE_BUF: &[u8; 1] = b"\x00";
pub const STR_END_EMPTY_LITTLE_BUF: &[u8; 0] = b"";

pub const U8_FIXED0_EMPTY_BIG_BUF: &[u8; 0] = &[0; 0];
pub const U8_PREFIX_U8_EMPTY_BIG_BUF: &[u8; 1] = b"\x00";
pub const U8_PREFIX_U16_EMPTY_BIG_BUF: &[u8; 2] = b"\x00\x00";
pub const U8_PREFIX_U32_EMPTY_BIG_BUF: &[u8; 4] = b"\x00\x00\x00\x00";
pub const U8_PREFIX_U32_7BIT_EMPTY_BIG_BUF: &[u8; 1] = b"\x00";
pub const U8_END_EMPTY_BIG_BUF: &[u8; 0] = b"";

pub const STR_FIXED0_EMPTY_BIG_BUF: &[u8; 0] = &[0; 0];
pub const STR_PREFIX_U8_EMPTY_BIG_BUF: &[u8; 1] = b"\x00";
pub const STR_PREFIX_U16_EMPTY_BIG_BUF: &[u8; 2] = b"\x00\x00";
pub const STR_PREFIX_U32_EMPTY_BIG_BUF: &[u8; 4] = b"\x00\x00\x00\x00";
pub const STR_PREFIX_U32_7BIT_EMPTY_BIG_BUF: &[u8; 1] = b"\x00";
pub const STR_END_EMPTY_BIG_BUF: &[u8; 0] = b"";

// ------------------------------------------
// ---------- Full data (FULL) --------------
// ------------------------------------------

// FIXED0 case is covered in EMPTY

pub const U8_FULL_BUF: &[u8; 3] = b"val";
// Raw bytes count per element
pub const U8_FULL_BUF_LEN: usize = 3;

pub const STR_FULL_VALUE: &str = "value";
pub const STR_FULL_VALUE_LEN: usize = 7;
pub const STR_FULL_BUF: &[&str; FIXED_LENGTH] = &[STR_FULL_VALUE; FIXED_LENGTH];

// ----------- U8 - LITTLE
pub const U8_FIXED_FULL_LITTLE_BUF: &[u8; FIXED_LENGTH] = b"val";
pub const U8_PREFIX_U8_FULL_LITTLE_BUF: &[u8; 1 + U8_FULL_BUF_LEN] = b"\x03val";
pub const U8_PREFIX_U16_FULL_LITTLE_BUF: &[u8; 2 + U8_FULL_BUF_LEN] = b"\x03\x00val";
pub const U8_PREFIX_U32_FULL_LITTLE_BUF: &[u8; 4 + U8_FULL_BUF_LEN] =
    b"\x03\x00\x00\x00val";
pub const U8_PREFIX_U32_7BIT_FULL_LITTLE_BUF: &[u8; 1 + U8_FULL_BUF_LEN] = b"\x03val";
pub const U8_END_FULL_LITTLE_BUF: &[u8; U8_FULL_BUF_LEN] = b"val";

// ----------- STR - LITTLE
pub const STR_FIXED_FULL_LITTLE_BUF: &[u8; FIXED_LENGTH * STR_FULL_VALUE_LEN] =
    b"\x05\x00value\x05\x00value\x05\x00value";
pub const STR_PREFIX_U8_FULL_LITTLE_BUF: &[u8; 1 + FIXED_LENGTH * STR_FULL_VALUE_LEN] =
    b"\x03\x05\x00value\x05\x00value\x05\x00value";
pub const STR_PREFIX_U16_FULL_LITTLE_BUF: &[u8; 2 + FIXED_LENGTH * STR_FULL_VALUE_LEN] =
    b"\x03\x00\x05\x00value\x05\x00value\x05\x00value";
pub const STR_PREFIX_U32_FULL_LITTLE_BUF: &[u8; 4 + FIXED_LENGTH * STR_FULL_VALUE_LEN] =
    b"\x03\x00\x00\x00\x05\x00value\x05\x00value\x05\x00value";
pub const STR_PREFIX_U32_7BIT_FULL_LITTLE_BUF: &[u8; 1 + FIXED_LENGTH
    * STR_FULL_VALUE_LEN] = b"\x03\x05\x00value\x05\x00value\x05\x00value";
pub const STR_END_FULL_LITTLE_BUF: &[u8; FIXED_LENGTH * STR_FULL_VALUE_LEN] =
    b"\x05\x00value\x05\x00value\x05\x00value";

// ----------- U8 - BIG
pub const U8_FIXED_FULL_BIG_BUF: &[u8; FIXED_LENGTH] = b"val";
pub const U8_PREFIX_U8_FULL_BIG_BUF: &[u8; 4] = b"\x03val";
pub const U8_PREFIX_U16_FULL_BIG_BUF: &[u8; 5] = b"\x00\x03val";
pub const U8_PREFIX_U32_FULL_BIG_BUF: &[u8; 7] = b"\x00\x00\x00\x03val";
pub const U8_PREFIX_U32_7BIT_FULL_BIG_BUF: &[u8; 4] = b"\x03val";
pub const U8_END_FULL_BIG_BUF: &[u8; 3] = b"val";

// ----------- STR - BIG
pub const STR_FIXED_FULL_BIG_BUF: &[u8; FIXED_LENGTH * STR_FULL_VALUE_LEN] =
    b"\x00\x05value\x00\x05value\x00\x05value";
pub const STR_PREFIX_U8_FULL_BIG_BUF: &[u8; 1 + FIXED_LENGTH * STR_FULL_VALUE_LEN] =
    b"\x03\x00\x05value\x00\x05value\x00\x05value";
pub const STR_PREFIX_U16_FULL_BIG_BUF: &[u8; 2 + FIXED_LENGTH * STR_FULL_VALUE_LEN] =
    b"\x00\x03\x00\x05value\x00\x05value\x00\x05value";
pub const STR_PREFIX_U32_FULL_BIG_BUF: &[u8; 4 + FIXED_LENGTH * STR_FULL_VALUE_LEN] =
    b"\x00\x00\x00\x03\x00\x05value\x00\x05value\x00\x05value";
pub const STR_PREFIX_U32_7BIT_FULL_BIG_BUF: &[u8; 1 + FIXED_LENGTH
    * STR_FULL_VALUE_LEN] = b"\x03\x00\x05value\x00\x05value\x00\x05value";
pub const STR_END_FULL_BIG_BUF: &[u8; FIXED_LENGTH * STR_FULL_VALUE_LEN] =
    b"\x00\x05value\x00\x05value\x00\x05value";

// ------------------------------------------------
// ------------ Partial data (PARTIAL) ------------
// ------------------------------------------------

pub const U8_PARTIAL_BUF: &[u8; 2] = b"va";
pub const U8_PARTIAL_BUF_OUT: &[u8; 3] = b"va\x00";

// Raw bytes count per element
pub const U8_PARTIAL_BUF_LEN: usize = 2;

pub const STR_PARTIAL_DATA: &str = "value";
pub const STR_PARTIAL_DATA_LEN: usize = 7;

pub const STR_PARTIAL_BUF: &[&str; 2] = &[STR_PARTIAL_DATA, STR_PARTIAL_DATA];
pub const STR_PARTIAL_BUF_OUT: &[&str; FIXED_LENGTH] =
    &[STR_PARTIAL_DATA, STR_PARTIAL_DATA, ""];

// ----------- U8 - LITTLE
pub const U8_FIXED_PARTIAL_LITTLE_BUF: &[u8; FIXED_LENGTH] = b"va\x00";

// ----------- STR - LITTLE
pub const STR_FIXED_PARTIAL_LITTLE_BUF: &[u8; 2 * STR_PARTIAL_DATA_LEN + 2] =
    b"\x05\x00value\x05\x00value\x00\x00";

// ----------- U8 - BIG
pub const U8_FIXED_PARTIAL_BIG_BUF: &[u8; FIXED_LENGTH] = b"va\x00";

// ----------- STR - BIG
pub const STR_FIXED_PARTIAL_BIG_BUF: &[u8; 2 * STR_PARTIAL_DATA_LEN + 2] =
    b"\x00\x05value\x00\x05value\x00\x00";

/// Raw data size
const UTF_8_255_DATA: usize = 255;

pub(crate) const U8_LEN_255_BUF: &[u8; UTF_8_255_DATA] =
    include_bytes!("../../data/data_255");

// ---------- UTF-8: 255 bytes in the buffer (little)
pub(crate) const U8_PREFIX_U8_LEN_255_LITTLE_BUF: &[u8; 1 + UTF_8_255_DATA] =
    include_bytes!("../../data/utf8_u8_little_data_255");

// ---------- UTF-8: 255 bytes in the buffer (big)
pub(crate) const U8_PREFIX_U8_LEN_255_BIG_BUF: &[u8; 1 + UTF_8_255_DATA] =
    include_bytes!("../../data/utf8_u8_big_data_255");
