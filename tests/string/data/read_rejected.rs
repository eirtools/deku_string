use super::layouts::FIXED_LENGTH;

/// Raw data size
const UTF_8_FIXED_LENGTH_DATA: usize = FIXED_LENGTH;
/// Raw data size
const UTF_16_FIXED_LENGTH_DATA: usize = FIXED_LENGTH * 2;
const UTF_32_FIXED_LENGTH_DATA: usize = FIXED_LENGTH * 4;

// -------------------------------------------------
// ---------- UTF encoding is invalid --------------
// -------------------------------------------------
//
// Error: parse

// ---------- UTF encoding is invalid (UTF-8 little)
pub(crate) const UTF_8_FIXED_FORCE_ZERO_LITTLE_INVALID_UTF:
    &[u8; UTF_8_FIXED_LENGTH_DATA] = b" \xe2\x28\xa1\x00";
pub(crate) const UTF_8_FIXED_ALLOW_NO_ZERO_LITTLE_INVALID_UTF:
    &[u8; UTF_8_FIXED_LENGTH_DATA] = b" \xe2\x28\xa1\x00";
pub(crate) const UTF_8_PREFIX_U8_LITTLE_INVALID_UTF: &[u8; 8] = b"\x07utf-\xe2\x28\xa1";
pub(crate) const UTF_8_PREFIX_U16_LITTLE_INVALID_UTF: &[u8; 9] =
    b"\x07\x00utf-\xe2\x28\xa1";
pub(crate) const UTF_8_PREFIX_U32_LITTLE_INVALID_UTF: &[u8; 11] =
    b"\x07\x00\x00\x00utf-\xe2\x28\xa1";
pub(crate) const UTF_8_PREFIX_U32_7BIT_LITTLE_INVALID_UTF: &[u8; 8] =
    b"\x07utf-\xe2\x28\xa1";
pub(crate) const UTF_8_ZERO_ENDED_LITTLE_INVALID_UTF: &[u8; UTF_8_FIXED_LENGTH_DATA] =
    b" \xe2\x28\xa1\x00";

// ---------- UTF encoding is invalid (UTF-16 little)
pub(crate) const UTF_16_FIXED_FORCE_ZERO_LITTLE_INVALID_UTF:
    &[u8; UTF_16_FIXED_LENGTH_DATA] = b" \x00\xdc\xdc\xdc\xdc\xdc\xdc\x00\x00";
pub(crate) const UTF_16_FIXED_ALLOW_NO_ZERO_LITTLE_INVALID_UTF:
    &[u8; UTF_16_FIXED_LENGTH_DATA] = b" \x00\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc";
pub(crate) const UTF_16_PREFIX_U8_LITTLE_INVALID_UTF: &[u8; 3] = b"\x01\xdc\xdc";
pub(crate) const UTF_16_PREFIX_U16_LITTLE_INVALID_UTF: &[u8; 4] = b"\x01\x00\xdc\xdc";
pub(crate) const UTF_16_PREFIX_U32_LITTLE_INVALID_UTF: &[u8; 6] =
    b"\x01\x00\x00\x00\xdc\xdc";
pub(crate) const UTF_16_PREFIX_U32_7BIT_LITTLE_INVALID_UTF: &[u8; 3] = b"\x01\xdc\xdc";
pub(crate) const UTF_16_ZERO_ENDED_LITTLE_INVALID_UTF: &[u8; UTF_16_FIXED_LENGTH_DATA] =
    b"\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\x00\x00";

// ---------- UTF encoding is invalid (UTF-32 little)
pub(crate) const UTF_32_FIXED_FORCE_ZERO_LITTLE_INVALID_UTF:
    &[u8; UTF_32_FIXED_LENGTH_DATA] =
    b" \x00\x00\x00\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\x00\x00\x00\x00";
pub(crate) const UTF_32_FIXED_ALLOW_NO_ZERO_LITTLE_INVALID_UTF:
    &[u8; UTF_32_FIXED_LENGTH_DATA] =
    b" \x00\x00\x00\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc";
