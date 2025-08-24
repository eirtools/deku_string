use super::layouts::FIXED_LENGTH;

/// Raw data size
const UTF_8_FIXED_LENGTH_DATA: usize = FIXED_LENGTH;
/// Raw data size
const UTF_16_FIXED_LENGTH_DATA: usize = FIXED_LENGTH * 2;
const UTF_32_FIXED_LENGTH_DATA: usize = FIXED_LENGTH * 4;

// ------------------------------------
// ---------- Empty data --------------
// ------------------------------------

// ---------- Empty data (str)
pub(crate) const FIXED_FORCE_ZERO_EMPTY_STR: &str = "";
pub(crate) const FIXED_ALLOW_NO_ZERO_EMPTY_STR: &str = "";
pub(crate) const PREFIX_U8_EMPTY_STR: &str = "";
pub(crate) const PREFIX_U16_EMPTY_STR: &str = "";
pub(crate) const PREFIX_U32_EMPTY_STR: &str = "";
pub(crate) const PREFIX_U32_7BIT_EMPTY_STR: &str = "";
pub(crate) const ZERO_ENDED_EMPTY_STR: &str = "";

// ---------- UTF-8: Empty data (little)
pub(crate) const UTF_8_FIXED_FORCE_ZERO_LITTLE_EMPTY_IN: &[u8;
     UTF_8_FIXED_LENGTH_DATA] = &[0; UTF_8_FIXED_LENGTH_DATA];
pub(crate) const UTF_8_FIXED_ALLOW_NO_ZERO_LITTLE_EMPTY_IN:
    &[u8; UTF_8_FIXED_LENGTH_DATA] = &[0; UTF_8_FIXED_LENGTH_DATA];
pub(crate) const UTF_8_PREFIX_U8_LITTLE_EMPTY_IN: &[u8; 1] = b"\x00";
pub(crate) const UTF_8_PREFIX_U16_LITTLE_EMPTY_IN: &[u8; 2] = b"\x00\x00";
pub(crate) const UTF_8_PREFIX_U32_LITTLE_EMPTY_IN: &[u8; 4] = b"\x00\x00\x00\x00";
pub(crate) const UTF_8_PREFIX_U32_7BIT_LITTLE_EMPTY_IN: &[u8; 1] = b"\x00";
pub(crate) const UTF_8_ZERO_ENDED_LITTLE_EMPTY_IN: &[u8; 1] = b"\x00";

// ---------- UTF-8: Empty data (big)
pub(crate) const UTF_8_FIXED_FORCE_ZERO_BIG_EMPTY_IN: &[u8; UTF_8_FIXED_LENGTH_DATA] =
    &[0; UTF_8_FIXED_LENGTH_DATA];
pub(crate) const UTF_8_FIXED_ALLOW_NO_ZERO_BIG_EMPTY_IN: &[u8;
     UTF_8_FIXED_LENGTH_DATA] = &[0; UTF_8_FIXED_LENGTH_DATA];
pub(crate) const UTF_8_PREFIX_U8_BIG_EMPTY_IN: &[u8; 1] = b"\x00";
pub(crate) const UTF_8_PREFIX_U16_BIG_EMPTY_IN: &[u8; 2] = b"\x00\x00";
pub(crate) const UTF_8_PREFIX_U32_BIG_EMPTY_IN: &[u8; 4] = b"\x00\x00\x00\x00";
pub(crate) const UTF_8_PREFIX_U32_7BIT_BIG_EMPTY_IN: &[u8; 1] = b"\x00";
pub(crate) const UTF_8_ZERO_ENDED_BIG_EMPTY_IN: &[u8; 1] = b"\x00";

// ---------- UTF-16: Empty data (little)
pub(crate) const UTF_16_FIXED_FORCE_ZERO_LITTLE_EMPTY_IN:
    &[u8; UTF_16_FIXED_LENGTH_DATA] = &[0; UTF_16_FIXED_LENGTH_DATA];
pub(crate) const UTF_16_FIXED_ALLOW_NO_ZERO_LITTLE_EMPTY_IN:
    &[u8; UTF_16_FIXED_LENGTH_DATA] = &[0; UTF_16_FIXED_LENGTH_DATA];
