use super::layouts::FIXED_LENGTH;

/// Raw data size
const UTF_8_FIXED_LENGTH_DATA: usize = FIXED_LENGTH;
/// Raw data size
const UTF_16_FIXED_LENGTH_DATA: usize = FIXED_LENGTH * 2;
const UTF_32_FIXED_LENGTH_DATA: usize = FIXED_LENGTH * 4;

// ---------- UTF encoding is invalid
// error: parse
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

// ---------- No zero while expected
// error: assertion
pub(crate) const UTF_8_FIXED_FORCE_ZERO_LITTLE_NO_ZERO_INSIDE:
    &[u8; UTF_8_FIXED_LENGTH_DATA] = b"zero?";
// pub(crate) const FIXED_ALLOW_NO_ZERO_LITTLE_NO_ZERO_INSIDE — is accepted
// pub(crate) const PREFIX_U8_LITTLE_NO_ZERO_INSIDE: &[u8; 3] — is accepted
// pub(crate) const PREFIX_U16_LITTLE_NO_ZERO_INSIDE: &[u8; 4] — is accepted
// pub(crate) const PREFIX_U32_LITTLE_NO_ZERO_INSIDE: &[u8; 6] — is accepted
// pub(crate) const ZERO_ENDED_LITTLE_NO_ZERO_INSIDE — is required

pub(crate) const UTF_8_FIXED_FORCE_ZERO_BIG_NO_ZERO_INSIDE:
    &[u8; UTF_8_FIXED_LENGTH_DATA] = b"zero?";
// pub(crate) const FIXED_ALLOW_NO_ZERO_BIG_NO_ZERO_INSIDE — is accepted
// pub(crate) const PREFIX_U8_BIG_NO_ZERO_INSIDE: &[u8; 3] — is accepted
// pub(crate) const PREFIX_U16_BIG_NO_ZERO_INSIDE: &[u8; 4] — is accepted
// pub(crate) const PREFIX_U32_BIG_NO_ZERO_INSIDE: &[u8; 6] — is accepted
// pub(crate) const ZERO_ENDED_BIG_NO_ZERO_INSIDE — is required

// ---------- Prefixed length, zero in the middle
// error: assertion
// pub(crate) const FIXED_FORCE_ZERO_LITTLE_ZERO_IN_MIDDLE — is accepted
// pub(crate) const FIXED_ALLOW_NO_ZERO_LITTLE_ZERO_IN_MIDDLE — is accepted
pub(crate) const UTF_8_PREFIX_U8_LITTLE_ZERO_IN_MIDDLE: &[u8; 3] = b"\x02\x00a";
pub(crate) const UTF_8_PREFIX_U16_LITTLE_ZERO_IN_MIDDLE: &[u8; 4] = b"\x02\x00\x00a";
pub(crate) const UTF_8_PREFIX_U32_LITTLE_ZERO_IN_MIDDLE: &[u8; 6] =
    b"\x02\x00\x00\x00\x00a";
pub(crate) const UTF_8_PREFIX_U32_7BIT_LITTLE_ZERO_IN_MIDDLE: &[u8; 3] = b"\x02\x00a";
// pub(crate) const UTF_8_ZERO_ENDED_LITTLE_ZERO_IN_MIDDLE- is accepted

// pub(crate) const FIXED_FORCE_ZERO_BIG_IN_MIDDLE — is accepted
// pub(crate) const FIXED_ALLOW_NO_ZERO_BIG_ZERO_IN_MIDDLE — is accepted
pub(crate) const UTF_8_PREFIX_U8_BIG_ZERO_IN_MIDDLE: &[u8; 3] = b"\x02\x00a";
pub(crate) const UTF_8_PREFIX_U16_BIG_ZERO_IN_MIDDLE: &[u8; 4] = b"\x00\x02\x00a";
pub(crate) const UTF_8_PREFIX_U32_BIG_ZERO_IN_MIDDLE: &[u8; 6] =
    b"\x00\x00\x00\x02\x00a";
pub(crate) const UTF_8_PREFIX_U32_7BIT_BIG_ZERO_IN_MIDDLE: &[u8; 3] = b"\x02\x00a";
// pub(crate) const ZERO_ENDED_BIG_ZERO_IN_MIDDLE — is accepted

