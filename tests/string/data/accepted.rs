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

// ---------- Empty data (UTF-8 little)
pub(crate) const UTF_8_FIXED_FORCE_ZERO_LITTLE_EMPTY_IN: &[u8;
     UTF_8_FIXED_LENGTH_DATA] = &[0; UTF_8_FIXED_LENGTH_DATA];
pub(crate) const UTF_8_FIXED_ALLOW_NO_ZERO_LITTLE_EMPTY_IN:
    &[u8; UTF_8_FIXED_LENGTH_DATA] = &[0; UTF_8_FIXED_LENGTH_DATA];
pub(crate) const UTF_8_PREFIX_U8_LITTLE_EMPTY_IN: &[u8; 1] = b"\x00";
pub(crate) const UTF_8_PREFIX_U16_LITTLE_EMPTY_IN: &[u8; 2] = b"\x00\x00";
pub(crate) const UTF_8_PREFIX_U32_LITTLE_EMPTY_IN: &[u8; 4] = b"\x00\x00\x00\x00";
pub(crate) const UTF_8_PREFIX_U32_7BIT_LITTLE_EMPTY_IN: &[u8; 1] = b"\x00";
pub(crate) const UTF_8_ZERO_ENDED_LITTLE_EMPTY_IN: &[u8; 1] = b"\x00";

// ---------- Empty data (UTF-16 little)
pub(crate) const UTF_16_FIXED_FORCE_ZERO_LITTLE_EMPTY_IN:
    &[u8; UTF_16_FIXED_LENGTH_DATA] = &[0; UTF_16_FIXED_LENGTH_DATA];
pub(crate) const UTF_16_FIXED_ALLOW_NO_ZERO_LITTLE_EMPTY_IN:
    &[u8; UTF_16_FIXED_LENGTH_DATA] = &[0; UTF_16_FIXED_LENGTH_DATA];
pub(crate) const UTF_16_PREFIX_U8_LITTLE_EMPTY_IN: &[u8; 1] = b"\x00";
pub(crate) const UTF_16_PREFIX_U16_LITTLE_EMPTY_IN: &[u8; 2] = b"\x00\x00";
pub(crate) const UTF_16_PREFIX_U32_LITTLE_EMPTY_IN: &[u8; 4] = b"\x00\x00\x00\x00";
pub(crate) const UTF_16_PREFIX_U32_7BIT_LITTLE_EMPTY_IN: &[u8; 1] = b"\x00";
pub(crate) const UTF_16_ZERO_ENDED_LITTLE_EMPTY_IN: &[u8; 2] = b"\x00\x00";

// ---------- Empty data (UTF-32 little)
pub(crate) const UTF_32_FIXED_FORCE_ZERO_LITTLE_EMPTY_IN:
    &[u8; UTF_32_FIXED_LENGTH_DATA] = &[0; UTF_32_FIXED_LENGTH_DATA];
pub(crate) const UTF_32_FIXED_ALLOW_NO_ZERO_LITTLE_EMPTY_IN:
    &[u8; UTF_32_FIXED_LENGTH_DATA] = &[0; UTF_32_FIXED_LENGTH_DATA];
pub(crate) const UTF_32_PREFIX_U8_LITTLE_EMPTY_IN: &[u8; 1] = b"\x00";
pub(crate) const UTF_32_PREFIX_U16_LITTLE_EMPTY_IN: &[u8; 2] = b"\x00\x00";
pub(crate) const UTF_32_PREFIX_U32_LITTLE_EMPTY_IN: &[u8; 4] = b"\x00\x00\x00\x00";
pub(crate) const UTF_32_PREFIX_U32_7BIT_LITTLE_EMPTY_IN: &[u8; 1] = b"\x00";
pub(crate) const UTF_32_ZERO_ENDED_LITTLE_EMPTY_IN: &[u8; 4] = b"\x00\x00\x00\x00";

// ---------- Empty data (UTF-8 big)
pub(crate) const UTF_8_FIXED_FORCE_ZERO_BIG_EMPTY_IN: &[u8; UTF_8_FIXED_LENGTH_DATA] =
    &[0; UTF_8_FIXED_LENGTH_DATA];