pub(crate) const UTF_16_PREFIX_U8_LITTLE_EMPTY_IN: &[u8; 1] = b"\x00";
pub(crate) const UTF_16_PREFIX_U16_LITTLE_EMPTY_IN: &[u8; 2] = b"\x00\x00";
pub(crate) const UTF_16_PREFIX_U32_LITTLE_EMPTY_IN: &[u8; 4] = b"\x00\x00\x00\x00";
pub(crate) const UTF_16_PREFIX_U32_7BIT_LITTLE_EMPTY_IN: &[u8; 1] = b"\x00";
pub(crate) const UTF_16_ZERO_ENDED_LITTLE_EMPTY_IN: &[u8; 2] = b"\x00\x00";

// ---------- UTF-16: Empty data (big)
pub(crate) const UTF_16_FIXED_FORCE_ZERO_BIG_EMPTY_IN: &[u8; UTF_16_FIXED_LENGTH_DATA] =
    &[0; UTF_16_FIXED_LENGTH_DATA];
pub(crate) const UTF_16_FIXED_ALLOW_NO_ZERO_BIG_EMPTY_IN:
    &[u8; UTF_16_FIXED_LENGTH_DATA] = &[0; UTF_16_FIXED_LENGTH_DATA];
pub(crate) const UTF_16_PREFIX_U8_BIG_EMPTY_IN: &[u8; 1] = b"\x00";
pub(crate) const UTF_16_PREFIX_U16_BIG_EMPTY_IN: &[u8; 2] = b"\x00\x00";
pub(crate) const UTF_16_PREFIX_U32_BIG_EMPTY_IN: &[u8; 4] = b"\x00\x00\x00\x00";
pub(crate) const UTF_16_PREFIX_U32_7BIT_BIG_EMPTY_IN: &[u8; 1] = b"\x00";
pub(crate) const UTF_16_ZERO_ENDED_BIG_EMPTY_IN: &[u8; 2] = b"\x00\x00";

// ---------- UTF-16: Empty data (little)
pub(crate) const UTF_32_FIXED_FORCE_ZERO_LITTLE_EMPTY_IN:
    &[u8; UTF_32_FIXED_LENGTH_DATA] = &[0; UTF_32_FIXED_LENGTH_DATA];
pub(crate) const UTF_32_FIXED_ALLOW_NO_ZERO_LITTLE_EMPTY_IN:
    &[u8; UTF_32_FIXED_LENGTH_DATA] = &[0; UTF_32_FIXED_LENGTH_DATA];
pub(crate) const UTF_32_PREFIX_U8_LITTLE_EMPTY_IN: &[u8; 1] = b"\x00";
pub(crate) const UTF_32_PREFIX_U16_LITTLE_EMPTY_IN: &[u8; 2] = b"\x00\x00";
pub(crate) const UTF_32_PREFIX_U32_LITTLE_EMPTY_IN: &[u8; 4] = b"\x00\x00\x00\x00";
pub(crate) const UTF_32_PREFIX_U32_7BIT_LITTLE_EMPTY_IN: &[u8; 1] = b"\x00";
pub(crate) const UTF_32_ZERO_ENDED_LITTLE_EMPTY_IN: &[u8; 4] = b"\x00\x00\x00\x00";

// ---------- UTF-32: Empty data (big)
pub(crate) const UTF_32_FIXED_FORCE_ZERO_BIG_EMPTY_IN: &[u8; UTF_32_FIXED_LENGTH_DATA] =
    &[0; UTF_32_FIXED_LENGTH_DATA];
pub(crate) const UTF_32_FIXED_ALLOW_NO_ZERO_BIG_EMPTY_IN:
    &[u8; UTF_32_FIXED_LENGTH_DATA] = &[0; UTF_32_FIXED_LENGTH_DATA];
pub(crate) const UTF_32_PREFIX_U8_BIG_EMPTY_IN: &[u8; 1] = b"\x00";
pub(crate) const UTF_32_PREFIX_U16_BIG_EMPTY_IN: &[u8; 2] = b"\x00\x00";
pub(crate) const UTF_32_PREFIX_U32_BIG_EMPTY_IN: &[u8; 4] = b"\x00\x00\x00\x00";
pub(crate) const UTF_32_PREFIX_U32_7BIT_BIG_EMPTY_IN: &[u8; 1] = b"\x00";
pub(crate) const UTF_32_ZERO_ENDED_BIG_EMPTY_IN: &[u8; 4] = b"\x00\x00\x00\x00";

