#![allow(dead_code)]
use super::layouts::FIXED_LENGTH;

/// Raw data size
const UTF_8_FIXED_LENGTH_DATA: usize = FIXED_LENGTH;
/// Raw data size
const UTF_16_FIXED_LENGTH_DATA: usize = FIXED_LENGTH * 2;

// ------------------------------------
// ---------- Empty data --------------
// ------------------------------------

// ---------- Empty data (str)
pub(crate) const FIXED_FORCE_ZERO_EMPTY_STR: &str = "";
pub(crate) const FIXED_ALLOW_NO_ZERO_EMPTY_STR: &str = "";
pub(crate) const PREFIX_U8_EMPTY_STR: &str = "";
pub(crate) const PREFIX_U16_EMPTY_STR: &str = "";
pub(crate) const PREFIX_U32_EMPTY_STR: &str = "";
pub(crate) const ZERO_ENDED_EMPTY_STR: &str = "";

// ---------- UTF-8: Empty data (little)
pub(crate) const UTF_8_FIXED_FORCE_ZERO_LITTLE_EMPTY_IN: &[u8; UTF_8_FIXED_LENGTH_DATA] =
    &[0; UTF_8_FIXED_LENGTH_DATA];
pub(crate) const UTF_8_FIXED_ALLOW_NO_ZERO_LITTLE_EMPTY_IN: &[u8; UTF_8_FIXED_LENGTH_DATA] =
    &[0; UTF_8_FIXED_LENGTH_DATA];
pub(crate) const UTF_8_PREFIX_U8_LITTLE_EMPTY_IN: &[u8; 1] = b"\x00";
pub(crate) const UTF_8_PREFIX_U16_LITTLE_EMPTY_IN: &[u8; 2] = b"\x00\x00";
pub(crate) const UTF_8_PREFIX_U32_LITTLE_EMPTY_IN: &[u8; 4] = b"\x00\x00\x00\x00";
pub(crate) const UTF_8_ZERO_ENDED_LITTLE_EMPTY_IN: &[u8; 1] = b"\x00";

// ---------- UTF-8: Empty data (big)
pub(crate) const UTF_8_FIXED_FORCE_ZERO_BIG_EMPTY_IN: &[u8; UTF_8_FIXED_LENGTH_DATA] =
    &[0; UTF_8_FIXED_LENGTH_DATA];
pub(crate) const UTF_8_FIXED_ALLOW_NO_ZERO_BIG_EMPTY_IN: &[u8; UTF_8_FIXED_LENGTH_DATA] =
    &[0; UTF_8_FIXED_LENGTH_DATA];
pub(crate) const UTF_8_PREFIX_U8_BIG_EMPTY_IN: &[u8; 1] = b"\x00";
pub(crate) const UTF_8_PREFIX_U16_BIG_EMPTY_IN: &[u8; 2] = b"\x00\x00";
pub(crate) const UTF_8_PREFIX_U32_BIG_EMPTY_IN: &[u8; 4] = b"\x00\x00\x00\x00";
pub(crate) const UTF_8_ZERO_ENDED_BIG_EMPTY_IN: &[u8; 1] = b"\x00";

// ---------- UTF-16: Empty data (little)
pub(crate) const UTF_16_FIXED_FORCE_ZERO_LITTLE_EMPTY_IN: &[u8; UTF_16_FIXED_LENGTH_DATA] =
    &[0; UTF_16_FIXED_LENGTH_DATA];
pub(crate) const UTF_16_FIXED_ALLOW_NO_ZERO_LITTLE_EMPTY_IN: &[u8; UTF_16_FIXED_LENGTH_DATA] =
    &[0; UTF_16_FIXED_LENGTH_DATA];
pub(crate) const UTF_16_PREFIX_U8_LITTLE_EMPTY_IN: &[u8; 1] = b"\x00";
pub(crate) const UTF_16_PREFIX_U16_LITTLE_EMPTY_IN: &[u8; 2] = b"\x00\x00";
pub(crate) const UTF_16_PREFIX_U32_LITTLE_EMPTY_IN: &[u8; 4] = b"\x00\x00\x00\x00";
pub(crate) const UTF_16_ZERO_ENDED_LITTLE_EMPTY_IN: &[u8; 2] = b"\x00\x00";