// ---------- Pascal string (short length in data)
// error: incomplete
// pub(crate) const FIXED_FORCE_ZERO_LITTLE_SHORT_LEN — length is specified in format
// pub(crate) const FIXED_ALLOW_NO_ZERO_LITTLE_SHORT_LEN — length is specified in format
pub(crate) const UTF_8_PREFIX_U8_LITTLE_SHORT_LEN: &[u8; 1] = b"\x01";
pub(crate) const UTF_8_PREFIX_U16_LITTLE_SHORT_LEN: &[u8; 2] = b"\x01\x00";
pub(crate) const UTF_8_PREFIX_U32_LITTLE_SHORT_LEN: &[u8; 4] = b"\x01\x00\x00\x00";
pub(crate) const UTF_8_PREFIX_U32_7BIT_LITTLE_SHORT_LEN: &[u8; 1] = b"\x01";
// pub(crate) const ZERO_ENDED_LITTLE_SHORT_LEN — length is dynamic

// pub(crate) const FIXED_FORCE_ZERO_BIG_SHORT_LEN — length is specified in format
// pub(crate) const FIXED_ALLOW_NO_ZERO_BIG_SHORT_LEN — length is specified in format
pub(crate) const UTF_8_PREFIX_U8_BIG_SHORT_LEN: &[u8; 1] = b"\x01";
pub(crate) const UTF_8_PREFIX_U16_BIG_SHORT_LEN: &[u8; 2] = b"\x01\x00";
pub(crate) const UTF_8_PREFIX_U32_BIG_SHORT_LEN: &[u8; 4] = b"\x01\x00\x00\x00";
pub(crate) const UTF_8_PREFIX_U32_7BIT_BIG_SHORT_LEN: &[u8; 1] = b"\x01";
// pub(crate) const ZERO_ENDED_BIG_SHORT_LEN — length is dynamic

// ---------- Short data (short data)
// error: incomplete
// pub(crate) const FIXED_FORCE_ZERO_LITTLE_SHORT_DATA — length is specified in format
// pub(crate) const FIXED_ALLOW_NO_ZERO_LITTLE_SHORT_DATA — length is specified in format
pub(crate) const UTF_8_PREFIX_U8_LITTLE_SHORT_DATA: &[u8; 2] = b"\x02a";
pub(crate) const UTF_8_PREFIX_U16_LITTLE_SHORT_DATA: &[u8; 3] = b"\x02\x00a";
pub(crate) const UTF_8_PREFIX_U32_LITTLE_SHORT_DATA: &[u8; 5] = b"\x02\x00\x00\x00a";
pub(crate) const UTF_8_PREFIX_U32_7BIT_LITTLE_SHORT_DATA: &[u8; 2] = b"\x02a";
// pub(crate) const ZERO_ENDED_LITTLE_SHORT_DATA — length is dynamic

// pub(crate) const FIXED_FORCE_ZERO_BIG_SHORT_DATA — length is specified in format
// pub(crate) const FIXED_ALLOW_NO_ZERO_BIG_SHORT_DATA — length is specified in format
pub(crate) const UTF_8_PREFIX_U8_BIG_SHORT_DATA: &[u8; 2] = b"\x02a";
pub(crate) const UTF_8_PREFIX_U16_BIG_SHORT_DATA: &[u8; 3] = b"\x02\x00a";
pub(crate) const UTF_8_PREFIX_U32_BIG_SHORT_DATA: &[u8; 5] = b"\x02\x00\x00\x00a";
pub(crate) const UTF_8_PREFIX_U32_7BIT_BIG_SHORT_DATA: &[u8; 2] = b"\x02a";
// pub(crate) const ZERO_ENDED_BIG_SHORT_DATA — length is dynamic

// ---------- Buffer is too small
// error: incomplete
pub(crate) const UTF_8_FIXED_FORCE_ZERO_LITTLE_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_8_FIXED_ALLOW_NO_ZERO_LITTLE_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_8_PREFIX_U8_LITTLE_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_8_PREFIX_U16_LITTLE_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_8_PREFIX_U32_LITTLE_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_8_PREFIX_U32_7BIT_LITTLE_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_8_ZERO_ENDED_LITTLE_SMALL_BUFFER: &[u8; 0] = &[];