pub(crate) const UTF_32_PREFIX_U8_LITTLE_INVALID_UTF: &[u8; 5] =
    b"\x01\xdc\xdc\xdc\xdc";
pub(crate) const UTF_32_PREFIX_U16_LITTLE_INVALID_UTF: &[u8; 6] =
    b"\x01\x00\xdc\xdc\xdc\xdc";
pub(crate) const UTF_32_PREFIX_U32_LITTLE_INVALID_UTF: &[u8; 8] =
    b"\x01\x00\x00\x00\xdc\xdc\xdc\xdc";
pub(crate) const UTF_32_PREFIX_U32_7BIT_LITTLE_INVALID_UTF: &[u8; 5] =
    b"\x01\xdc\xdc\xdc\xdc";
pub(crate) const UTF_32_ZERO_ENDED_LITTLE_INVALID_UTF: &[u8; UTF_32_FIXED_LENGTH_DATA] =
    b"\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\x00\x00\x00\x00\x00";

// ---------- UTF encoding is invalid (UTF-8 big)
pub(crate) const UTF_8_FIXED_FORCE_ZERO_BIG_INVALID_UTF: &[u8;
     UTF_8_FIXED_LENGTH_DATA] = b" \xe2\x28\xa1\x00";
pub(crate) const UTF_8_FIXED_ALLOW_NO_ZERO_BIG_INVALID_UTF:
    &[u8; UTF_8_FIXED_LENGTH_DATA] = b" \xe2\x28\xa1\x00";
pub(crate) const UTF_8_PREFIX_U8_BIG_INVALID_UTF: &[u8; 8] = b"\x07utf-\xe2\x28\xa1";
pub(crate) const UTF_8_PREFIX_U16_BIG_INVALID_UTF: &[u8; 9] =
    b"\x00\x07utf-\xe2\x28\xa1";
pub(crate) const UTF_8_PREFIX_U32_BIG_INVALID_UTF: &[u8; 11] =
    b"\x00\x00\x00\x07utf-\xe2\x28\xa1";
pub(crate) const UTF_8_PREFIX_U32_7BIT_BIG_INVALID_UTF: &[u8; 8] =
    b"\x07utf-\xe2\x28\xa1";
pub(crate) const UTF_8_ZERO_ENDED_BIG_INVALID_UTF: &[u8; UTF_8_FIXED_LENGTH_DATA] =
    b" \xe2\x28\xa1\x00";

// ---------- UTF encoding is invalid (UTF-16 big)
pub(crate) const UTF_16_FIXED_FORCE_ZERO_BIG_INVALID_UTF:
    &[u8; UTF_16_FIXED_LENGTH_DATA] = b"\x00 \xdc\xdc\xdc\xdc\xdc\xdc\x00\x00";
pub(crate) const UTF_16_FIXED_ALLOW_NO_ZERO_BIG_INVALID_UTF:
    &[u8; UTF_16_FIXED_LENGTH_DATA] = b"\x00 \xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc";
pub(crate) const UTF_16_PREFIX_U8_BIG_INVALID_UTF: &[u8; 3] = b"\x01\xdc\xdc";
pub(crate) const UTF_16_PREFIX_U16_BIG_INVALID_UTF: &[u8; 4] = b"\x00\x01\xdc\xdc";
pub(crate) const UTF_16_PREFIX_U32_BIG_INVALID_UTF: &[u8; 6] =
    b"\x00\x00\x00\x01\xdc\xdc";
pub(crate) const UTF_16_PREFIX_U32_7BIT_BIG_INVALID_UTF: &[u8; 3] = b"\x01\xdc\xdc";
pub(crate) const UTF_16_ZERO_ENDED_BIG_INVALID_UTF: &[u8; UTF_16_FIXED_LENGTH_DATA] =
    b"\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\x00\x00";