// ------------------------------------
// ------- Full buffer valid ----------
// ------------------------------------

// ---------- UTF-8: Full buffer valid (str)
pub(crate) const FIXED_FORCE_ZERO_VALID_STR: &str = "zero";
pub(crate) const FIXED_ALLOW_NO_ZERO_VALID_STR: &str = "zero?";
pub(crate) const PREFIX_U8_VALID_STR: &str = "valid";
pub(crate) const PREFIX_U16_VALID_STR: &str = "valid";
pub(crate) const PREFIX_U32_VALID_STR: &str = "valid";
pub(crate) const PREFIX_U32_7BIT_VALID_STR: &str = "valid";
pub(crate) const ZERO_ENDED_VALID_STR: &str = "valid";

// ---------- UTF-8: Full buffer valid (little)
pub(crate) const UTF_8_FIXED_FORCE_ZERO_LITTLE_VALID_IN: &[u8;
     UTF_8_FIXED_LENGTH_DATA] = b"zero\x00";
pub(crate) const UTF_8_FIXED_ALLOW_NO_ZERO_LITTLE_VALID_IN:
    &[u8; UTF_8_FIXED_LENGTH_DATA] = b"zero?";
pub(crate) const UTF_8_PREFIX_U8_LITTLE_VALID_IN: &[u8; 1 + UTF_8_FIXED_LENGTH_DATA] =
    b"\x05valid";
pub(crate) const UTF_8_PREFIX_U16_LITTLE_VALID_IN: &[u8; 2 + UTF_8_FIXED_LENGTH_DATA] =
    b"\x05\x00valid";
pub(crate) const UTF_8_PREFIX_U32_LITTLE_VALID_IN: &[u8; 4 + UTF_8_FIXED_LENGTH_DATA] =
    b"\x05\x00\x00\x00valid";
pub(crate) const UTF_8_PREFIX_U32_7BIT_LITTLE_VALID_IN:
    &[u8; 1 + UTF_8_FIXED_LENGTH_DATA] = b"\x05valid";
pub(crate) const UTF_8_ZERO_ENDED_LITTLE_VALID_IN: &[u8; 1 + UTF_8_FIXED_LENGTH_DATA] =
    b"valid\x00";

// ---------- UTF-8: Full buffer valid (big)
pub(crate) const UTF_8_FIXED_FORCE_ZERO_BIG_VALID_IN: &[u8; UTF_8_FIXED_LENGTH_DATA] =
    b"zero\x00";
pub(crate) const UTF_8_FIXED_ALLOW_NO_ZERO_BIG_VALID_IN: &[u8;
     UTF_8_FIXED_LENGTH_DATA] = b"zero?";
pub(crate) const UTF_8_PREFIX_U8_BIG_VALID_IN: &[u8; 1 + UTF_8_FIXED_LENGTH_DATA] =
    b"\x05valid";
pub(crate) const UTF_8_PREFIX_U16_BIG_VALID_IN: &[u8; 2 + UTF_8_FIXED_LENGTH_DATA] =
    b"\x00\x05valid";
pub(crate) const UTF_8_PREFIX_U32_BIG_VALID_IN: &[u8; 4 + UTF_8_FIXED_LENGTH_DATA] =
    b"\x00\x00\x00\x05valid";
pub(crate) const UTF_8_PREFIX_U32_7BIT_BIG_VALID_IN: &[u8; 1
     + UTF_8_FIXED_LENGTH_DATA] = b"\x05valid";
pub(crate) const UTF_8_ZERO_ENDED_BIG_VALID_IN: &[u8; 1 + UTF_8_FIXED_LENGTH_DATA] =
    b"valid\x00";

// ---------- UTF-16: Full buffer valid (little)
pub(crate) const UTF_16_FIXED_FORCE_ZERO_LITTLE_VALID_IN:
    &[u8; UTF_16_FIXED_LENGTH_DATA] = b"z\x00e\x00r\x00o\x00\x00\x00";
pub(crate) const UTF_16_FIXED_ALLOW_NO_ZERO_LITTLE_VALID_IN:
    &[u8; UTF_16_FIXED_LENGTH_DATA] = b"z\x00e\x00r\x00o\x00?\x00";