pub(crate) const UTF_8_FIXED_FORCE_ZERO_BIG_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_8_FIXED_ALLOW_NO_ZERO_BIG_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_8_PREFIX_U8_BIG_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_8_PREFIX_U16_BIG_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_8_PREFIX_U32_BIG_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_8_PREFIX_U32_7BIT_BIG_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_8_ZERO_ENDED_BIG_SMALL_BUFFER: &[u8; 0] = &[];

//  ---------- Zero boundary not found
// error: incomplete
// pub(crate) const FIXED_FORCE_ZERO_LITTLE_NO_ZERO_INSIDE — length is specified in format
// pub(crate) const FIXED_ALLOW_NO_ZERO_LITTLE_NO_ZERO_INSIDE — length is specified in format
// pub(crate) const PREFIX_U8_LITTLE_NO_ZERO_INSIDE: &[u8; 1] — length is specified in format
// pub(crate) const PREFIX_U16_LITTLE_NO_ZERO_INSIDE: &[u8; 2] — length is specified in format
// pub(crate) const PREFIX_U32_LITTLE_NO_ZERO_INSIDE: &[u8; 4] — length is specified in format
pub(crate) const UTF_8_ZERO_ENDED_LITTLE_NO_ZERO_INSIDE: &[u8; 7] = b"no zero";

// pub(crate) const FIXED_FORCE_ZERO_BIG_NO_ZERO_INSIDE — length is specified in format
// pub(crate) const FIXED_ALLOW_NO_ZERO_BIG_NO_ZERO_INSIDE — length is specified in format
// pub(crate) const PREFIX_U8_BIG_NO_ZERO_INSIDE: &[u8; 1] — length is specified in format
// pub(crate) const PREFIX_U16_BIG_NO_ZERO_INSIDE: &[u8; 2] — length is specified in format
// pub(crate) const PREFIX_U32_BIG_NO_ZERO_INSIDE: &[u8; 4] — length is specified in format
pub(crate) const UTF_8_ZERO_ENDED_BIG_NO_ZERO_INSIDE: &[u8; 7] = b"no zero";

// ---------- UTF encoding is invalid
// error: parse
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

// ---------- No zero while expected
// error: assertion
pub(crate) const UTF_16_FIXED_FORCE_ZERO_LITTLE_NO_ZERO_INSIDE:
    &[u8; UTF_16_FIXED_LENGTH_DATA] = b"z\x00e\x00r\x00o\x00?\x00";
// pub(crate) const FIXED_ALLOW_NO_ZERO_LITTLE_NO_ZERO_INSIDE — is accepted
// pub(crate) const PREFIX_U8_LITTLE_NO_ZERO_INSIDE: &[u8; 3] — is accepted
// pub(crate) const PREFIX_U16_LITTLE_NO_ZERO_INSIDE: &[u8; 4] — is accepted
// pub(crate) const PREFIX_U32_LITTLE_NO_ZERO_INSIDE: &[u8; 6] — is accepted
// pub(crate) const ZERO_ENDED_LITTLE_NO_ZERO_INSIDE — is required

pub(crate) const UTF_16_FIXED_FORCE_ZERO_BIG_NO_ZERO_INSIDE:
    &[u8; UTF_16_FIXED_LENGTH_DATA] = b"\x00z\x00e\x00r\x00o\x00?";
// pub(crate) const FIXED_ALLOW_NO_ZERO_BIG_NO_ZERO_INSIDE — is accepted
// pub(crate) const PREFIX_U8_BIG_NO_ZERO_INSIDE: &[u8; 3] — is accepted
// pub(crate) const PREFIX_U16_BIG_NO_ZERO_INSIDE: &[u8; 4] — is accepted
// pub(crate) const PREFIX_U32_BIG_NO_ZERO_INSIDE: &[u8; 6] — is accepted
// pub(crate) const ZERO_ENDED_BIG_NO_ZERO_INSIDE — is required