// ---------- UTF-16: Empty data (big)
pub(crate) const UTF_16_FIXED_FORCE_ZERO_BIG_EMPTY_IN: &[u8; UTF_16_FIXED_LENGTH_DATA] =
    &[0; UTF_16_FIXED_LENGTH_DATA];
pub(crate) const UTF_16_FIXED_ALLOW_NO_ZERO_BIG_EMPTY_IN: &[u8; UTF_16_FIXED_LENGTH_DATA] =
    &[0; UTF_16_FIXED_LENGTH_DATA];
pub(crate) const UTF_16_PREFIX_U8_BIG_EMPTY_IN: &[u8; 1] = b"\x00";
pub(crate) const UTF_16_PREFIX_U16_BIG_EMPTY_IN: &[u8; 2] = b"\x00\x00";
pub(crate) const UTF_16_PREFIX_U32_BIG_EMPTY_IN: &[u8; 4] = b"\x00\x00\x00\x00";
pub(crate) const UTF_16_ZERO_ENDED_BIG_EMPTY_IN: &[u8; 2] = b"\x00\x00";

// ------------------------------------
// ------- Full buffer valid ----------
// ------------------------------------

// ---------- UTF-8: Full buffer valid (str)
pub(crate) const FIXED_FORCE_ZERO_VALID_STR: &str = "valid test case w zero";
pub(crate) const FIXED_ALLOW_NO_ZERO_VALID_STR: &str = "valid test case w zero";
pub(crate) const PREFIX_U8_VALID_STR: &str = "valid test case";
pub(crate) const PREFIX_U16_VALID_STR: &str = "valid test case";
pub(crate) const PREFIX_U32_VALID_STR: &str = "valid test case";
pub(crate) const ZERO_ENDED_VALID_STR: &str = "valid test case";

// ---------- UTF-8: Full buffer valid (little)
pub(crate) const UTF_8_FIXED_FORCE_ZERO_LITTLE_VALID_IN: &[u8; UTF_8_FIXED_LENGTH_DATA] =
    b"valid test case w zero\x00";
pub(crate) const UTF_8_FIXED_ALLOW_NO_ZERO_LITTLE_VALID_IN: &[u8; UTF_8_FIXED_LENGTH_DATA] =
    b"valid test case w zero\x00";
pub(crate) const UTF_8_PREFIX_U8_LITTLE_VALID_IN: &[u8; 16] = b"\x0Fvalid test case";
pub(crate) const UTF_8_PREFIX_U16_LITTLE_VALID_IN: &[u8; 17] = b"\x0F\x00valid test case";
pub(crate) const UTF_8_PREFIX_U32_LITTLE_VALID_IN: &[u8; 19] = b"\x0F\x00\x00\x00valid test case";
pub(crate) const UTF_8_ZERO_ENDED_LITTLE_VALID_IN: &[u8; 16] = b"valid test case\x00";

// ---------- UTF-8: Full buffer valid (big)
pub(crate) const UTF_8_FIXED_FORCE_ZERO_BIG_VALID_IN: &[u8; UTF_8_FIXED_LENGTH_DATA] =
    b"valid test case w zero\x00";
pub(crate) const UTF_8_FIXED_ALLOW_NO_ZERO_BIG_VALID_IN: &[u8; UTF_8_FIXED_LENGTH_DATA] =
    b"valid test case w zero\x00";
pub(crate) const UTF_8_PREFIX_U8_BIG_VALID_IN: &[u8; 16] = b"\x0Fvalid test case";
pub(crate) const UTF_8_PREFIX_U16_BIG_VALID_IN: &[u8; 17] = b"\x00\x0Fvalid test case";
pub(crate) const UTF_8_PREFIX_U32_BIG_VALID_IN: &[u8; 19] = b"\x00\x00\x00\x0Fvalid test case";
pub(crate) const UTF_8_ZERO_ENDED_BIG_VALID_IN: &[u8; 16] = b"valid test case\x00";