pub(crate) const UTF_8_FIXED_ALLOW_NO_ZERO_BIG_EMPTY_IN: &[u8;
     UTF_8_FIXED_LENGTH_DATA] = &[0; UTF_8_FIXED_LENGTH_DATA];
pub(crate) const UTF_8_PREFIX_U8_BIG_EMPTY_IN: &[u8; 1] = b"\x00";
pub(crate) const UTF_8_PREFIX_U16_BIG_EMPTY_IN: &[u8; 2] = b"\x00\x00";
pub(crate) const UTF_8_PREFIX_U32_BIG_EMPTY_IN: &[u8; 4] = b"\x00\x00\x00\x00";
pub(crate) const UTF_8_PREFIX_U32_7BIT_BIG_EMPTY_IN: &[u8; 1] = b"\x00";
pub(crate) const UTF_8_ZERO_ENDED_BIG_EMPTY_IN: &[u8; 1] = b"\x00";

// ---------- Empty data (UTF-16 big)
pub(crate) const UTF_16_FIXED_FORCE_ZERO_BIG_EMPTY_IN: &[u8; UTF_16_FIXED_LENGTH_DATA] =
    &[0; UTF_16_FIXED_LENGTH_DATA];
pub(crate) const UTF_16_FIXED_ALLOW_NO_ZERO_BIG_EMPTY_IN:
    &[u8; UTF_16_FIXED_LENGTH_DATA] = &[0; UTF_16_FIXED_LENGTH_DATA];
pub(crate) const UTF_16_PREFIX_U8_BIG_EMPTY_IN: &[u8; 1] = b"\x00";
pub(crate) const UTF_16_PREFIX_U16_BIG_EMPTY_IN: &[u8; 2] = b"\x00\x00";
pub(crate) const UTF_16_PREFIX_U32_BIG_EMPTY_IN: &[u8; 4] = b"\x00\x00\x00\x00";
pub(crate) const UTF_16_PREFIX_U32_7BIT_BIG_EMPTY_IN: &[u8; 1] = b"\x00";
pub(crate) const UTF_16_ZERO_ENDED_BIG_EMPTY_IN: &[u8; 2] = b"\x00\x00";

// ---------- Empty data (UTF-32 big)
pub(crate) const UTF_32_FIXED_FORCE_ZERO_BIG_EMPTY_IN: &[u8; UTF_32_FIXED_LENGTH_DATA] =
    &[0; UTF_32_FIXED_LENGTH_DATA];
pub(crate) const UTF_32_FIXED_ALLOW_NO_ZERO_BIG_EMPTY_IN:
    &[u8; UTF_32_FIXED_LENGTH_DATA] = &[0; UTF_32_FIXED_LENGTH_DATA];
pub(crate) const UTF_32_PREFIX_U8_BIG_EMPTY_IN: &[u8; 1] = b"\x00";
pub(crate) const UTF_32_PREFIX_U16_BIG_EMPTY_IN: &[u8; 2] = b"\x00\x00";
pub(crate) const UTF_32_PREFIX_U32_BIG_EMPTY_IN: &[u8; 4] = b"\x00\x00\x00\x00";
pub(crate) const UTF_32_PREFIX_U32_7BIT_BIG_EMPTY_IN: &[u8; 1] = b"\x00";
pub(crate) const UTF_32_ZERO_ENDED_BIG_EMPTY_IN: &[u8; 4] = b"\x00\x00\x00\x00";

// -------------%<-------------%<------|------->%------------->%-------------

// -------------------------------
// ------- Valid buffer ----------
// -------------------------------
//
// * `StringLayout::FixedLength` — no padding