// ---------- Prefixed length, zero in the middle
// error: assertion
// pub(crate) const FIXED_FORCE_ZERO_LITTLE_ZERO_IN_MIDDLE — is accepted
// pub(crate) const FIXED_ALLOW_NO_ZERO_LITTLE_ZERO_IN_MIDDLE — is accepted
pub(crate) const UTF_16_PREFIX_U8_LITTLE_ZERO_IN_MIDDLE: &[u8; 5] =
    b"\x02\x00\x00a\x00";
pub(crate) const UTF_16_PREFIX_U16_LITTLE_ZERO_IN_MIDDLE: &[u8; 6] =
    b"\x02\x00\x00\x00a\x00";
pub(crate) const UTF_16_PREFIX_U32_LITTLE_ZERO_IN_MIDDLE: &[u8; 8] =
    b"\x02\x00\x00\x00\x00\x00a\x00";
pub(crate) const UTF_16_PREFIX_U32_7BIT_LITTLE_ZERO_IN_MIDDLE: &[u8; 5] =
    b"\x02\x00\x00a\x00";
// pub(crate) const ZERO_ENDED_LITTLE_ZERO_IN_MIDDLE — is accepted

// pub(crate) const FIXED_FORCE_ZERO_BIG_IN_MIDDLE — is accepted
// pub(crate) const FIXED_ALLOW_NO_ZERO_BIG_ZERO_IN_MIDDLE — is accepted
pub(crate) const UTF_16_PREFIX_U8_BIG_ZERO_IN_MIDDLE: &[u8; 5] = b"\x02\x00\x00\x00a";
pub(crate) const UTF_16_PREFIX_U16_BIG_ZERO_IN_MIDDLE: &[u8; 6] =
    b"\x00\x02\x00\x00\x00a";
pub(crate) const UTF_16_PREFIX_U32_BIG_ZERO_IN_MIDDLE: &[u8; 8] =
    b"\x00\x00\x00\x02\x00\x00\x00a";
pub(crate) const UTF_16_PREFIX_U32_7BIT_BIG_ZERO_IN_MIDDLE: &[u8; 5] =
    b"\x02\x00\x00\x00a";
// pub(crate) const ZERO_ENDED_BIG_ZERO_IN_MIDDLE — is accepted

// ---------- Pascal string (short length in data)
// error: incomplete
// pub(crate) const FIXED_FORCE_ZERO_LITTLE_SHORT_LEN — length is specified in format
// pub(crate) const FIXED_ALLOW_NO_ZERO_LITTLE_SHORT_LEN — length is specified in format
pub(crate) const UTF_16_PREFIX_U8_LITTLE_SHORT_LEN: &[u8; 1] = b"\x01";
pub(crate) const UTF_16_PREFIX_U16_LITTLE_SHORT_LEN: &[u8; 2] = b"\x01\x00";
pub(crate) const UTF_16_PREFIX_U32_LITTLE_SHORT_LEN: &[u8; 4] = b"\x01\x00\x00\x00";
pub(crate) const UTF_16_PREFIX_U32_7BIT_LITTLE_SHORT_LEN: &[u8; 1] = b"\x01";
// pub(crate) const ZERO_ENDED_LITTLE_SHORT_LEN — length is dynamic

// pub(crate) const FIXED_FORCE_ZERO_BIG_SHORT_LEN — length is specified in format
// pub(crate) const FIXED_ALLOW_NO_ZERO_BIG_SHORT_LEN — length is specified in format
pub(crate) const UTF_16_PREFIX_U8_BIG_SHORT_LEN: &[u8; 1] = b"\x01";
pub(crate) const UTF_16_PREFIX_U16_BIG_SHORT_LEN: &[u8; 2] = b"\x00\x01";
pub(crate) const UTF_16_PREFIX_U32_BIG_SHORT_LEN: &[u8; 4] = b"\x01\x00\x00\x00";
pub(crate) const UTF_16_PREFIX_U32_7BIT_BIG_SHORT_LEN: &[u8; 1] = b"\x01";
// pub(crate) const ZERO_ENDED_BIG_SHORT_LEN — length is dynamic