// ---------- UTF-16: Full buffer valid (little)
pub(crate) const UTF_16_FIXED_FORCE_ZERO_LITTLE_VALID_IN: &[u8; UTF_16_FIXED_LENGTH_DATA] = b"v\x00a\x00l\x00i\x00d\x00 \x00t\x00e\x00s\x00t\x00 \x00c\x00a\x00s\x00e\x00 \x00w\x00 \x00z\x00e\x00r\x00o\x00\x00\x00";
pub(crate) const UTF_16_FIXED_ALLOW_NO_ZERO_LITTLE_VALID_IN: &[u8; UTF_16_FIXED_LENGTH_DATA] = b"v\x00a\x00l\x00i\x00d\x00 \x00t\x00e\x00s\x00t\x00 \x00c\x00a\x00s\x00e\x00 \x00w\x00 \x00z\x00e\x00r\x00o\x00\x00\x00";
pub(crate) const UTF_16_PREFIX_U8_LITTLE_VALID_IN: &[u8; 31] =
    b"\x0Fv\x00a\x00l\x00i\x00d\x00 \x00t\x00e\x00s\x00t\x00 \x00c\x00a\x00s\x00e\x00";
pub(crate) const UTF_16_PREFIX_U16_LITTLE_VALID_IN: &[u8; 32] =
    b"\x0F\x00v\x00a\x00l\x00i\x00d\x00 \x00t\x00e\x00s\x00t\x00 \x00c\x00a\x00s\x00e\x00";
pub(crate) const UTF_16_PREFIX_U32_LITTLE_VALID_IN: &[u8; 34] =
    b"\x0F\x00\x00\x00v\x00a\x00l\x00i\x00d\x00 \x00t\x00e\x00s\x00t\x00 \x00c\x00a\x00s\x00e\x00";
pub(crate) const UTF_16_ZERO_ENDED_LITTLE_VALID_IN: &[u8; 32] =
    b"v\x00a\x00l\x00i\x00d\x00 \x00t\x00e\x00s\x00t\x00 \x00c\x00a\x00s\x00e\x00\x00\x00";

// ---------- UTF-16: Full buffer valid (big)
pub(crate) const UTF_16_FIXED_FORCE_ZERO_BIG_VALID_IN: &[u8; UTF_16_FIXED_LENGTH_DATA] = b"\x00v\x00a\x00l\x00i\x00d\x00 \x00t\x00e\x00s\x00t\x00 \x00c\x00a\x00s\x00e\x00 \x00w\x00 \x00z\x00e\x00r\x00o\x00\x00";
pub(crate) const UTF_16_FIXED_ALLOW_NO_ZERO_BIG_VALID_IN: &[u8; UTF_16_FIXED_LENGTH_DATA] = b"\x00v\x00a\x00l\x00i\x00d\x00 \x00t\x00e\x00s\x00t\x00 \x00c\x00a\x00s\x00e\x00 \x00w\x00 \x00z\x00e\x00r\x00o\x00\x00";
pub(crate) const UTF_16_PREFIX_U8_BIG_VALID_IN: &[u8; 31] =
    b"\x0F\x00v\x00a\x00l\x00i\x00d\x00 \x00t\x00e\x00s\x00t\x00 \x00c\x00a\x00s\x00e";
pub(crate) const UTF_16_PREFIX_U16_BIG_VALID_IN: &[u8; 32] =
    b"\x00\x0F\x00v\x00a\x00l\x00i\x00d\x00 \x00t\x00e\x00s\x00t\x00 \x00c\x00a\x00s\x00e";
pub(crate) const UTF_16_PREFIX_U32_BIG_VALID_IN: &[u8; 34] =
    b"\x00\x00\x00\x0F\x00v\x00a\x00l\x00i\x00d\x00 \x00t\x00e\x00s\x00t\x00 \x00c\x00a\x00s\x00e";
pub(crate) const UTF_16_ZERO_ENDED_BIG_VALID_IN: &[u8; 32] =
    b"\x00v\x00a\x00l\x00i\x00d\x00 \x00t\x00e\x00s\x00t\x00 \x00c\x00a\x00s\x00e\x00\x00";

// ------------------------------------
// ------- Zero in the middle ---------
// ------------------------------------