// ---------- UTF encoding is invalid (UTF-32 big)
pub(crate) const UTF_32_FIXED_FORCE_ZERO_BIG_INVALID_UTF:
    &[u8; UTF_32_FIXED_LENGTH_DATA] =
    b"\x00\x00\x00 \xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\x00\x00\x00\x00";
pub(crate) const UTF_32_FIXED_ALLOW_NO_ZERO_BIG_INVALID_UTF:
    &[u8; UTF_32_FIXED_LENGTH_DATA] =
    b"\x00\x00\x00 \xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc";
pub(crate) const UTF_32_PREFIX_U8_BIG_INVALID_UTF: &[u8; 5] = b"\x01\xdc\xdc\xdc\xdc";
pub(crate) const UTF_32_PREFIX_U16_BIG_INVALID_UTF: &[u8; 6] =
    b"\x00\x01\xdc\xdc\xdc\xdc";
pub(crate) const UTF_32_PREFIX_U32_BIG_INVALID_UTF: &[u8; 8] =
    b"\x00\x00\x00\x01\xdc\xdc\xdc\xdc";
pub(crate) const UTF_32_PREFIX_U32_7BIT_BIG_INVALID_UTF: &[u8; 5] =
    b"\x01\xdc\xdc\xdc\xdc";
pub(crate) const UTF_32_ZERO_ENDED_BIG_INVALID_UTF: &[u8; UTF_32_FIXED_LENGTH_DATA] =
    b"\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\x00\x00\x00\x00\x00";

// -------------%<-------------%<------|------->%------------->%-------------

// -------------------------------------------------
// ---------- No null while expected --------------
// -------------------------------------------------
//
// Error: assertion
//
// * `StringLayout::FixedLength(allow_no_null = true)` — is accepted
// * `StringLayout::LengthPrefix` — is accepted

// ---------- No null while expected (UTF-8 little)
pub(crate) const UTF_8_FIXED_FORCE_ZERO_LITTLE_NO_NULL_INSIDE:
    &[u8; UTF_8_FIXED_LENGTH_DATA] = b"zero?";
pub(crate) const UTF_8_ZERO_ENDED_LITTLE_NO_NULL_INSIDE: &[u8;
     UTF_8_FIXED_LENGTH_DATA] = b"zero?";

// ---------- No null while expected (UTF-16 little)
pub(crate) const UTF_16_FIXED_FORCE_ZERO_LITTLE_NO_NULL_INSIDE:
    &[u8; UTF_16_FIXED_LENGTH_DATA] = b"z\x00e\x00r\x00o\x00?\x00";
pub(crate) const UTF_16_ZERO_ENDED_LITTLE_NO_NULL_INSIDE: &[u8; 10] =
    b"z\x00e\x00r\x00o\x00?\x00";

// ---------- No null while expected (UTF-32 little)
pub(crate) const UTF_32_FIXED_FORCE_ZERO_LITTLE_NO_NULL_INSIDE:
    &[u8; UTF_32_FIXED_LENGTH_DATA] =
    b"z\x00\x00\x00e\x00\x00\x00r\x00\x00\x00o\x00\x00\x00?\x00\x00\x00";
pub(crate) const UTF_32_ZERO_ENDED_LITTLE_NO_NULL_INSIDE:
    &[u8; UTF_32_FIXED_LENGTH_DATA] =
    b"z\x00\x00\x00e\x00\x00\x00r\x00\x00\x00o\x00\x00\x00?\x00\x00\x00";

// ---------- No null while expected (UTF-8 big)
pub(crate) const UTF_8_FIXED_FORCE_ZERO_BIG_NO_NULL_INSIDE:
    &[u8; UTF_8_FIXED_LENGTH_DATA] = b"zero?";
pub(crate) const UTF_8_ZERO_ENDED_BIG_NO_NULL_INSIDE: &[u8; UTF_8_FIXED_LENGTH_DATA] =
    b"zero?";