pub(crate) const UTF_16_PREFIX_U8_LITTLE_VALID_IN: &[u8; 1 + UTF_16_FIXED_LENGTH_DATA] =
    b"\x05v\x00a\x00l\x00i\x00d\x00";
pub(crate) const UTF_16_PREFIX_U16_LITTLE_VALID_IN: &[u8; 2
     + UTF_16_FIXED_LENGTH_DATA] = b"\x05\x00v\x00a\x00l\x00i\x00d\x00";
pub(crate) const UTF_16_PREFIX_U32_LITTLE_VALID_IN: &[u8; 4
     + UTF_16_FIXED_LENGTH_DATA] = b"\x05\x00\x00\x00v\x00a\x00l\x00i\x00d\x00";
pub(crate) const UTF_16_PREFIX_U32_7BIT_LITTLE_VALID_IN:
    &[u8; 1 + UTF_16_FIXED_LENGTH_DATA] = b"\x05v\x00a\x00l\x00i\x00d\x00";
pub(crate) const UTF_16_ZERO_ENDED_LITTLE_VALID_IN: &[u8; 2
     + UTF_16_FIXED_LENGTH_DATA] = b"v\x00a\x00l\x00i\x00d\x00\x00\x00";

// ---------- UTF-16: Full buffer valid (big)
pub(crate) const UTF_16_FIXED_FORCE_ZERO_BIG_VALID_IN: &[u8; UTF_16_FIXED_LENGTH_DATA] =
    b"\x00z\x00e\x00r\x00o\x00\x00";
pub(crate) const UTF_16_FIXED_ALLOW_NO_ZERO_BIG_VALID_IN:
    &[u8; UTF_16_FIXED_LENGTH_DATA] = b"\x00z\x00e\x00r\x00o\x00?";
pub(crate) const UTF_16_PREFIX_U8_BIG_VALID_IN: &[u8; 1 + UTF_16_FIXED_LENGTH_DATA] =
    b"\x05\x00v\x00a\x00l\x00i\x00d";
pub(crate) const UTF_16_PREFIX_U16_BIG_VALID_IN: &[u8; 2 + UTF_16_FIXED_LENGTH_DATA] =
    b"\x00\x05\x00v\x00a\x00l\x00i\x00d";
pub(crate) const UTF_16_PREFIX_U32_BIG_VALID_IN: &[u8; 4 + UTF_16_FIXED_LENGTH_DATA] =
    b"\x00\x00\x00\x05\x00v\x00a\x00l\x00i\x00d";
pub(crate) const UTF_16_PREFIX_U32_7BIT_BIG_VALID_IN: &[u8; 1
     + UTF_16_FIXED_LENGTH_DATA] = b"\x05\x00v\x00a\x00l\x00i\x00d";
pub(crate) const UTF_16_ZERO_ENDED_BIG_VALID_IN: &[u8; 2 + UTF_16_FIXED_LENGTH_DATA] =
    b"\x00v\x00a\x00l\x00i\x00d\x00\x00";

// ---------- UTF-32: Full buffer valid (little)
pub(crate) const UTF_32_FIXED_FORCE_ZERO_LITTLE_VALID_IN:
    &[u8; UTF_32_FIXED_LENGTH_DATA] =
    b"z\x00\x00\x00e\x00\x00\x00r\x00\x00\x00o\x00\x00\x00\x00\x00\x00\x00";
pub(crate) const UTF_32_FIXED_ALLOW_NO_ZERO_LITTLE_VALID_IN:
    &[u8; UTF_32_FIXED_LENGTH_DATA] =
    b"z\x00\x00\x00e\x00\x00\x00r\x00\x00\x00o\x00\x00\x00?\x00\x00\x00";
pub(crate) const UTF_32_PREFIX_U8_LITTLE_VALID_IN: &[u8; 1 + UTF_32_FIXED_LENGTH_DATA] =
    b"\x05v\x00\x00\x00a\x00\x00\x00l\x00\x00\x00i\x00\x00\x00d\x00\x00\x00";