// ---------- Valid buffer (str)
pub(crate) const FIXED_FORCE_ZERO_VALID_STR: &str = "zero";
pub(crate) const FIXED_ALLOW_NO_ZERO_VALID_STR: &str = "zero?";
pub(crate) const PREFIX_U8_VALID_STR: &str = "valid";
pub(crate) const PREFIX_U16_VALID_STR: &str = "valid";
pub(crate) const PREFIX_U32_VALID_STR: &str = "valid";
pub(crate) const PREFIX_U32_7BIT_VALID_STR: &str = "valid";
pub(crate) const ZERO_ENDED_VALID_STR: &str = "valid";

// ---------- Valid buffer (UTF-8 little)
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

// ---------- Valid buffer (UTF-16 little)
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

// ---------- Valid buffer (UTF-32 little)
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

// ---------- Valid buffer (UTF-8 big)
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

// ---------- Valid buffer (UTF-16 big)
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

// ---------- Valid buffer (UTF-32 big)
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

// -------------%<-------------%<------|------->%------------->%-------------

// ------------------------------------
// ------- Zero in the middle ---------
// ------------------------------------
//
// Input buffer has zero byte
//
// * `StringLayout::FixedLength` — not allowed
// * `StringLayout::ZeroEnded` — null character is the end of the data

// ---------- Zero in the middle (str)
pub(crate) const FIXED_FORCE_ZERO_ZERO_IN_MIDDLE_STR: &str = "va";
pub(crate) const FIXED_ALLOW_NO_ZERO_ZERO_IN_MIDDLE_STR: &str = "va";

// ---------- Zero in the middle (UTF-8 little) — IN
pub(crate) const UTF_8_FIXED_FORCE_ZERO_LITTLE_ZERO_IN_MIDDLE_IN:
    &[u8; UTF_8_FIXED_LENGTH_DATA] = b"va\x00id";
pub(crate) const UTF_8_FIXED_ALLOW_NO_ZERO_LITTLE_ZERO_IN_MIDDLE_IN:
    &[u8; UTF_8_FIXED_LENGTH_DATA] = b"va\x00id";

// ---------- Zero in the middle (UTF-8 little) — OUT
pub(crate) const UTF_8_FIXED_FORCE_ZERO_LITTLE_ZERO_IN_MIDDLE_OUT:
    &[u8; UTF_8_FIXED_LENGTH_DATA] = b"va\x00\x00\x00";
pub(crate) const UTF_8_FIXED_ALLOW_NO_ZERO_LITTLE_ZERO_IN_MIDDLE_OUT:
    &[u8; UTF_8_FIXED_LENGTH_DATA] = b"va\x00\x00\x00";

// ---------- Zero in the middle (UTF-16 little) — IN
pub(crate) const UTF_16_FIXED_FORCE_ZERO_LITTLE_ZERO_IN_MIDDLE_IN:
    &[u8; UTF_16_FIXED_LENGTH_DATA] = b"v\x00a\x00\x00\x00i\x00d\x00";
pub(crate) const UTF_16_FIXED_ALLOW_NO_ZERO_LITTLE_ZERO_IN_MIDDLE_IN:
    &[u8; UTF_16_FIXED_LENGTH_DATA] = b"v\x00a\x00\x00\x00i\x00d\x00";

// ---------- Zero in the middle (UTF-16 little) — OUT
pub(crate) const UTF_16_FIXED_FORCE_ZERO_LITTLE_ZERO_IN_MIDDLE_OUT:
    &[u8; UTF_16_FIXED_LENGTH_DATA] = b"v\x00a\x00\x00\x00\x00\x00\x00\x00";
pub(crate) const UTF_16_FIXED_ALLOW_NO_ZERO_LITTLE_ZERO_IN_MIDDLE_OUT:
    &[u8; UTF_16_FIXED_LENGTH_DATA] = b"v\x00a\x00\x00\x00\x00\x00\x00\x00";

// ---------- Zero in the middle (UTF-32 little) — IN
pub(crate) const UTF_32_FIXED_FORCE_ZERO_LITTLE_ZERO_IN_MIDDLE_IN:
    &[u8; UTF_32_FIXED_LENGTH_DATA] =
    b"v\x00\x00\x00a\x00\x00\x00\x00\x00\x00\x00i\x00\x00\x00d\x00\x00\x00";