// ---------- Zero in the middle (str)
pub(crate) const FIXED_FORCE_ZERO_ZERO_IN_MIDDLE_STR: &str = "valid test";
pub(crate) const FIXED_ALLOW_NO_ZERO_ZERO_IN_MIDDLE_STR: &str = "valid test";
// pub(crate) const PREFIX_U8_ZERO_IN_MIDDLE_STR -- not allowed
// pub(crate) const PREFIX_U16_ZERO_IN_MIDDLE_STR -- not allowed
// pub(crate) const PREFIX_U32_ZERO_IN_MIDDLE_STR -- not allowed
// pub(crate) const ZERO_ENDED_ZERO_IN_MIDDLE_STR -- is the end of the value

// ---------- UTF-8: Zero in the middle (little-in)
pub(crate) const UTF_8_FIXED_FORCE_ZERO_LITTLE_ZERO_IN_MIDDLE_IN: &[u8; UTF_8_FIXED_LENGTH_DATA] =
    b"valid test\x00with garbage";
pub(crate) const UTF_8_FIXED_ALLOW_NO_ZERO_LITTLE_ZERO_IN_MIDDLE_IN: &[u8;
     UTF_8_FIXED_LENGTH_DATA] = b"valid test\x00with garbage";

// ---------- UTF-8: Zero in the middle (little-out)
pub(crate) const UTF_8_FIXED_FORCE_ZERO_LITTLE_ZERO_IN_MIDDLE_OUT: &[u8; UTF_8_FIXED_LENGTH_DATA] =
    b"valid test\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";
pub(crate) const UTF_8_FIXED_ALLOW_NO_ZERO_LITTLE_ZERO_IN_MIDDLE_OUT: &[u8;
     UTF_8_FIXED_LENGTH_DATA] = b"valid test\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";

// ---------- UTF-8: Zero in the middle (big-in)
pub(crate) const UTF_8_FIXED_FORCE_ZERO_BIG_ZERO_IN_MIDDLE_IN: &[u8; UTF_8_FIXED_LENGTH_DATA] =
    b"valid test\x00with garbage";
pub(crate) const UTF_8_FIXED_ALLOW_NO_ZERO_BIG_ZERO_IN_MIDDLE_IN: &[u8; UTF_8_FIXED_LENGTH_DATA] =
    b"valid test\x00with garbage";

// ---------- UTF-8: Zero in the middle (big-out)
pub(crate) const UTF_8_FIXED_FORCE_ZERO_BIG_ZERO_IN_MIDDLE_OUT: &[u8; UTF_8_FIXED_LENGTH_DATA] =
    b"valid test\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";
pub(crate) const UTF_8_FIXED_ALLOW_NO_ZERO_BIG_ZERO_IN_MIDDLE_OUT: &[u8; UTF_8_FIXED_LENGTH_DATA] =
    b"valid test\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";

// ---------- UTF-16: Zero in the middle (little-in)
pub(crate) const UTF_16_FIXED_FORCE_ZERO_LITTLE_ZERO_IN_MIDDLE_IN: &[u8; UTF_16_FIXED_LENGTH_DATA] = b"v\x00a\x00l\x00i\x00d\x00 \x00t\x00e\x00s\x00t\x00\x00\x00w\x00i\x00t\x00h\x00 \x00g\x00a\x00r\x00b\x00a\x00g\x00e\x00";
pub(crate) const UTF_16_FIXED_ALLOW_NO_ZERO_LITTLE_ZERO_IN_MIDDLE_IN: &[u8; UTF_16_FIXED_LENGTH_DATA] = b"v\x00a\x00l\x00i\x00d\x00 \x00t\x00e\x00s\x00t\x00\x00\x00w\x00i\x00t\x00h\x00 \x00g\x00a\x00r\x00b\x00a\x00g\x00e\x00";

// ---------- UTF-16: Zero in the middle (little-out)
pub(crate) const UTF_16_FIXED_FORCE_ZERO_LITTLE_ZERO_IN_MIDDLE_OUT: &[u8; UTF_16_FIXED_LENGTH_DATA] =
    b"v\x00a\x00l\x00i\x00d\x00 \x00t\x00e\x00s\x00t\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";
pub(crate) const UTF_16_FIXED_ALLOW_NO_ZERO_LITTLE_ZERO_IN_MIDDLE_OUT: &[u8; UTF_16_FIXED_LENGTH_DATA] =
    b"v\x00a\x00l\x00i\x00d\x00 \x00t\x00e\x00s\x00t\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";