// ---------- No null while expected (UTF-16 big)
pub(crate) const UTF_16_FIXED_FORCE_ZERO_BIG_NO_NULL_INSIDE:
    &[u8; UTF_16_FIXED_LENGTH_DATA] = b"\x00z\x00e\x00r\x00o\x00?";
pub(crate) const UTF_16_ZERO_ENDED_BIG_NO_NULL_INSIDE: &[u8; 10] =
    b"\x00z\x00e\x00r\x00o\x00?";

// ---------- No null while expected (UTF-32 big)
pub(crate) const UTF_32_FIXED_FORCE_ZERO_BIG_NO_NULL_INSIDE:
    &[u8; UTF_32_FIXED_LENGTH_DATA] =
    b"\x00\x00\x00z\x00\x00\x00e\x00\x00\x00r\x00\x00\x00o\x00\x00\x00?";
pub(crate) const UTF_32_ZERO_ENDED_BIG_NO_NULL_INSIDE: &[u8; UTF_32_FIXED_LENGTH_DATA] =
    b"\x00\x00\x00z\x00\x00\x00e\x00\x00\x00r\x00\x00\x00o\x00\x00\x00?";

// -------------%<-------------%<------|------->%------------->%-------------

// --------------------------------------------
// ---------- Zero in the middle --------------
// --------------------------------------------
//
// Error: assertion
//
// * `StringLayout::FixedLength` — is accepted
// * `StringLayout::ZeroEnded` — is end of data

// ---------- Zero in the middle (UTF-8 little)
pub(crate) const UTF_8_PREFIX_U8_LITTLE_ZERO_IN_MIDDLE: &[u8; 3] = b"\x02\x00a";
pub(crate) const UTF_8_PREFIX_U16_LITTLE_ZERO_IN_MIDDLE: &[u8; 4] = b"\x02\x00\x00a";
pub(crate) const UTF_8_PREFIX_U32_LITTLE_ZERO_IN_MIDDLE: &[u8; 6] =
    b"\x02\x00\x00\x00\x00a";
pub(crate) const UTF_8_PREFIX_U32_7BIT_LITTLE_ZERO_IN_MIDDLE: &[u8; 3] = b"\x02\x00a";

// ---------- Zero in the middle (UTF-16 little)
pub(crate) const UTF_16_PREFIX_U8_LITTLE_ZERO_IN_MIDDLE: &[u8; 5] =
    b"\x02\x00\x00a\x00";
pub(crate) const UTF_16_PREFIX_U16_LITTLE_ZERO_IN_MIDDLE: &[u8; 6] =
    b"\x02\x00\x00\x00a\x00";
pub(crate) const UTF_16_PREFIX_U32_LITTLE_ZERO_IN_MIDDLE: &[u8; 8] =
    b"\x02\x00\x00\x00\x00\x00a\x00";
pub(crate) const UTF_16_PREFIX_U32_7BIT_LITTLE_ZERO_IN_MIDDLE: &[u8; 5] =
    b"\x02\x00\x00a\x00";

// ---------- Zero in the middle (UTF-32 little)
pub(crate) const UTF_32_PREFIX_U8_LITTLE_ZERO_IN_MIDDLE: &[u8; 9] =
    b"\x02\x00\x00\x00a\x00\x00\x00\x00";
pub(crate) const UTF_32_PREFIX_U16_LITTLE_ZERO_IN_MIDDLE: &[u8; 11] =
    b"\x02\x00\x00\x00\x00\x00\x00a\x00\x00\x00";
pub(crate) const UTF_32_PREFIX_U32_LITTLE_ZERO_IN_MIDDLE: &[u8; 12] =
    b"\x02\x00\x00\x00a\x00\x00\x00\x00\x00\x00\x00";
pub(crate) const UTF_32_PREFIX_U32_7BIT_LITTLE_ZERO_IN_MIDDLE: &[u8; 9] =
    b"\x02a\x00\x00\x00\x00\x00\x00\x00";