pub(crate) const UTF_32_FIXED_ALLOW_NO_ZERO_LITTLE_ZERO_IN_MIDDLE_IN:
    &[u8; UTF_32_FIXED_LENGTH_DATA] =
    b"v\x00\x00\x00a\x00\x00\x00\x00\x00\x00\x00i\x00\x00\x00d\x00\x00\x00";

// ---------- Zero in the middle (UTF-32 little) — OUT
pub(crate) const UTF_32_FIXED_FORCE_ZERO_LITTLE_ZERO_IN_MIDDLE_OUT:
    &[u8; UTF_32_FIXED_LENGTH_DATA] =
    b"v\x00\x00\x00a\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";
pub(crate) const UTF_32_FIXED_ALLOW_NO_ZERO_LITTLE_ZERO_IN_MIDDLE_OUT:
    &[u8; UTF_32_FIXED_LENGTH_DATA] =
    b"v\x00\x00\x00a\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";

// ---------- Zero in the middle (UTF-8 big) — IN
pub(crate) const UTF_8_FIXED_FORCE_ZERO_BIG_ZERO_IN_MIDDLE_IN:
    &[u8; UTF_8_FIXED_LENGTH_DATA] = b"va\x00id";
pub(crate) const UTF_8_FIXED_ALLOW_NO_ZERO_BIG_ZERO_IN_MIDDLE_IN:
    &[u8; UTF_8_FIXED_LENGTH_DATA] = b"va\x00id";

// ---------- Zero in the middle (UTF-8 big) — OUT
pub(crate) const UTF_8_FIXED_FORCE_ZERO_BIG_ZERO_IN_MIDDLE_OUT:
    &[u8; UTF_8_FIXED_LENGTH_DATA] = b"va\x00\x00\x00";
pub(crate) const UTF_8_FIXED_ALLOW_NO_ZERO_BIG_ZERO_IN_MIDDLE_OUT:
    &[u8; UTF_8_FIXED_LENGTH_DATA] = b"va\x00\x00\x00";

// ---------- Zero in the middle (UTF-16 big) — IN
pub(crate) const UTF_16_FIXED_FORCE_ZERO_BIG_ZERO_IN_MIDDLE_IN:
    &[u8; UTF_16_FIXED_LENGTH_DATA] = b"\x00v\x00a\x00\x00\x00i\x00d";
pub(crate) const UTF_16_FIXED_ALLOW_NO_ZERO_BIG_ZERO_IN_MIDDLE_IN:
    &[u8; UTF_16_FIXED_LENGTH_DATA] = b"\x00v\x00a\x00\x00\x00i\x00d";

// ---------- Zero in the middle (UTF-16 big) — OUT
pub(crate) const UTF_16_FIXED_FORCE_ZERO_BIG_ZERO_IN_MIDDLE_OUT:
    &[u8; UTF_16_FIXED_LENGTH_DATA] = b"\x00v\x00a\x00\x00\x00\x00\x00\x00";
pub(crate) const UTF_16_FIXED_ALLOW_NO_ZERO_BIG_ZERO_IN_MIDDLE_OUT:
    &[u8; UTF_16_FIXED_LENGTH_DATA] = b"\x00v\x00a\x00\x00\x00\x00\x00\x00";

// ---------- Zero in the middle (UTF-32 big) — IN
pub(crate) const UTF_32_FIXED_FORCE_ZERO_BIG_ZERO_IN_MIDDLE_IN:
    &[u8; UTF_32_FIXED_LENGTH_DATA] =
    b"\x00\x00\x00v\x00\x00\x00a\x00\x00\x00\x00\x00\x00\x00i\x00\x00\x00d";
pub(crate) const UTF_32_FIXED_ALLOW_NO_ZERO_BIG_ZERO_IN_MIDDLE_IN:
    &[u8; UTF_32_FIXED_LENGTH_DATA] =
    b"\x00\x00\x00v\x00\x00\x00a\x00\x00\x00\x00\x00\x00\x00i\x00\x00\x00d";