// ---------- Short data (short data)
// error: incomplete
// pub(crate) const FIXED_FORCE_ZERO_LITTLE_SHORT_DATA — length is specified in format
// pub(crate) const FIXED_ALLOW_NO_ZERO_LITTLE_SHORT_DATA — length is specified in format
pub(crate) const UTF_16_PREFIX_U8_LITTLE_SHORT_DATA: &[u8; 3] = b"\x02a\x00";
pub(crate) const UTF_16_PREFIX_U16_LITTLE_SHORT_DATA: &[u8; 4] = b"\x02\x00a\x00";
pub(crate) const UTF_16_PREFIX_U32_LITTLE_SHORT_DATA: &[u8; 6] =
    b"\x02\x00\x00\x00a\x00";
pub(crate) const UTF_16_PREFIX_U32_7BIT_LITTLE_SHORT_DATA: &[u8; 3] = b"\x02a\x00";

// pub(crate) const ZERO_ENDED_LITTLE_SHORT_DATA — length is dynamic

// pub(crate) const FIXED_FORCE_ZERO_BIG_SHORT_DATA — length is specified in format
// pub(crate) const FIXED_ALLOW_NO_ZERO_BIG_SHORT_DATA — length is specified in format
pub(crate) const UTF_16_PREFIX_U8_BIG_SHORT_DATA: &[u8; 3] = b"\x02\x00a";
pub(crate) const UTF_16_PREFIX_U16_BIG_SHORT_DATA: &[u8; 4] = b"\x00\x02\x00a";
pub(crate) const UTF_16_PREFIX_U32_BIG_SHORT_DATA: &[u8; 6] = b"\x00\x00\x00\x02\x00a";
pub(crate) const UTF_16_PREFIX_U32_7BIT_BIG_SHORT_DATA: &[u8; 3] = b"\x02\x00a";
// pub(crate) const ZERO_ENDED_BIG_SHORT_DATA — length is dynamic

// ---------- Buffer is too small
// error: incomplete
pub(crate) const UTF_16_FIXED_FORCE_ZERO_LITTLE_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_16_FIXED_ALLOW_NO_ZERO_LITTLE_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_16_PREFIX_U8_LITTLE_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_16_PREFIX_U16_LITTLE_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_16_PREFIX_U32_LITTLE_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_16_PREFIX_U32_7BIT_LITTLE_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_16_ZERO_ENDED_LITTLE_SMALL_BUFFER: &[u8; 0] = &[];

pub(crate) const UTF_16_FIXED_FORCE_ZERO_BIG_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_16_FIXED_ALLOW_NO_ZERO_BIG_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_16_PREFIX_U8_BIG_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_16_PREFIX_U16_BIG_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_16_PREFIX_U32_BIG_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_16_PREFIX_U32_7BIT_BIG_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_16_ZERO_ENDED_BIG_SMALL_BUFFER: &[u8; 0] = &[];

//  ---------- Zero boundary not found
// error: incomplete
// pub(crate) const FIXED_FORCE_ZERO_LITTLE_NO_ZERO_INSIDE — length is specified in format
// pub(crate) const FIXED_ALLOW_NO_ZERO_LITTLE_NO_ZERO_INSIDE — length is specified in format
// pub(crate) const PREFIX_U8_LITTLE_NO_ZERO_INSIDE: &[u8; 1] — length is specified in format
// pub(crate) const PREFIX_U16_LITTLE_NO_ZERO_INSIDE: &[u8; 2] — length is specified in format
// pub(crate) const PREFIX_U32_LITTLE_NO_ZERO_INSIDE: &[u8; 4] — length is specified in format
pub(crate) const UTF_16_ZERO_ENDED_LITTLE_NO_ZERO_INSIDE: &[u8; 10] =
    b"z\x00e\x00r\x00o\x00?\x00";

// pub(crate) const FIXED_FORCE_ZERO_BIG_NO_ZERO_INSIDE — length is specified in format
// pub(crate) const FIXED_ALLOW_NO_ZERO_BIG_NO_ZERO_INSIDE — length is specified in format
// pub(crate) const PREFIX_U8_BIG_NO_ZERO_INSIDE: &[u8; 1] — length is specified in format
// pub(crate) const PREFIX_U16_BIG_NO_ZERO_INSIDE: &[u8; 2] — length is specified in format
// pub(crate) const PREFIX_U32_BIG_NO_ZERO_INSIDE: &[u8; 4] — length is specified in format
pub(crate) const UTF_16_ZERO_ENDED_BIG_NO_ZERO_INSIDE: &[u8; 10] =
    b"\x00z\x00e\x00r\x00o\x00?";