// ---------- Zero in the middle (UTF-8 big)
pub(crate) const UTF_8_PREFIX_U8_BIG_ZERO_IN_MIDDLE: &[u8; 3] = b"\x02\x00a";
pub(crate) const UTF_8_PREFIX_U16_BIG_ZERO_IN_MIDDLE: &[u8; 4] = b"\x00\x02\x00a";
pub(crate) const UTF_8_PREFIX_U32_BIG_ZERO_IN_MIDDLE: &[u8; 6] =
    b"\x00\x00\x00\x02\x00a";
pub(crate) const UTF_8_PREFIX_U32_7BIT_BIG_ZERO_IN_MIDDLE: &[u8; 3] = b"\x02\x00a";

// ---------- Zero in the middle (UTF-16 big)
pub(crate) const UTF_16_PREFIX_U8_BIG_ZERO_IN_MIDDLE: &[u8; 5] = b"\x02\x00\x00\x00a";
pub(crate) const UTF_16_PREFIX_U16_BIG_ZERO_IN_MIDDLE: &[u8; 6] =
    b"\x00\x02\x00\x00\x00a";
pub(crate) const UTF_16_PREFIX_U32_BIG_ZERO_IN_MIDDLE: &[u8; 8] =
    b"\x00\x00\x00\x02\x00\x00\x00a";
pub(crate) const UTF_16_PREFIX_U32_7BIT_BIG_ZERO_IN_MIDDLE: &[u8; 5] =
    b"\x02\x00\x00\x00a";

// ---------- Zero in the middle (UTF-32 big)
pub(crate) const UTF_32_PREFIX_U8_BIG_ZERO_IN_MIDDLE: &[u8; 9] =
    b"\x02\x00\x00\x00\x00\x00\x00\x00a";
pub(crate) const UTF_32_PREFIX_U16_BIG_ZERO_IN_MIDDLE: &[u8; 10] =
    b"\x00\x02\x00\x00\x00a\x00\x00\x00\x00";
pub(crate) const UTF_32_PREFIX_U32_BIG_ZERO_IN_MIDDLE: &[u8; 12] =
    b"\x00\x00\x00\x02\x00\x00\x00a\x00\x00\x00\x00";
pub(crate) const UTF_32_PREFIX_U32_7BIT_BIG_ZERO_IN_MIDDLE: &[u8; 9] =
    b"\x02\x00\x00\x00a\x00\x00\x00\x00";

// -------------%<-------------%<------|------->%------------->%-------------

// --------------------------------------------
// ---------- Short length, no data -----------
// --------------------------------------------
//
// Error: incomplete
//
// * `StringLayout::FixedLength` — length is specified in format
// * `StringLayout::ZeroEnded` — length is dynamic

// ---------- Short length, no data (UTF-8 little)
pub(crate) const UTF_8_PREFIX_U8_LITTLE_SHORT_LEN: &[u8; 1] = b"\x01";
pub(crate) const UTF_8_PREFIX_U16_LITTLE_SHORT_LEN: &[u8; 2] = b"\x01\x00";
pub(crate) const UTF_8_PREFIX_U32_LITTLE_SHORT_LEN: &[u8; 4] = b"\x01\x00\x00\x00";
pub(crate) const UTF_8_PREFIX_U32_7BIT_LITTLE_SHORT_LEN: &[u8; 1] = b"\x01";

// ---------- Short length, no data (UTF-16 little)
pub(crate) const UTF_16_PREFIX_U8_LITTLE_SHORT_LEN: &[u8; 1] = b"\x01";
pub(crate) const UTF_16_PREFIX_U16_LITTLE_SHORT_LEN: &[u8; 2] = b"\x01\x00";
pub(crate) const UTF_16_PREFIX_U32_LITTLE_SHORT_LEN: &[u8; 4] = b"\x01\x00\x00\x00";
pub(crate) const UTF_16_PREFIX_U32_7BIT_LITTLE_SHORT_LEN: &[u8; 1] = b"\x01";