pub(crate) const UTF_32_PREFIX_U16_LITTLE_VALID_IN: &[u8; 2
     + UTF_32_FIXED_LENGTH_DATA] =
    b"\x05\x00v\x00\x00\x00a\x00\x00\x00l\x00\x00\x00i\x00\x00\x00d\x00\x00\x00";
pub(crate) const UTF_32_PREFIX_U32_LITTLE_VALID_IN: &[u8; 4 + UTF_32_FIXED_LENGTH_DATA] =
    b"\x05\x00\x00\x00v\x00\x00\x00a\x00\x00\x00l\x00\x00\x00i\x00\x00\x00d\x00\x00\x00";
pub(crate) const UTF_32_PREFIX_U32_7BIT_LITTLE_VALID_IN:
    &[u8; 1 + UTF_32_FIXED_LENGTH_DATA] =
    b"\x05v\x00\x00\x00a\x00\x00\x00l\x00\x00\x00i\x00\x00\x00d\x00\x00\x00";
pub(crate) const UTF_32_ZERO_ENDED_LITTLE_VALID_IN: &[u8; 4 + UTF_32_FIXED_LENGTH_DATA] =
    b"v\x00\x00\x00a\x00\x00\x00l\x00\x00\x00i\x00\x00\x00d\x00\x00\x00\x00\x00\x00\x00";

// ---------- UTF-32: Full buffer valid (big)
pub(crate) const UTF_32_FIXED_FORCE_ZERO_BIG_VALID_IN: &[u8; UTF_32_FIXED_LENGTH_DATA] =
    b"\x00\x00\x00z\x00\x00\x00e\x00\x00\x00r\x00\x00\x00o\x00\x00\x00\x00";
pub(crate) const UTF_32_FIXED_ALLOW_NO_ZERO_BIG_VALID_IN:
    &[u8; UTF_32_FIXED_LENGTH_DATA] =
    b"\x00\x00\x00z\x00\x00\x00e\x00\x00\x00r\x00\x00\x00o\x00\x00\x00?";
pub(crate) const UTF_32_PREFIX_U8_BIG_VALID_IN: &[u8; 1 + UTF_32_FIXED_LENGTH_DATA] =
    b"\x05\x00\x00\x00v\x00\x00\x00a\x00\x00\x00l\x00\x00\x00i\x00\x00\x00d";
pub(crate) const UTF_32_PREFIX_U16_BIG_VALID_IN: &[u8; 2 + UTF_32_FIXED_LENGTH_DATA] =
    b"\x00\x05\x00\x00\x00v\x00\x00\x00a\x00\x00\x00l\x00\x00\x00i\x00\x00\x00d";
pub(crate) const UTF_32_PREFIX_U32_BIG_VALID_IN: &[u8; 4 + UTF_32_FIXED_LENGTH_DATA] =
    b"\x00\x00\x00\x05\x00\x00\x00v\x00\x00\x00a\x00\x00\x00l\x00\x00\x00i\x00\x00\x00d";
pub(crate) const UTF_32_PREFIX_U32_7BIT_BIG_VALID_IN: &[u8; 1
     + UTF_32_FIXED_LENGTH_DATA] =
    b"\x05\x00\x00\x00v\x00\x00\x00a\x00\x00\x00l\x00\x00\x00i\x00\x00\x00d";
pub(crate) const UTF_32_ZERO_ENDED_BIG_VALID_IN: &[u8; 4 + UTF_32_FIXED_LENGTH_DATA] =
    b"\x00\x00\x00v\x00\x00\x00a\x00\x00\x00l\x00\x00\x00i\x00\x00\x00d\x00\x00\x00\x00";

// ------------------------------------
// ------- Zero in the middle ---------
// ------------------------------------

// ---------- Zero in the middle (str)
pub(crate) const FIXED_FORCE_ZERO_ZERO_IN_MIDDLE_STR: &str = "va";
pub(crate) const FIXED_ALLOW_NO_ZERO_ZERO_IN_MIDDLE_STR: &str = "va";
// pub(crate) const PREFIX_U8_ZERO_IN_MIDDLE_STR -- not allowed
// pub(crate) const PREFIX_U16_ZERO_IN_MIDDLE_STR -- not allowed
// pub(crate) const PREFIX_U32_ZERO_IN_MIDDLE_STR -- not allowed
// pub(crate) const ZERO_ENDED_ZERO_IN_MIDDLE_STR -- is the end of the value