// ---------- UTF encoding is invalid
// error: parse
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

// ---------- No zero while expected
// error: assertion
pub(crate) const UTF_32_FIXED_FORCE_ZERO_LITTLE_NO_ZERO_INSIDE:
    &[u8; UTF_32_FIXED_LENGTH_DATA] =
    b"z\x00\x00\x00e\x00\x00\x00r\x00\x00\x00o\x00\x00\x00?\x00\x00\x00";
// pub(crate) const FIXED_ALLOW_NO_ZERO_LITTLE_NO_ZERO_INSIDE — is accepted
// pub(crate) const PREFIX_U8_LITTLE_NO_ZERO_INSIDE: &[u8; 3] — is accepted
// pub(crate) const PREFIX_U16_LITTLE_NO_ZERO_INSIDE: &[u8; 4] — is accepted
// pub(crate) const PREFIX_U32_LITTLE_NO_ZERO_INSIDE: &[u8; 6] — is accepted
// pub(crate) const ZERO_ENDED_LITTLE_NO_ZERO_INSIDE — is required

pub(crate) const UTF_32_FIXED_FORCE_ZERO_BIG_NO_ZERO_INSIDE:
    &[u8; UTF_32_FIXED_LENGTH_DATA] =
    b"\x00\x00\x00z\x00\x00\x00e\x00\x00\x00r\x00\x00\x00o\x00\x00\x00?";
// pub(crate) const FIXED_ALLOW_NO_ZERO_BIG_NO_ZERO_INSIDE — is accepted
// pub(crate) const PREFIX_U8_BIG_NO_ZERO_INSIDE: &[u8; 3] — is accepted
// pub(crate) const PREFIX_U16_BIG_NO_ZERO_INSIDE: &[u8; 4] — is accepted
// pub(crate) const PREFIX_U32_BIG_NO_ZERO_INSIDE: &[u8; 6] — is accepted
// pub(crate) const ZERO_ENDED_BIG_NO_ZERO_INSIDE — is required

// ---------- Prefixed length, zero in the middle
// error: assertion
// pub(crate) const FIXED_FORCE_ZERO_LITTLE_ZERO_IN_MIDDLE — is accepted
// pub(crate) const FIXED_ALLOW_NO_ZERO_LITTLE_ZERO_IN_MIDDLE — is accepted
pub(crate) const UTF_32_PREFIX_U8_LITTLE_ZERO_IN_MIDDLE: &[u8; 9] =
    b"\x02\x00\x00\x00a\x00\x00\x00\x00";
pub(crate) const UTF_32_PREFIX_U16_LITTLE_ZERO_IN_MIDDLE: &[u8; 11] =
    b"\x02\x00\x00\x00\x00\x00\x00a\x00\x00\x00";
pub(crate) const UTF_32_PREFIX_U32_LITTLE_ZERO_IN_MIDDLE: &[u8; 12] =
    b"\x02\x00\x00\x00a\x00\x00\x00\x00\x00\x00\x00";
pub(crate) const UTF_32_PREFIX_U32_7BIT_LITTLE_ZERO_IN_MIDDLE: &[u8; 9] =
    b"\x02a\x00\x00\x00\x00\x00\x00\x00";
// pub(crate) const ZERO_ENDED_LITTLE_ZERO_IN_MIDDLE — is accepted

// pub(crate) const FIXED_FORCE_ZERO_BIG_IN_MIDDLE — is accepted
// pub(crate) const FIXED_ALLOW_NO_ZERO_BIG_ZERO_IN_MIDDLE — is accepted
pub(crate) const UTF_32_PREFIX_U8_BIG_ZERO_IN_MIDDLE: &[u8; 9] =
    b"\x02\x00\x00\x00\x00\x00\x00\x00a";
pub(crate) const UTF_32_PREFIX_U16_BIG_ZERO_IN_MIDDLE: &[u8; 10] =
    b"\x00\x02\x00\x00\x00a\x00\x00\x00\x00";
pub(crate) const UTF_32_PREFIX_U32_BIG_ZERO_IN_MIDDLE: &[u8; 12] =
    b"\x00\x00\x00\x02\x00\x00\x00a\x00\x00\x00\x00";