// ---------- Short length, no data (UTF-32 little)
pub(crate) const UTF_32_PREFIX_U8_LITTLE_SHORT_LEN: &[u8; 1] = b"\x01";
pub(crate) const UTF_32_PREFIX_U16_LITTLE_SHORT_LEN: &[u8; 2] = b"\x01\x00";
pub(crate) const UTF_32_PREFIX_U32_LITTLE_SHORT_LEN: &[u8; 4] = b"\x01\x00\x00\x00";
pub(crate) const UTF_32_PREFIX_U32_7BIT_LITTLE_SHORT_LEN: &[u8; 1] = b"\x01";

// ---------- Short length, no data (UTF-8 big)
pub(crate) const UTF_8_PREFIX_U8_BIG_SHORT_LEN: &[u8; 1] = b"\x01";
pub(crate) const UTF_8_PREFIX_U16_BIG_SHORT_LEN: &[u8; 2] = b"\x01\x00";
pub(crate) const UTF_8_PREFIX_U32_BIG_SHORT_LEN: &[u8; 4] = b"\x01\x00\x00\x00";
pub(crate) const UTF_8_PREFIX_U32_7BIT_BIG_SHORT_LEN: &[u8; 1] = b"\x01";

// ---------- Short length, no data (UTF-16 big)
pub(crate) const UTF_16_PREFIX_U8_BIG_SHORT_LEN: &[u8; 1] = b"\x01";
pub(crate) const UTF_16_PREFIX_U16_BIG_SHORT_LEN: &[u8; 2] = b"\x00\x01";
pub(crate) const UTF_16_PREFIX_U32_BIG_SHORT_LEN: &[u8; 4] = b"\x01\x00\x00\x00";
pub(crate) const UTF_16_PREFIX_U32_7BIT_BIG_SHORT_LEN: &[u8; 1] = b"\x01";

// ---------- Short length, no data (UTF-32 big)
pub(crate) const UTF_32_PREFIX_U8_BIG_SHORT_LEN: &[u8; 1] = b"\x01";
pub(crate) const UTF_32_PREFIX_U16_BIG_SHORT_LEN: &[u8; 2] = b"\x00\x01";
pub(crate) const UTF_32_PREFIX_U32_BIG_SHORT_LEN: &[u8; 4] = b"\x01\x00\x00\x00";
pub(crate) const UTF_32_PREFIX_U32_7BIT_BIG_SHORT_LEN: &[u8; 1] = b"\x01";

// -------------%<-------------%<------|------->%------------->%-------------

// ----------------------------------------------
// ---------- Good length, short data -----------
// ----------------------------------------------
//
// Error: incomplete
//
// * `StringLayout::FixedLength` — length is specified in format
// * `StringLayout::ZeroEnded` — length is dynamic

// ---------- Short length, no data (UTF-8 little)
pub(crate) const UTF_8_PREFIX_U8_LITTLE_SHORT_DATA: &[u8; 2] = b"\x02a";
pub(crate) const UTF_8_PREFIX_U16_LITTLE_SHORT_DATA: &[u8; 3] = b"\x02\x00a";
pub(crate) const UTF_8_PREFIX_U32_LITTLE_SHORT_DATA: &[u8; 5] = b"\x02\x00\x00\x00a";
pub(crate) const UTF_8_PREFIX_U32_7BIT_LITTLE_SHORT_DATA: &[u8; 2] = b"\x02a";

// ---------- Short length, no data (UTF-16 little)
pub(crate) const UTF_16_PREFIX_U8_LITTLE_SHORT_DATA: &[u8; 3] = b"\x02a\x00";
pub(crate) const UTF_16_PREFIX_U16_LITTLE_SHORT_DATA: &[u8; 4] = b"\x02\x00a\x00";
pub(crate) const UTF_16_PREFIX_U32_LITTLE_SHORT_DATA: &[u8; 6] =
    b"\x02\x00\x00\x00a\x00";
pub(crate) const UTF_16_PREFIX_U32_7BIT_LITTLE_SHORT_DATA: &[u8; 3] = b"\x02a\x00";