// ---------- UTF-8: Zero in the middle (little-in)
pub(crate) const UTF_8_FIXED_FORCE_ZERO_LITTLE_ZERO_IN_MIDDLE_IN:
    &[u8; UTF_8_FIXED_LENGTH_DATA] = b"va\x00id";
pub(crate) const UTF_8_FIXED_ALLOW_NO_ZERO_LITTLE_ZERO_IN_MIDDLE_IN:
    &[u8; UTF_8_FIXED_LENGTH_DATA] = b"va\x00id";

// ---------- UTF-8: Zero in the middle (little-out)
pub(crate) const UTF_8_FIXED_FORCE_ZERO_LITTLE_ZERO_IN_MIDDLE_OUT:
    &[u8; UTF_8_FIXED_LENGTH_DATA] = b"va\x00\x00\x00";
pub(crate) const UTF_8_FIXED_ALLOW_NO_ZERO_LITTLE_ZERO_IN_MIDDLE_OUT:
    &[u8; UTF_8_FIXED_LENGTH_DATA] = b"va\x00\x00\x00";

// ---------- UTF-8: Zero in the middle (big-in)
pub(crate) const UTF_8_FIXED_FORCE_ZERO_BIG_ZERO_IN_MIDDLE_IN:
    &[u8; UTF_8_FIXED_LENGTH_DATA] = b"va\x00id";
pub(crate) const UTF_8_FIXED_ALLOW_NO_ZERO_BIG_ZERO_IN_MIDDLE_IN:
    &[u8; UTF_8_FIXED_LENGTH_DATA] = b"va\x00id";

// ---------- UTF-8: Zero in the middle (big-out)
pub(crate) const UTF_8_FIXED_FORCE_ZERO_BIG_ZERO_IN_MIDDLE_OUT:
    &[u8; UTF_8_FIXED_LENGTH_DATA] = b"va\x00\x00\x00";
pub(crate) const UTF_8_FIXED_ALLOW_NO_ZERO_BIG_ZERO_IN_MIDDLE_OUT:
    &[u8; UTF_8_FIXED_LENGTH_DATA] = b"va\x00\x00\x00";

// ---------- UTF-16: Zero in the middle (little-in)
pub(crate) const UTF_16_FIXED_FORCE_ZERO_LITTLE_ZERO_IN_MIDDLE_IN:
    &[u8; UTF_16_FIXED_LENGTH_DATA] = b"v\x00a\x00\x00\x00i\x00d\x00";
pub(crate) const UTF_16_FIXED_ALLOW_NO_ZERO_LITTLE_ZERO_IN_MIDDLE_IN:
    &[u8; UTF_16_FIXED_LENGTH_DATA] = b"v\x00a\x00\x00\x00i\x00d\x00";

// ---------- UTF-16: Zero in the middle (little-out)
pub(crate) const UTF_16_FIXED_FORCE_ZERO_LITTLE_ZERO_IN_MIDDLE_OUT:
    &[u8; UTF_16_FIXED_LENGTH_DATA] = b"v\x00a\x00\x00\x00\x00\x00\x00\x00";
pub(crate) const UTF_16_FIXED_ALLOW_NO_ZERO_LITTLE_ZERO_IN_MIDDLE_OUT:
    &[u8; UTF_16_FIXED_LENGTH_DATA] = b"v\x00a\x00\x00\x00\x00\x00\x00\x00";

// ---------- UTF-16: Zero in the middle (big-in)
pub(crate) const UTF_16_FIXED_FORCE_ZERO_BIG_ZERO_IN_MIDDLE_IN:
    &[u8; UTF_16_FIXED_LENGTH_DATA] = b"\x00v\x00a\x00\x00\x00i\x00d";
pub(crate) const UTF_16_FIXED_ALLOW_NO_ZERO_BIG_ZERO_IN_MIDDLE_IN:
    &[u8; UTF_16_FIXED_LENGTH_DATA] = b"\x00v\x00a\x00\x00\x00i\x00d";

// ---------- UTF-16: Zero in the middle (big-out)
pub(crate) const UTF_16_FIXED_FORCE_ZERO_BIG_ZERO_IN_MIDDLE_OUT:
    &[u8; UTF_16_FIXED_LENGTH_DATA] = b"\x00v\x00a\x00\x00\x00\x00\x00\x00";