// ---------- Zero in the middle (UTF-32 big) — OUT
pub(crate) const UTF_32_FIXED_FORCE_ZERO_BIG_ZERO_IN_MIDDLE_OUT:
    &[u8; UTF_32_FIXED_LENGTH_DATA] =
    b"\x00\x00\x00v\x00\x00\x00a\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";
pub(crate) const UTF_32_FIXED_ALLOW_NO_ZERO_BIG_ZERO_IN_MIDDLE_OUT:
    &[u8; UTF_32_FIXED_LENGTH_DATA] =
    b"\x00\x00\x00v\x00\x00\x00a\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00";

// -------------%<-------------%<------|------->%------------->%-------------

// ------------------------------------
// ------ No null in the buffer -------
// ------------------------------------
//
// * `StringLayout::LengthPrefix` — is required, same as "valid" case
// * `StringLayout::FixedLength(allow_no_null = false)` — not allowed
// * `StringLayout::ZeroEnded` — not allowed

// ---------- No null in the buffer (str)
pub(crate) const FIXED_ALLOW_NO_ZERO_NO_NULL_INSIDE_STR: &str = "valid";

// ---------- No null in the buffer (UTF-8 little)
pub(crate) const UTF_8_FIXED_ALLOW_NO_ZERO_LITTLE_NO_NULL_INSIDE_IN:
    &[u8; UTF_8_FIXED_LENGTH_DATA] = b"valid";

// ---------- No null in the buffer (UTF-16 little)
pub(crate) const UTF_16_FIXED_ALLOW_NO_ZERO_LITTLE_NO_NULL_INSIDE_IN:
    &[u8; UTF_16_FIXED_LENGTH_DATA] = b"v\x00a\x00l\x00i\x00d\x00";

// ---------- No null in the buffer (UTF-32 little)
pub(crate) const UTF_32_FIXED_ALLOW_NO_ZERO_LITTLE_NO_NULL_INSIDE_IN:
    &[u8; UTF_32_FIXED_LENGTH_DATA] =
    b"v\x00\x00\x00a\x00\x00\x00l\x00\x00\x00i\x00\x00\x00d\x00\x00\x00";

// ---------- No null in the buffer (UTF-8 big)
pub(crate) const UTF_8_FIXED_ALLOW_NO_ZERO_BIG_NO_NULL_INSIDE_IN:
    &[u8; UTF_8_FIXED_LENGTH_DATA] = b"valid";

// ---------- No null in the buffer (UTF-16 big)
pub(crate) const UTF_16_FIXED_ALLOW_NO_ZERO_BIG_NO_NULL_INSIDE_IN:
    &[u8; UTF_16_FIXED_LENGTH_DATA] = b"\x00v\x00a\x00l\x00i\x00d";

// ---------- No null in the buffer (UTF-32 big)
pub(crate) const UTF_32_FIXED_ALLOW_NO_ZERO_BIG_NO_NULL_INSIDE_IN:
    &[u8; UTF_32_FIXED_LENGTH_DATA] =
    b"\x00\x00\x00v\x00\x00\x00a\x00\x00\x00l\x00\x00\x00i\x00\x00\x00d";

// -------------%<-------------%<------|------->%------------->%-------------

// --------------------------------------
// ------ 255 bytes in the buffer -------
// --------------------------------------
//
// Same as valid case but algorithm boundary for `StringLayout::PrefixLength(Size::U8)`
//
// * `StringLayout::FixedLength` — buffer is too big
// * `StringLayout::ZeroEnded` — same as "valid" case
// * `StringLayout::PrefixLength` (not u8) — same as "valid" case

/// Raw data size
const UTF_8_255_DATA: usize = 255;
/// Raw data size
const UTF_16_255_DATA: usize = 255 * 2;
const UTF_32_255_DATA: usize = 255 * 4;

pub(crate) const PREFIX_U8_LEN_255_STR: &str = include_str!("../../data/data_255");

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

// -------------%<-------------%<------|------->%------------->%-------------