// ---------- Short length, no data (UTF-32 little)
pub(crate) const UTF_32_PREFIX_U8_LITTLE_SHORT_DATA: &[u8; 5] = b"\x02a\x00\x00\x00";
pub(crate) const UTF_32_PREFIX_U16_LITTLE_SHORT_DATA: &[u8; 6] =
    b"\x02\x00a\x00\x00\x00";
pub(crate) const UTF_32_PREFIX_U32_LITTLE_SHORT_DATA: &[u8; 8] =
    b"\x02\x00\x00\x00a\x00\x00\x00";
pub(crate) const UTF_32_PREFIX_U32_7BIT_LITTLE_SHORT_DATA: &[u8; 5] =
    b"\x02a\x00\x00\x00";

// ---------- Short length, no data (UTF-8 big)
pub(crate) const UTF_8_PREFIX_U8_BIG_SHORT_DATA: &[u8; 2] = b"\x02a";
pub(crate) const UTF_8_PREFIX_U16_BIG_SHORT_DATA: &[u8; 3] = b"\x02\x00a";
pub(crate) const UTF_8_PREFIX_U32_BIG_SHORT_DATA: &[u8; 5] = b"\x02\x00\x00\x00a";
pub(crate) const UTF_8_PREFIX_U32_7BIT_BIG_SHORT_DATA: &[u8; 2] = b"\x02a";

// ---------- Short length, no data (UTF-16 big)
pub(crate) const UTF_16_PREFIX_U8_BIG_SHORT_DATA: &[u8; 3] = b"\x02\x00a";
pub(crate) const UTF_16_PREFIX_U16_BIG_SHORT_DATA: &[u8; 4] = b"\x00\x02\x00a";
pub(crate) const UTF_16_PREFIX_U32_BIG_SHORT_DATA: &[u8; 6] = b"\x00\x00\x00\x02\x00a";
pub(crate) const UTF_16_PREFIX_U32_7BIT_BIG_SHORT_DATA: &[u8; 3] = b"\x02\x00a";

// ---------- Short length, no data (UTF-32 big)
pub(crate) const UTF_32_PREFIX_U8_BIG_SHORT_DATA: &[u8; 5] = b"\x02\x00\x00\x00a";
pub(crate) const UTF_32_PREFIX_U16_BIG_SHORT_DATA: &[u8; 6] = b"\x00\x02\x00\x00\x00a";
pub(crate) const UTF_32_PREFIX_U32_BIG_SHORT_DATA: &[u8; 8] =
    b"\x00\x00\x00\x02\x00\x00\x00a";
pub(crate) const UTF_32_PREFIX_U32_7BIT_BIG_SHORT_DATA: &[u8; 5] = b"\x02\x00\x00\x00a";

// -------------%<-------------%<------|------->%------------->%-------------

// ------------------------------------------
// ---------- Buffer is too small -----------
// ------------------------------------------
//
// Error: incomplete

// ---------- Buffer is too small (UTF-8 little)
pub(crate) const UTF_8_FIXED_FORCE_ZERO_LITTLE_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_8_FIXED_ALLOW_NO_ZERO_LITTLE_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_8_PREFIX_U8_LITTLE_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_8_PREFIX_U16_LITTLE_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_8_PREFIX_U32_LITTLE_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_8_PREFIX_U32_7BIT_LITTLE_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_8_ZERO_ENDED_LITTLE_SMALL_BUFFER: &[u8; 0] = &[];

// ---------- Buffer is too small (UTF-16 little)
pub(crate) const UTF_16_FIXED_FORCE_ZERO_LITTLE_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_16_FIXED_ALLOW_NO_ZERO_LITTLE_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_16_PREFIX_U8_LITTLE_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_16_PREFIX_U16_LITTLE_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_16_PREFIX_U32_LITTLE_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_16_PREFIX_U32_7BIT_LITTLE_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_16_ZERO_ENDED_LITTLE_SMALL_BUFFER: &[u8; 0] = &[];