pub(crate) const UTF_16_FIXED_ALLOW_NO_ZERO_BIG_ZERO_IN_MIDDLE_OUT:
    &[u8; UTF_16_FIXED_LENGTH_DATA] = b"\x00v\x00a\x00\x00\x00\x00\x00\x00";

// ---------- UTF-32: Zero in the middle (little-in)
pub(crate) const UTF_32_FIXED_FORCE_ZERO_LITTLE_ZERO_IN_MIDDLE_IN:
    &[u8; UTF_32_FIXED_LENGTH_DATA] =
    b"v\x00\x00\x00a\x00\x00\x00\x00\x00\x00\x00i\x00\x00\x00d\x00\x00\x00";
pub(crate) const UTF_32_FIXED_ALLOW_NO_ZERO_LITTLE_ZERO_IN_MIDDLE_IN:
    &[u8; UTF_32_FIXED_LENGTH_DATA] =
    b"v\x00\x00\x00a\x00\x00\x00\x00\x00\x00\x00i\x00\x00\x00d\x00\x00\x00";

// ---------- UTF-32: Zero in the middle (little-out)
pub(crate) const UTF_32_FIXED_FORCE_ZERO_LITTLE_ZERO_IN_MIDDLE_OUT:
    &[u8; UTF_32_FIXED_LENGTH_DATA] =
    b"v\x00\x00\x00a\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";
pub(crate) const UTF_32_FIXED_ALLOW_NO_ZERO_LITTLE_ZERO_IN_MIDDLE_OUT:
    &[u8; UTF_32_FIXED_LENGTH_DATA] =
    b"v\x00\x00\x00a\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";

// ---------- UTF-32: Zero in the middle (big-in)
pub(crate) const UTF_32_FIXED_FORCE_ZERO_BIG_ZERO_IN_MIDDLE_IN:
    &[u8; UTF_32_FIXED_LENGTH_DATA] =
    b"\x00\x00\x00v\x00\x00\x00a\x00\x00\x00\x00\x00\x00\x00i\x00\x00\x00d";
pub(crate) const UTF_32_FIXED_ALLOW_NO_ZERO_BIG_ZERO_IN_MIDDLE_IN:
    &[u8; UTF_32_FIXED_LENGTH_DATA] =
    b"\x00\x00\x00v\x00\x00\x00a\x00\x00\x00\x00\x00\x00\x00i\x00\x00\x00d";

// ---------- UTF-32: Zero in the middle (big-out)
pub(crate) const UTF_32_FIXED_FORCE_ZERO_BIG_ZERO_IN_MIDDLE_OUT:
    &[u8; UTF_32_FIXED_LENGTH_DATA] =
    b"\x00\x00\x00v\x00\x00\x00a\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";
pub(crate) const UTF_32_FIXED_ALLOW_NO_ZERO_BIG_ZERO_IN_MIDDLE_OUT:
    &[u8; UTF_32_FIXED_LENGTH_DATA] =
    b"\x00\x00\x00v\x00\x00\x00a\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";
// ------------------------------------
// ------ No Zero in the buffer -------
// ------------------------------------

// ---------- UTF-8: No zero in fixed buffer (str)
// pub(crate) const FIXED_FORCE_ZERO_NO_ZERO_INSIDE_STR -- not allowed
pub(crate) const FIXED_ALLOW_NO_ZERO_NO_ZERO_INSIDE_STR: &str = "valid";
// pub(crate) const PREFIX_U8_NO_ZERO_INSIDE_STR -- same as "valid" case
// pub(crate) const PREFIX_U16_NO_ZERO_INSIDE_STR -- same as "valid" case
// pub(crate) const PREFIX_U32_NO_ZERO_INSIDE_STR -- same as "valid" case
// pub(crate) const ZERO_ENDED_NO_ZERO_INSIDE_STR -- same as "valid" case

// ---------- UTF-8: No zero in fixed buffer (little)
pub(crate) const UTF_8_FIXED_ALLOW_NO_ZERO_LITTLE_NO_ZERO_INSIDE_IN:
    &[u8; UTF_8_FIXED_LENGTH_DATA] = b"valid";