pub(crate) const UTF_32_PREFIX_U32_7BIT_BIG_ZERO_IN_MIDDLE: &[u8; 9] =
    b"\x02\x00\x00\x00a\x00\x00\x00\x00";
// pub(crate) const ZERO_ENDED_BIG_ZERO_IN_MIDDLE — is accepted

// ---------- Pascal string (short length in data)
// error: incomplete
// pub(crate) const FIXED_FORCE_ZERO_LITTLE_SHORT_LEN — length is specified in format
// pub(crate) const FIXED_ALLOW_NO_ZERO_LITTLE_SHORT_LEN — length is specified in format
pub(crate) const UTF_32_PREFIX_U8_LITTLE_SHORT_LEN: &[u8; 1] = b"\x01";
pub(crate) const UTF_32_PREFIX_U16_LITTLE_SHORT_LEN: &[u8; 2] = b"\x01\x00";
pub(crate) const UTF_32_PREFIX_U32_LITTLE_SHORT_LEN: &[u8; 4] = b"\x01\x00\x00\x00";
pub(crate) const UTF_32_PREFIX_U32_7BIT_LITTLE_SHORT_LEN: &[u8; 1] = b"\x01";
// pub(crate) const ZERO_ENDED_LITTLE_SHORT_LEN — length is dynamic

// pub(crate) const FIXED_FORCE_ZERO_BIG_SHORT_LEN — length is specified in format
// pub(crate) const FIXED_ALLOW_NO_ZERO_BIG_SHORT_LEN — length is specified in format
pub(crate) const UTF_32_PREFIX_U8_BIG_SHORT_LEN: &[u8; 1] = b"\x01";
pub(crate) const UTF_32_PREFIX_U16_BIG_SHORT_LEN: &[u8; 2] = b"\x00\x01";
pub(crate) const UTF_32_PREFIX_U32_BIG_SHORT_LEN: &[u8; 4] = b"\x01\x00\x00\x00";
pub(crate) const UTF_32_PREFIX_U32_7BIT_BIG_SHORT_LEN: &[u8; 1] = b"\x01";
// pub(crate) const ZERO_ENDED_BIG_SHORT_LEN — length is dynamic

// ---------- Short data (short data)
// error: incomplete
// pub(crate) const FIXED_FORCE_ZERO_LITTLE_SHORT_DATA — length is specified in format
// pub(crate) const FIXED_ALLOW_NO_ZERO_LITTLE_SHORT_DATA — length is specified in format
pub(crate) const UTF_32_PREFIX_U8_LITTLE_SHORT_DATA: &[u8; 5] = b"\x02a\x00\x00\x00";
pub(crate) const UTF_32_PREFIX_U16_LITTLE_SHORT_DATA: &[u8; 6] =
    b"\x02\x00a\x00\x00\x00";
pub(crate) const UTF_32_PREFIX_U32_LITTLE_SHORT_DATA: &[u8; 8] =
    b"\x02\x00\x00\x00a\x00\x00\x00";
pub(crate) const UTF_32_PREFIX_U32_7BIT_LITTLE_SHORT_DATA: &[u8; 5] =
    b"\x02a\x00\x00\x00";

// pub(crate) const ZERO_ENDED_LITTLE_SHORT_DATA — length is dynamic

// pub(crate) const FIXED_FORCE_ZERO_BIG_SHORT_DATA — length is specified in format
// pub(crate) const FIXED_ALLOW_NO_ZERO_BIG_SHORT_DATA — length is specified in format
pub(crate) const UTF_32_PREFIX_U8_BIG_SHORT_DATA: &[u8; 5] = b"\x02\x00\x00\x00a";
pub(crate) const UTF_32_PREFIX_U16_BIG_SHORT_DATA: &[u8; 6] = b"\x00\x02\x00\x00\x00a";
pub(crate) const UTF_32_PREFIX_U32_BIG_SHORT_DATA: &[u8; 8] =
    b"\x00\x00\x00\x02\x00\x00\x00a";
pub(crate) const UTF_32_PREFIX_U32_7BIT_BIG_SHORT_DATA: &[u8; 5] = b"\x02\x00\x00\x00a";
// pub(crate) const ZERO_ENDED_BIG_SHORT_DATA — length is dynamic