// ---------- Buffer is too small (UTF-32 little)
pub(crate) const UTF_32_FIXED_FORCE_ZERO_LITTLE_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_32_FIXED_ALLOW_NO_ZERO_LITTLE_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_32_PREFIX_U8_LITTLE_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_32_PREFIX_U16_LITTLE_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_32_PREFIX_U32_LITTLE_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_32_PREFIX_U32_7BIT_LITTLE_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_32_ZERO_ENDED_LITTLE_SMALL_BUFFER: &[u8; 0] = &[];

// ---------- Buffer is too small (UTF-8 big)
pub(crate) const UTF_8_FIXED_FORCE_ZERO_BIG_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_8_FIXED_ALLOW_NO_ZERO_BIG_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_8_PREFIX_U8_BIG_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_8_PREFIX_U16_BIG_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_8_PREFIX_U32_BIG_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_8_PREFIX_U32_7BIT_BIG_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_8_ZERO_ENDED_BIG_SMALL_BUFFER: &[u8; 0] = &[];

// ---------- Buffer is too small (UTF-16 big)
pub(crate) const UTF_16_FIXED_FORCE_ZERO_BIG_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_16_FIXED_ALLOW_NO_ZERO_BIG_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_16_PREFIX_U8_BIG_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_16_PREFIX_U16_BIG_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_16_PREFIX_U32_BIG_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_16_PREFIX_U32_7BIT_BIG_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_16_ZERO_ENDED_BIG_SMALL_BUFFER: &[u8; 0] = &[];
// ---------- Buffer is too small  (UTF-32 little)
pub(crate) const UTF_32_FIXED_FORCE_ZERO_BIG_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_32_FIXED_ALLOW_NO_ZERO_BIG_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_32_PREFIX_U8_BIG_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_32_PREFIX_U16_BIG_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_32_PREFIX_U32_BIG_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_32_PREFIX_U32_7BIT_BIG_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_32_ZERO_ENDED_BIG_SMALL_BUFFER: &[u8; 0] = &[];

// -------------%<-------------%<------|------->%------------->%-------------

// -------------------------------------------------
// ---------- Invalid length encoding --------------
// -------------------------------------------------
//
// Error: assertion
//
// * `StringLayout::FixedLength` — no length prefix
// * `StringLayout::ZeroEnded` — no length prefix
// * `StringLayout::LengthPrefix` (other than 32-7bit) — not applicable

// ---------- Invalid length encoding (UTF-8 little)
pub(crate) const UTF_8_PREFIX_U32_7BIT_LITTLE_INVALID_SIZE_ENCODING: &[u8; 5] =
    b"\xff\xff\xff\xff\xff";

// ---------- Invalid length encoding (UTF-16 little)
pub(crate) const UTF_16_PREFIX_U32_7BIT_LITTLE_INVALID_SIZE_ENCODING: &[u8; 5] =
    b"\xff\xff\xff\xff\xff";

// ---------- Invalid length encoding (UTF-32 little)
pub(crate) const UTF_32_PREFIX_U32_7BIT_LITTLE_INVALID_SIZE_ENCODING: &[u8; 5] =
    b"\xff\xff\xff\xff\xff";

// ---------- Invalid length encoding (UTF-8 big)
pub(crate) const UTF_8_PREFIX_U32_7BIT_BIG_INVALID_SIZE_ENCODING: &[u8; 5] =
    b"\xff\xff\xff\xff\xff";

// ---------- Invalid length encoding (UTF-16 big)
pub(crate) const UTF_16_PREFIX_U32_7BIT_BIG_INVALID_SIZE_ENCODING: &[u8; 5] =
    b"\xff\xff\xff\xff\xff";

// ---------- Invalid length encoding (UTF-32 big)
pub(crate) const UTF_32_PREFIX_U32_7BIT_BIG_INVALID_SIZE_ENCODING: &[u8; 5] =
    b"\xff\xff\xff\xff\xff";

// -------------%<-------------%<------|------->%------------->%-------------