// ---------- UTF-8: No zero in fixed buffer (big)
pub(crate) const UTF_8_FIXED_ALLOW_NO_ZERO_BIG_NO_ZERO_INSIDE_IN:
    &[u8; UTF_8_FIXED_LENGTH_DATA] = b"valid";

// ---------- UTF-16: No zero in fixed buffer (little)
pub(crate) const UTF_16_FIXED_ALLOW_NO_ZERO_LITTLE_NO_ZERO_INSIDE_IN:
    &[u8; UTF_16_FIXED_LENGTH_DATA] = b"v\x00a\x00l\x00i\x00d\x00";

// ---------- UTF-16: No zero in fixed buffer (big)
pub(crate) const UTF_16_FIXED_ALLOW_NO_ZERO_BIG_NO_ZERO_INSIDE_IN:
    &[u8; UTF_16_FIXED_LENGTH_DATA] = b"\x00v\x00a\x00l\x00i\x00d";

// ---------- UTF-32: No zero in fixed buffer (little)
pub(crate) const UTF_32_FIXED_ALLOW_NO_ZERO_LITTLE_NO_ZERO_INSIDE_IN:
    &[u8; UTF_32_FIXED_LENGTH_DATA] =
    b"v\x00\x00\x00a\x00\x00\x00l\x00\x00\x00i\x00\x00\x00d\x00\x00\x00";

// ---------- UTF-32: No zero in fixed buffer (big)
pub(crate) const UTF_32_FIXED_ALLOW_NO_ZERO_BIG_NO_ZERO_INSIDE_IN:
    &[u8; UTF_32_FIXED_LENGTH_DATA] =
    b"\x00\x00\x00v\x00\x00\x00a\x00\x00\x00l\x00\x00\x00i\x00\x00\x00d";

// --------------------------------------
// ------ 255 bytes in the buffer -------
// --------------------------------------

/// Raw data size
const UTF_8_255_DATA: usize = 255;
/// Raw data size
const UTF_16_255_DATA: usize = 255 * 2;
const UTF_32_255_DATA: usize = 255 * 4;

// pub(crate) const FIXED_FORCE_LEN_255_STR -- same as "valid" case
// pub(crate) const FIXED_LEN_255_STR -- same as "valid" case
pub(crate) const PREFIX_U8_LEN_255_STR: &str = include_str!("../../data/data_255");
// pub(crate) const PREFIX_U16_LEN_255_STR -- same as "valid" case
// pub(crate) const PREFIX_U32_LEN_255_STR -- same as "valid" case
// pub(crate) const ZERO_ENDED_LEN_255_STR -- same as "valid" case

// ---------- UTF-8: 255 bytes in the buffer (little)
pub(crate) const UTF_8_PREFIX_U8_LITTLE_LEN_255_IN: &[u8; 1 + UTF_8_255_DATA] =
    include_bytes!("../../data/utf8_u8_little_data_255");

// ---------- UTF-16: 255 bytes in the buffer (little)
pub(crate) const UTF_16_PREFIX_U8_LITTLE_LEN_255_IN: &[u8; 1 + UTF_16_255_DATA] =
    include_bytes!("../../data/utf16_u8_little_data_255");

// ---------- UTF-32: 255 bytes in the buffer (little)
pub(crate) const UTF_32_PREFIX_U8_LITTLE_LEN_255_IN: &[u8; 1 + UTF_32_255_DATA] =
    include_bytes!("../../data/utf32_u8_little_data_255");

// ---------- UTF-8: 255 bytes in the buffer (big)
pub(crate) const UTF_8_PREFIX_U8_BIG_LEN_255_IN: &[u8; 1 + UTF_8_255_DATA] =
    include_bytes!("../../data/utf8_u8_big_data_255");

// ---------- UTF-16: 255 bytes in the buffer (big)
pub(crate) const UTF_16_PREFIX_U8_BIG_LEN_255_IN: &[u8; 1 + UTF_16_255_DATA] =
    include_bytes!("../../data/utf16_u8_big_data_255");

// ---------- UTF-32: 255 bytes in the buffer (big)
pub(crate) const UTF_32_PREFIX_U8_BIG_LEN_255_IN: &[u8; 1 + UTF_32_255_DATA] =
    include_bytes!("../../data/utf32_u8_big_data_255");