// ---------- UTF-16: Zero in the middle (big-in)
pub(crate) const UTF_16_FIXED_FORCE_ZERO_BIG_ZERO_IN_MIDDLE_IN: &[u8; UTF_16_FIXED_LENGTH_DATA] = b"\x00v\x00a\x00l\x00i\x00d\x00 \x00t\x00e\x00s\x00t\x00\x00\x00w\x00i\x00t\x00h\x00 \x00g\x00a\x00r\x00b\x00a\x00g\x00e";
pub(crate) const UTF_16_FIXED_ALLOW_NO_ZERO_BIG_ZERO_IN_MIDDLE_IN: &[u8; UTF_16_FIXED_LENGTH_DATA] = b"\x00v\x00a\x00l\x00i\x00d\x00 \x00t\x00e\x00s\x00t\x00\x00\x00w\x00i\x00t\x00h\x00 \x00g\x00a\x00r\x00b\x00a\x00g\x00e";

// ---------- UTF-16: Zero in the middle (big-out)
pub(crate) const UTF_16_FIXED_FORCE_ZERO_BIG_ZERO_IN_MIDDLE_OUT: &[u8; UTF_16_FIXED_LENGTH_DATA] =
    b"\x00v\x00a\x00l\x00i\x00d\x00 \x00t\x00e\x00s\x00t\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";
pub(crate) const UTF_16_FIXED_ALLOW_NO_ZERO_BIG_ZERO_IN_MIDDLE_OUT: &[u8; UTF_16_FIXED_LENGTH_DATA] =
    b"\x00v\x00a\x00l\x00i\x00d\x00 \x00t\x00e\x00s\x00t\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";

// ------------------------------------
// ------ No Zero in the buffer -------
// ------------------------------------

// ---------- UTF-8: No zero in fixed buffer (str)
// pub(crate) const FIXED_FORCE_ZERO_NO_ZERO_INSIDE_STR -- not allowed
pub(crate) const FIXED_ALLOW_NO_ZERO_NO_ZERO_INSIDE_STR: &str = "valid test case no zero";
// pub(crate) const PREFIX_U8_NO_ZERO_INSIDE_STR -- same as "valid" case
// pub(crate) const PREFIX_U16_NO_ZERO_INSIDE_STR -- same as "valid" case
// pub(crate) const PREFIX_U32_NO_ZERO_INSIDE_STR -- same as "valid" case
// pub(crate) const ZERO_ENDED_NO_ZERO_INSIDE_STR -- same as "valid" case

// ---------- UTF-8: No zero in fixed buffer (little)
pub(crate) const UTF_8_FIXED_ALLOW_NO_ZERO_LITTLE_NO_ZERO_INSIDE_IN: &[u8;
     UTF_8_FIXED_LENGTH_DATA] = b"valid test case no zero";

// ---------- UTF-8: No zero in fixed buffer (big)
pub(crate) const UTF_8_FIXED_ALLOW_NO_ZERO_BIG_NO_ZERO_INSIDE_IN: &[u8; UTF_8_FIXED_LENGTH_DATA] =
    b"valid test case no zero";

// ---------- UTF-16: No zero in fixed buffer (little)
pub(crate) const UTF_16_FIXED_ALLOW_NO_ZERO_LITTLE_NO_ZERO_INSIDE_IN: &[u8; UTF_16_FIXED_LENGTH_DATA] = b"v\x00a\x00l\x00i\x00d\x00 \x00t\x00e\x00s\x00t\x00 \x00c\x00a\x00s\x00e\x00 \x00n\x00o\x00 \x00z\x00e\x00r\x00o\x00";

// ---------- UTF-16: No zero in fixed buffer (big)
pub(crate) const UTF_16_FIXED_ALLOW_NO_ZERO_BIG_NO_ZERO_INSIDE_IN: &[u8; UTF_16_FIXED_LENGTH_DATA] = b"\x00v\x00a\x00l\x00i\x00d\x00 \x00t\x00e\x00s\x00t\x00 \x00c\x00a\x00s\x00e\x00 \x00n\x00o\x00 \x00z\x00e\x00r\x00o";