// ---------- Buffer is too small
// error: incomplete
pub(crate) const UTF_32_FIXED_FORCE_ZERO_LITTLE_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_32_FIXED_ALLOW_NO_ZERO_LITTLE_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_32_PREFIX_U8_LITTLE_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_32_PREFIX_U16_LITTLE_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_32_PREFIX_U32_LITTLE_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_32_PREFIX_U32_7BIT_LITTLE_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_32_ZERO_ENDED_LITTLE_SMALL_BUFFER: &[u8; 0] = &[];

pub(crate) const UTF_32_FIXED_FORCE_ZERO_BIG_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_32_FIXED_ALLOW_NO_ZERO_BIG_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_32_PREFIX_U8_BIG_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_32_PREFIX_U16_BIG_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_32_PREFIX_U32_BIG_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_32_PREFIX_U32_7BIT_BIG_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_32_ZERO_ENDED_BIG_SMALL_BUFFER: &[u8; 0] = &[];

//  ---------- Zero boundary not found
// error: incomplete
// pub(crate) const FIXED_FORCE_ZERO_LITTLE_NO_ZERO_INSIDE — length is specified in format
// pub(crate) const FIXED_ALLOW_NO_ZERO_LITTLE_NO_ZERO_INSIDE — length is specified in format
// pub(crate) const PREFIX_U8_LITTLE_NO_ZERO_INSIDE: &[u8; 1] — length is specified in format
// pub(crate) const PREFIX_U16_LITTLE_NO_ZERO_INSIDE: &[u8; 2] — length is specified in format
// pub(crate) const PREFIX_U32_LITTLE_NO_ZERO_INSIDE: &[u8; 4] — length is specified in format
pub(crate) const UTF_32_ZERO_ENDED_LITTLE_NO_ZERO_INSIDE:
    &[u8; UTF_32_FIXED_LENGTH_DATA] =
    b"z\x00\x00\x00e\x00\x00\x00r\x00\x00\x00o\x00\x00\x00?\x00\x00\x00";

// pub(crate) const FIXED_FORCE_ZERO_BIG_NO_ZERO_INSIDE — length is specified in format
// pub(crate) const FIXED_ALLOW_NO_ZERO_BIG_NO_ZERO_INSIDE — length is specified in format
// pub(crate) const PREFIX_U8_BIG_NO_ZERO_INSIDE: &[u8; 1] — length is specified in format
// pub(crate) const PREFIX_U16_BIG_NO_ZERO_INSIDE: &[u8; 2] — length is specified in format
// pub(crate) const PREFIX_U32_BIG_NO_ZERO_INSIDE: &[u8; 4] — length is specified in format
pub(crate) const UTF_32_ZERO_ENDED_BIG_NO_ZERO_INSIDE: &[u8; UTF_32_FIXED_LENGTH_DATA] =
    b"\x00\x00\x00z\x00\x00\x00e\x00\x00\x00r\x00\x00\x00o\x00\x00\x00?";

pub(crate) const UTF_8_PREFIX_U32_7BIT_LITTLE_INVALID_SIZE_ENCODING: &[u8; 5] =
    b"\xff\xff\xff\xff\xff";
pub(crate) const UTF_8_PREFIX_U32_7BIT_BIG_INVALID_SIZE_ENCODING: &[u8; 5] =
    b"\xff\xff\xff\xff\xff";

pub(crate) const UTF_16_PREFIX_U32_7BIT_LITTLE_INVALID_SIZE_ENCODING: &[u8; 5] =
    b"\xff\xff\xff\xff\xff";
pub(crate) const UTF_16_PREFIX_U32_7BIT_BIG_INVALID_SIZE_ENCODING: &[u8; 5] =
    b"\xff\xff\xff\xff\xff";

pub(crate) const UTF_32_PREFIX_U32_7BIT_LITTLE_INVALID_SIZE_ENCODING: &[u8; 5] =
    b"\xff\xff\xff\xff\xff";
pub(crate) const UTF_32_PREFIX_U32_7BIT_BIG_INVALID_SIZE_ENCODING: &[u8; 5] =
    b"\xff\xff\xff\xff\xff";
