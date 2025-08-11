use super::layouts::FIXED_LENGTH;

/// Raw data size
const UTF_8_FIXED_LENGTH_DATA: usize = FIXED_LENGTH;
/// Raw data size
const UTF_16_FIXED_LENGTH_DATA: usize = FIXED_LENGTH * 2;
const UTF_32_FIXED_LENGTH_DATA: usize = FIXED_LENGTH * 4;

// ---------- UTF encoding is invalid
// error: parse
pub(crate) const UTF_8_FIXED_FORCE_ZERO_LITTLE_INVALID_UTF:
    &[u8; UTF_8_FIXED_LENGTH_DATA] = b"invalid test case  \xe2\x28\xa1\x00";
pub(crate) const UTF_8_FIXED_ALLOW_NO_ZERO_LITTLE_INVALID_UTF:
    &[u8; UTF_8_FIXED_LENGTH_DATA] = b"invalid test case  \xe2\x28\xa1\x00";
pub(crate) const UTF_8_PREFIX_U8_LITTLE_INVALID_UTF: &[u8; 8] = b"\x07utf-\xe2\x28\xa1";
pub(crate) const UTF_8_PREFIX_U16_LITTLE_INVALID_UTF: &[u8; 9] =
    b"\x07\x00utf-\xe2\x28\xa1";
pub(crate) const UTF_8_PREFIX_U32_LITTLE_INVALID_UTF: &[u8; 11] =
    b"\x07\x00\x00\x00utf-\xe2\x28\xa1";
pub(crate) const UTF_8_ZERO_ENDED_LITTLE_INVALID_UTF: &[u8; UTF_8_FIXED_LENGTH_DATA] =
    b"invalid test case  \xe2\x28\xa1\x00";

pub(crate) const UTF_8_FIXED_FORCE_ZERO_BIG_INVALID_UTF: &[u8;
     UTF_8_FIXED_LENGTH_DATA] = b"invalid test case  \xe2\x28\xa1\x00";
pub(crate) const UTF_8_FIXED_ALLOW_NO_ZERO_BIG_INVALID_UTF:
    &[u8; UTF_8_FIXED_LENGTH_DATA] = b"invalid test case  \xe2\x28\xa1\x00";
pub(crate) const UTF_8_PREFIX_U8_BIG_INVALID_UTF: &[u8; 8] = b"\x07utf-\xe2\x28\xa1";
pub(crate) const UTF_8_PREFIX_U16_BIG_INVALID_UTF: &[u8; 9] =
    b"\x00\x07utf-\xe2\x28\xa1";
pub(crate) const UTF_8_PREFIX_U32_BIG_INVALID_UTF: &[u8; 11] =
    b"\x00\x00\x00\x07utf-\xe2\x28\xa1";
pub(crate) const UTF_8_ZERO_ENDED_BIG_INVALID_UTF: &[u8; UTF_8_FIXED_LENGTH_DATA] =
    b"invalid test case  \xe2\x28\xa1\x00";

// ---------- No zero while expected
// error: assertion
pub(crate) const UTF_8_FIXED_FORCE_ZERO_LITTLE_NO_ZERO_INSIDE:
    &[u8; UTF_8_FIXED_LENGTH_DATA] = b"invalid test case zero?";
// pub(crate) const FIXED_ALLOW_NO_ZERO_LITTLE_NO_ZERO_INSIDE — is accepted
// pub(crate) const PREFIX_U8_LITTLE_NO_ZERO_INSIDE: &[u8; 3] — is accepted
// pub(crate) const PREFIX_U16_LITTLE_NO_ZERO_INSIDE: &[u8; 4] — is accepted
// pub(crate) const PREFIX_U32_LITTLE_NO_ZERO_INSIDE: &[u8; 6] — is accepted
// pub(crate) const ZERO_ENDED_LITTLE_NO_ZERO_INSIDE — is required

pub(crate) const UTF_8_FIXED_FORCE_ZERO_BIG_NO_ZERO_INSIDE:
    &[u8; UTF_8_FIXED_LENGTH_DATA] = b"invalid test case zero?";
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
// pub(crate) const UTF_8_ZERO_ENDED_LITTLE_ZERO_IN_MIDDLE- is accepted

// pub(crate) const FIXED_FORCE_ZERO_BIG_IN_MIDDLE — is accepted
// pub(crate) const FIXED_ALLOW_NO_ZERO_BIG_ZERO_IN_MIDDLE — is accepted
pub(crate) const UTF_8_PREFIX_U8_BIG_ZERO_IN_MIDDLE: &[u8; 3] = b"\x02\x00a";
pub(crate) const UTF_8_PREFIX_U16_BIG_ZERO_IN_MIDDLE: &[u8; 4] = b"\x00\x02\x00a";
pub(crate) const UTF_8_PREFIX_U32_BIG_ZERO_IN_MIDDLE: &[u8; 6] =
    b"\x00\x00\x00\x02\x00a";
// pub(crate) const ZERO_ENDED_BIG_ZERO_IN_MIDDLE — is accepted

// ---------- Pascal string (short length in data)
// error: incomplete
// pub(crate) const FIXED_FORCE_ZERO_LITTLE_SHORT_LEN — length is specified in format
// pub(crate) const FIXED_ALLOW_NO_ZERO_LITTLE_SHORT_LEN — length is specified in format
pub(crate) const UTF_8_PREFIX_U8_LITTLE_SHORT_LEN: &[u8; 1] = b"\x01";
pub(crate) const UTF_8_PREFIX_U16_LITTLE_SHORT_LEN: &[u8; 2] = b"\x01\x00";
pub(crate) const UTF_8_PREFIX_U32_LITTLE_SHORT_LEN: &[u8; 4] = b"\x01\x00\x00\x00";
// pub(crate) const ZERO_ENDED_LITTLE_SHORT_LEN — length is dynamic

// pub(crate) const FIXED_FORCE_ZERO_BIG_SHORT_LEN — length is specified in format
// pub(crate) const FIXED_ALLOW_NO_ZERO_BIG_SHORT_LEN — length is specified in format
pub(crate) const UTF_8_PREFIX_U8_BIG_SHORT_LEN: &[u8; 1] = b"\x01";
pub(crate) const UTF_8_PREFIX_U16_BIG_SHORT_LEN: &[u8; 2] = b"\x01\x00";
pub(crate) const UTF_8_PREFIX_U32_BIG_SHORT_LEN: &[u8; 4] = b"\x01\x00\x00\x00";
// pub(crate) const ZERO_ENDED_BIG_SHORT_LEN — length is dynamic

// ---------- Short data (short data)
// error: incomplete
// pub(crate) const FIXED_FORCE_ZERO_LITTLE_SHORT_DATA — length is specified in format
// pub(crate) const FIXED_ALLOW_NO_ZERO_LITTLE_SHORT_DATA — length is specified in format
pub(crate) const UTF_8_PREFIX_U8_LITTLE_SHORT_DATA: &[u8; 2] = b"\x02a";
pub(crate) const UTF_8_PREFIX_U16_LITTLE_SHORT_DATA: &[u8; 3] = b"\x02\x00a";
pub(crate) const UTF_8_PREFIX_U32_LITTLE_SHORT_DATA: &[u8; 5] = b"\x02\x00\x00\x00a";
// pub(crate) const ZERO_ENDED_LITTLE_SHORT_DATA — length is dynamic

// pub(crate) const FIXED_FORCE_ZERO_BIG_SHORT_DATA — length is specified in format
// pub(crate) const FIXED_ALLOW_NO_ZERO_BIG_SHORT_DATA — length is specified in format
pub(crate) const UTF_8_PREFIX_U8_BIG_SHORT_DATA: &[u8; 2] = b"\x02a";
pub(crate) const UTF_8_PREFIX_U16_BIG_SHORT_DATA: &[u8; 3] = b"\x02\x00a";
pub(crate) const UTF_8_PREFIX_U32_BIG_SHORT_DATA: &[u8; 5] = b"\x02\x00\x00\x00a";
// pub(crate) const ZERO_ENDED_BIG_SHORT_DATA — length is dynamic

// ---------- Buffer is too small
// error: incomplete
pub(crate) const UTF_8_FIXED_FORCE_ZERO_LITTLE_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_8_FIXED_ALLOW_NO_ZERO_LITTLE_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_8_PREFIX_U8_LITTLE_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_8_PREFIX_U16_LITTLE_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_8_PREFIX_U32_LITTLE_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_8_ZERO_ENDED_LITTLE_SMALL_BUFFER: &[u8; 0] = &[];

pub(crate) const UTF_8_FIXED_FORCE_ZERO_BIG_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_8_FIXED_ALLOW_NO_ZERO_BIG_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_8_PREFIX_U8_BIG_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_8_PREFIX_U16_BIG_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_8_PREFIX_U32_BIG_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_8_ZERO_ENDED_BIG_SMALL_BUFFER: &[u8; 0] = &[];

//  ---------- Zero boundary not found
// error: incomplete
// pub(crate) const FIXED_FORCE_ZERO_LITTLE_NO_ZERO_INSIDE — length is specified in format
// pub(crate) const FIXED_ALLOW_NO_ZERO_LITTLE_NO_ZERO_INSIDE — length is specified in format
// pub(crate) const PREFIX_U8_LITTLE_NO_ZERO_INSIDE: &[u8; 1] — length is specified in format
// pub(crate) const PREFIX_U16_LITTLE_NO_ZERO_INSIDE: &[u8; 2] — length is specified in format
// pub(crate) const PREFIX_U32_LITTLE_NO_ZERO_INSIDE: &[u8; 4] — length is specified in format
pub(crate) const UTF_8_ZERO_ENDED_LITTLE_NO_ZERO_INSIDE: &[u8; 17] =
    b"invalid test case";

// pub(crate) const FIXED_FORCE_ZERO_BIG_NO_ZERO_INSIDE — length is specified in format
// pub(crate) const FIXED_ALLOW_NO_ZERO_BIG_NO_ZERO_INSIDE — length is specified in format
// pub(crate) const PREFIX_U8_BIG_NO_ZERO_INSIDE: &[u8; 1] — length is specified in format
// pub(crate) const PREFIX_U16_BIG_NO_ZERO_INSIDE: &[u8; 2] — length is specified in format
// pub(crate) const PREFIX_U32_BIG_NO_ZERO_INSIDE: &[u8; 4] — length is specified in format
pub(crate) const UTF_8_ZERO_ENDED_BIG_NO_ZERO_INSIDE: &[u8; 17] = b"invalid test case";

// ---------- UTF encoding is invalid
// error: parse
pub(crate) const UTF_16_FIXED_FORCE_ZERO_LITTLE_INVALID_UTF: &[u8; UTF_16_FIXED_LENGTH_DATA] = b"i\x00n\x00v\x00a\x00l\x00i\x00d\x00 \x00t\x00e\x00s\x00t\x00 \x00c\x00a\x00s\x00e\x00 \x00 \x00\xdc\xdc\xdc\xdc\xdc\xdc\x00\x00";
pub(crate) const UTF_16_FIXED_ALLOW_NO_ZERO_LITTLE_INVALID_UTF: &[u8; UTF_16_FIXED_LENGTH_DATA] = b"i\x00n\x00v\x00a\x00l\x00i\x00d\x00 \x00t\x00e\x00s\x00t\x00 \x00c\x00a\x00s\x00e\x00 \x00 \x00\xdc\xdc\xdc\xdc\xdc\xdc\x00\x00";
pub(crate) const UTF_16_PREFIX_U8_LITTLE_INVALID_UTF: &[u8; 3] = b"\x01\xdc\xdc";
pub(crate) const UTF_16_PREFIX_U16_LITTLE_INVALID_UTF: &[u8; 4] = b"\x01\x00\xdc\xdc";
pub(crate) const UTF_16_PREFIX_U32_LITTLE_INVALID_UTF: &[u8; 6] =
    b"\x01\x00\x00\x00\xdc\xdc";
pub(crate) const UTF_16_ZERO_ENDED_LITTLE_INVALID_UTF: &[u8; UTF_16_FIXED_LENGTH_DATA] = b"i\x00n\x00v\x00a\x00l\x00i\x00d\x00 \x00t\x00e\x00s\x00t\x00 \x00c\x00a\x00s\x00e\x00 \x00 \x00\xdc\xdc\xdc\xdc\xdc\xdc\x00\x00";

pub(crate) const UTF_16_FIXED_FORCE_ZERO_BIG_INVALID_UTF: &[u8; UTF_16_FIXED_LENGTH_DATA] = b"\x00i\x00n\x00v\x00a\x00l\x00i\x00d\x00 \x00t\x00e\x00s\x00t\x00 \x00c\x00a\x00s\x00e\x00 \x00 \xdc\xdc\xdc\xdc\xdc\xdc\x00\x00";
pub(crate) const UTF_16_FIXED_ALLOW_NO_ZERO_BIG_INVALID_UTF: &[u8; UTF_16_FIXED_LENGTH_DATA] = b"\x00i\x00n\x00v\x00a\x00l\x00i\x00d\x00 \x00t\x00e\x00s\x00t\x00 \x00c\x00a\x00s\x00e\x00 \x00 \xdc\xdc\xdc\xdc\xdc\xdc\x00\x00";
pub(crate) const UTF_16_PREFIX_U8_BIG_INVALID_UTF: &[u8; 3] = b"\x01\xdc\xdc";
pub(crate) const UTF_16_PREFIX_U16_BIG_INVALID_UTF: &[u8; 4] = b"\x00\x01\xdc\xdc";
pub(crate) const UTF_16_PREFIX_U32_BIG_INVALID_UTF: &[u8; 6] =
    b"\x00\x00\x00\x01\xdc\xdc";
pub(crate) const UTF_16_ZERO_ENDED_BIG_INVALID_UTF: &[u8; UTF_16_FIXED_LENGTH_DATA] = b"\x00i\x00n\x00v\x00a\x00l\x00i\x00d\x00 \x00t\x00e\x00s\x00t\x00 \x00c\x00a\x00s\x00e\x00 \x00 \xdc\xdc\xdc\xdc\xdc\xdc\x00\x00";

// ---------- No zero while expected
// error: assertion
pub(crate) const UTF_16_FIXED_FORCE_ZERO_LITTLE_NO_ZERO_INSIDE: &[u8; UTF_16_FIXED_LENGTH_DATA] = b"v\x00a\x00l\x00i\x00d\x00 \x00t\x00e\x00s\x00t\x00 \x00c\x00a\x00s\x00e\x00 \x00n\x00o\x00 \x00z\x00e\x00r\x00o\x00";
// pub(crate) const FIXED_ALLOW_NO_ZERO_LITTLE_NO_ZERO_INSIDE — is accepted
// pub(crate) const PREFIX_U8_LITTLE_NO_ZERO_INSIDE: &[u8; 3] — is accepted
// pub(crate) const PREFIX_U16_LITTLE_NO_ZERO_INSIDE: &[u8; 4] — is accepted
// pub(crate) const PREFIX_U32_LITTLE_NO_ZERO_INSIDE: &[u8; 6] — is accepted
// pub(crate) const ZERO_ENDED_LITTLE_NO_ZERO_INSIDE — is required

pub(crate) const UTF_16_FIXED_FORCE_ZERO_BIG_NO_ZERO_INSIDE: &[u8; UTF_16_FIXED_LENGTH_DATA] = b"\x00v\x00a\x00l\x00i\x00d\x00 \x00t\x00e\x00s\x00t\x00 \x00c\x00a\x00s\x00e\x00 \x00n\x00o\x00 \x00z\x00e\x00r\x00o";
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
// pub(crate) const ZERO_ENDED_LITTLE_ZERO_IN_MIDDLE — is accepted

// pub(crate) const FIXED_FORCE_ZERO_BIG_IN_MIDDLE — is accepted
// pub(crate) const FIXED_ALLOW_NO_ZERO_BIG_ZERO_IN_MIDDLE — is accepted
pub(crate) const UTF_16_PREFIX_U8_BIG_ZERO_IN_MIDDLE: &[u8; 5] = b"\x02\x00\x00\x00a";
pub(crate) const UTF_16_PREFIX_U16_BIG_ZERO_IN_MIDDLE: &[u8; 6] =
    b"\x00\x02\x00\x00\x00a";
pub(crate) const UTF_16_PREFIX_U32_BIG_ZERO_IN_MIDDLE: &[u8; 8] =
    b"\x00\x00\x00\x02\x00\x00\x00a";
// pub(crate) const ZERO_ENDED_BIG_ZERO_IN_MIDDLE — is accepted

// ---------- Pascal string (short length in data)
// error: incomplete
// pub(crate) const FIXED_FORCE_ZERO_LITTLE_SHORT_LEN — length is specified in format
// pub(crate) const FIXED_ALLOW_NO_ZERO_LITTLE_SHORT_LEN — length is specified in format
pub(crate) const UTF_16_PREFIX_U8_LITTLE_SHORT_LEN: &[u8; 1] = b"\x01";
pub(crate) const UTF_16_PREFIX_U16_LITTLE_SHORT_LEN: &[u8; 2] = b"\x01\x00";
pub(crate) const UTF_16_PREFIX_U32_LITTLE_SHORT_LEN: &[u8; 4] = b"\x01\x00\x00\x00";
// pub(crate) const ZERO_ENDED_LITTLE_SHORT_LEN — length is dynamic

// pub(crate) const FIXED_FORCE_ZERO_BIG_SHORT_LEN — length is specified in format
// pub(crate) const FIXED_ALLOW_NO_ZERO_BIG_SHORT_LEN — length is specified in format
pub(crate) const UTF_16_PREFIX_U8_BIG_SHORT_LEN: &[u8; 1] = b"\x01";
pub(crate) const UTF_16_PREFIX_U16_BIG_SHORT_LEN: &[u8; 2] = b"\x00\x01";
pub(crate) const UTF_16_PREFIX_U32_BIG_SHORT_LEN: &[u8; 4] = b"\x01\x00\x00\x00";
// pub(crate) const ZERO_ENDED_BIG_SHORT_LEN — length is dynamic

// ---------- Short data (short data)
// error: incomplete
// pub(crate) const FIXED_FORCE_ZERO_LITTLE_SHORT_DATA — length is specified in format
// pub(crate) const FIXED_ALLOW_NO_ZERO_LITTLE_SHORT_DATA — length is specified in format
pub(crate) const UTF_16_PREFIX_U8_LITTLE_SHORT_DATA: &[u8; 3] = b"\x02a\x00";
pub(crate) const UTF_16_PREFIX_U16_LITTLE_SHORT_DATA: &[u8; 4] = b"\x02\x00a\x00";
pub(crate) const UTF_16_PREFIX_U32_LITTLE_SHORT_DATA: &[u8; 6] =
    b"\x02\x00\x00\x00a\x00";

// pub(crate) const ZERO_ENDED_LITTLE_SHORT_DATA — length is dynamic

// pub(crate) const FIXED_FORCE_ZERO_BIG_SHORT_DATA — length is specified in format
// pub(crate) const FIXED_ALLOW_NO_ZERO_BIG_SHORT_DATA — length is specified in format
pub(crate) const UTF_16_PREFIX_U8_BIG_SHORT_DATA: &[u8; 3] = b"\x02\x00a";
pub(crate) const UTF_16_PREFIX_U16_BIG_SHORT_DATA: &[u8; 4] = b"\x00\x02\x00a";
pub(crate) const UTF_16_PREFIX_U32_BIG_SHORT_DATA: &[u8; 6] = b"\x02\x00\x00\x00\x00a";
// pub(crate) const ZERO_ENDED_BIG_SHORT_DATA — length is dynamic

// ---------- Buffer is too small
// error: incomplete
pub(crate) const UTF_16_FIXED_FORCE_ZERO_LITTLE_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_16_FIXED_ALLOW_NO_ZERO_LITTLE_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_16_PREFIX_U8_LITTLE_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_16_PREFIX_U16_LITTLE_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_16_PREFIX_U32_LITTLE_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_16_ZERO_ENDED_LITTLE_SMALL_BUFFER: &[u8; 0] = &[];

pub(crate) const UTF_16_FIXED_FORCE_ZERO_BIG_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_16_FIXED_ALLOW_NO_ZERO_BIG_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_16_PREFIX_U8_BIG_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_16_PREFIX_U16_BIG_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_16_PREFIX_U32_BIG_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_16_ZERO_ENDED_BIG_SMALL_BUFFER: &[u8; 0] = &[];

//  ---------- Zero boundary not found
// error: incomplete
// pub(crate) const FIXED_FORCE_ZERO_LITTLE_NO_ZERO_INSIDE — length is specified in format
// pub(crate) const FIXED_ALLOW_NO_ZERO_LITTLE_NO_ZERO_INSIDE — length is specified in format
// pub(crate) const PREFIX_U8_LITTLE_NO_ZERO_INSIDE: &[u8; 1] — length is specified in format
// pub(crate) const PREFIX_U16_LITTLE_NO_ZERO_INSIDE: &[u8; 2] — length is specified in format
// pub(crate) const PREFIX_U32_LITTLE_NO_ZERO_INSIDE: &[u8; 4] — length is specified in format
pub(crate) const UTF_16_ZERO_ENDED_LITTLE_NO_ZERO_INSIDE: &[u8; 34] =
    b"i\x00n\x00v\x00a\x00l\x00i\x00d\x00 \x00t\x00e\x00s\x00t\x00 \x00c\x00a\x00s\x00e\x00";

// pub(crate) const FIXED_FORCE_ZERO_BIG_NO_ZERO_INSIDE — length is specified in format
// pub(crate) const FIXED_ALLOW_NO_ZERO_BIG_NO_ZERO_INSIDE — length is specified in format
// pub(crate) const PREFIX_U8_BIG_NO_ZERO_INSIDE: &[u8; 1] — length is specified in format
// pub(crate) const PREFIX_U16_BIG_NO_ZERO_INSIDE: &[u8; 2] — length is specified in format
// pub(crate) const PREFIX_U32_BIG_NO_ZERO_INSIDE: &[u8; 4] — length is specified in format
pub(crate) const UTF_16_ZERO_ENDED_BIG_NO_ZERO_INSIDE: &[u8; 34] =
    b"\x00i\x00n\x00v\x00a\x00l\x00i\x00d\x00 \x00t\x00e\x00s\x00t\x00 \x00c\x00a\x00s\x00e";

// ---------- UTF encoding is invalid
// error: parse
pub(crate) const UTF_32_FIXED_FORCE_ZERO_LITTLE_INVALID_UTF: &[u8; UTF_32_FIXED_LENGTH_DATA] = b"i\x00\x00\x00n\x00\x00\x00v\x00\x00\x00a\x00\x00\x00l\x00\x00\x00i\x00\x00\x00d\x00\x00\x00 \x00\x00\x00t\x00\x00\x00e\x00\x00\x00s\x00\x00\x00t\x00\x00\x00 \x00\x00\x00c\x00\x00\x00a\x00\x00\x00s\x00\x00\x00e\x00\x00\x00 \x00\x00\x00 \x00\x00\x00\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\x00\x00\x00\x00\x00\x00";
pub(crate) const UTF_32_FIXED_ALLOW_NO_ZERO_LITTLE_INVALID_UTF: &[u8; UTF_32_FIXED_LENGTH_DATA] = b"i\x00\x00\x00n\x00\x00\x00v\x00\x00\x00a\x00\x00\x00l\x00\x00\x00i\x00\x00\x00d\x00\x00\x00 \x00\x00\x00t\x00\x00\x00e\x00\x00\x00s\x00\x00\x00t\x00\x00\x00 \x00\x00\x00c\x00\x00\x00a\x00\x00\x00s\x00\x00\x00e\x00\x00\x00 \x00\x00\x00 \x00\x00\x00\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\x00\x00\x00\x00\x00\x00";
pub(crate) const UTF_32_PREFIX_U8_LITTLE_INVALID_UTF: &[u8; 5] =
    b"\x01\xdc\xdc\xdc\xdc";
pub(crate) const UTF_32_PREFIX_U16_LITTLE_INVALID_UTF: &[u8; 6] =
    b"\x01\x00\xdc\xdc\xdc\xdc";
pub(crate) const UTF_32_PREFIX_U32_LITTLE_INVALID_UTF: &[u8; 8] =
    b"\x01\x00\x00\x00\xdc\xdc\xdc\xdc";
pub(crate) const UTF_32_ZERO_ENDED_LITTLE_INVALID_UTF: &[u8; UTF_32_FIXED_LENGTH_DATA] = b"i\x00\x00\x00n\x00\x00\x00v\x00\x00\x00a\x00\x00\x00l\x00\x00\x00i\x00\x00\x00d\x00\x00\x00 \x00\x00\x00t\x00\x00\x00e\x00\x00\x00s\x00\x00\x00t\x00\x00\x00 \x00\x00\x00c\x00\x00\x00a\x00\x00\x00s\x00\x00\x00e\x00\x00\x00 \x00\x00\x00\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\x00\x00\x00\x00\x00\x00";

pub(crate) const UTF_32_FIXED_FORCE_ZERO_BIG_INVALID_UTF: &[u8; UTF_32_FIXED_LENGTH_DATA] = b"\x00\x00\x00i\x00\x00\x00n\x00\x00\x00v\x00\x00\x00a\x00\x00\x00l\x00\x00\x00i\x00\x00\x00d\x00\x00\x00 \x00\x00\x00t\x00\x00\x00e\x00\x00\x00s\x00\x00\x00t\x00\x00\x00 \x00\x00\x00c\x00\x00\x00a\x00\x00\x00s\x00\x00\x00e\x00\x00\x00 \xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\x00\x00\x00\x00\x00\x00";
pub(crate) const UTF_32_FIXED_ALLOW_NO_ZERO_BIG_INVALID_UTF: &[u8; UTF_32_FIXED_LENGTH_DATA] = b"\x00\x00\x00i\x00\x00\x00n\x00\x00\x00v\x00\x00\x00a\x00\x00\x00l\x00\x00\x00i\x00\x00\x00d\x00\x00\x00 \x00\x00\x00t\x00\x00\x00e\x00\x00\x00s\x00\x00\x00t\x00\x00\x00 \x00\x00\x00c\x00\x00\x00a\x00\x00\x00s\x00\x00\x00e\x00\x00\x00 \xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\x00\x00\x00\x00\x00\x00";
pub(crate) const UTF_32_PREFIX_U8_BIG_INVALID_UTF: &[u8; 5] = b"\x01\xdc\xdc\xdc\xdc";
pub(crate) const UTF_32_PREFIX_U16_BIG_INVALID_UTF: &[u8; 6] =
    b"\x00\x01\xdc\xdc\xdc\xdc";
pub(crate) const UTF_32_PREFIX_U32_BIG_INVALID_UTF: &[u8; 8] =
    b"\x00\x00\x00\x01\xdc\xdc\xdc\xdc";
pub(crate) const UTF_32_ZERO_ENDED_BIG_INVALID_UTF: &[u8; UTF_32_FIXED_LENGTH_DATA] = b"\x00\x00\x00i\x00\x00\x00n\x00\x00\x00v\x00\x00\x00a\x00\x00\x00l\x00\x00\x00i\x00\x00\x00d\x00\x00\x00 \x00\x00\x00t\x00\x00\x00e\x00\x00\x00s\x00\x00\x00t\x00\x00\x00 \x00\x00\x00c\x00\x00\x00a\x00\x00\x00s\x00\x00\x00e\x00\x00\x00 \xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\xdc\x00\x00\x00\x00\x00\x00";

// ---------- No zero while expected
// error: assertion
pub(crate) const UTF_32_FIXED_FORCE_ZERO_LITTLE_NO_ZERO_INSIDE: &[u8; UTF_32_FIXED_LENGTH_DATA] = b"v\x00\x00\x00a\x00\x00\x00l\x00\x00\x00i\x00\x00\x00d\x00\x00\x00 \x00\x00\x00t\x00\x00\x00e\x00\x00\x00s\x00\x00\x00t\x00\x00\x00 \x00\x00\x00c\x00\x00\x00a\x00\x00\x00s\x00\x00\x00e\x00\x00\x00 \x00\x00\x00n\x00\x00\x00o\x00\x00\x00 \x00\x00\x00z\x00\x00\x00e\x00\x00\x00r\x00\x00\x00o\x00\x00\x00";
// pub(crate) const FIXED_ALLOW_NO_ZERO_LITTLE_NO_ZERO_INSIDE — is accepted
// pub(crate) const PREFIX_U8_LITTLE_NO_ZERO_INSIDE: &[u8; 3] — is accepted
// pub(crate) const PREFIX_U16_LITTLE_NO_ZERO_INSIDE: &[u8; 4] — is accepted
// pub(crate) const PREFIX_U32_LITTLE_NO_ZERO_INSIDE: &[u8; 6] — is accepted
// pub(crate) const ZERO_ENDED_LITTLE_NO_ZERO_INSIDE — is required

pub(crate) const UTF_32_FIXED_FORCE_ZERO_BIG_NO_ZERO_INSIDE: &[u8; UTF_32_FIXED_LENGTH_DATA] = b"\x00\x00\x00v\x00\x00\x00a\x00\x00\x00l\x00\x00\x00i\x00\x00\x00d\x00\x00\x00 \x00\x00\x00t\x00\x00\x00e\x00\x00\x00s\x00\x00\x00t\x00\x00\x00 \x00\x00\x00c\x00\x00\x00a\x00\x00\x00s\x00\x00\x00e\x00\x00\x00 \x00\x00\x00n\x00\x00\x00o\x00\x00\x00 \x00\x00\x00z\x00\x00\x00e\x00\x00\x00r\x00\x00\x00o";
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
// pub(crate) const ZERO_ENDED_LITTLE_ZERO_IN_MIDDLE — is accepted

// pub(crate) const FIXED_FORCE_ZERO_BIG_IN_MIDDLE — is accepted
// pub(crate) const FIXED_ALLOW_NO_ZERO_BIG_ZERO_IN_MIDDLE — is accepted
pub(crate) const UTF_32_PREFIX_U8_BIG_ZERO_IN_MIDDLE: &[u8; 9] =
    b"\x02\x00\x00\x00\x00\x00\x00\x00a";
pub(crate) const UTF_32_PREFIX_U16_BIG_ZERO_IN_MIDDLE: &[u8; 10] =
    b"\x00\x02\x00\x00\x00a\x00\x00\x00\x00";
pub(crate) const UTF_32_PREFIX_U32_BIG_ZERO_IN_MIDDLE: &[u8; 12] =
    b"\x00\x00\x00\x02\x00\x00\x00a\x00\x00\x00\x00";
// pub(crate) const ZERO_ENDED_BIG_ZERO_IN_MIDDLE — is accepted

// ---------- Pascal string (short length in data)
// error: incomplete
// pub(crate) const FIXED_FORCE_ZERO_LITTLE_SHORT_LEN — length is specified in format
// pub(crate) const FIXED_ALLOW_NO_ZERO_LITTLE_SHORT_LEN — length is specified in format
pub(crate) const UTF_32_PREFIX_U8_LITTLE_SHORT_LEN: &[u8; 1] = b"\x01";
pub(crate) const UTF_32_PREFIX_U16_LITTLE_SHORT_LEN: &[u8; 2] = b"\x01\x00";
pub(crate) const UTF_32_PREFIX_U32_LITTLE_SHORT_LEN: &[u8; 4] = b"\x01\x00\x00\x00";
// pub(crate) const ZERO_ENDED_LITTLE_SHORT_LEN — length is dynamic

// pub(crate) const FIXED_FORCE_ZERO_BIG_SHORT_LEN — length is specified in format
// pub(crate) const FIXED_ALLOW_NO_ZERO_BIG_SHORT_LEN — length is specified in format
pub(crate) const UTF_32_PREFIX_U8_BIG_SHORT_LEN: &[u8; 1] = b"\x01";
pub(crate) const UTF_32_PREFIX_U16_BIG_SHORT_LEN: &[u8; 2] = b"\x00\x01";
pub(crate) const UTF_32_PREFIX_U32_BIG_SHORT_LEN: &[u8; 4] = b"\x01\x00\x00\x00";
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

// pub(crate) const ZERO_ENDED_LITTLE_SHORT_DATA — length is dynamic

// pub(crate) const FIXED_FORCE_ZERO_BIG_SHORT_DATA — length is specified in format
// pub(crate) const FIXED_ALLOW_NO_ZERO_BIG_SHORT_DATA — length is specified in format
pub(crate) const UTF_32_PREFIX_U8_BIG_SHORT_DATA: &[u8; 5] = b"\x02\x00\x00\x00a";
pub(crate) const UTF_32_PREFIX_U16_BIG_SHORT_DATA: &[u8; 6] = b"\x00\x02\x00\x00\x00a";
pub(crate) const UTF_32_PREFIX_U32_BIG_SHORT_DATA: &[u8; 8] =
    b"\x00\x00\x00\x02\x00\x00\x00a";
// pub(crate) const ZERO_ENDED_BIG_SHORT_DATA — length is dynamic

// ---------- Buffer is too small
// error: incomplete
pub(crate) const UTF_32_FIXED_FORCE_ZERO_LITTLE_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_32_FIXED_ALLOW_NO_ZERO_LITTLE_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_32_PREFIX_U8_LITTLE_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_32_PREFIX_U16_LITTLE_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_32_PREFIX_U32_LITTLE_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_32_ZERO_ENDED_LITTLE_SMALL_BUFFER: &[u8; 0] = &[];

pub(crate) const UTF_32_FIXED_FORCE_ZERO_BIG_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_32_FIXED_ALLOW_NO_ZERO_BIG_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_32_PREFIX_U8_BIG_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_32_PREFIX_U16_BIG_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_32_PREFIX_U32_BIG_SMALL_BUFFER: &[u8; 0] = &[];
pub(crate) const UTF_32_ZERO_ENDED_BIG_SMALL_BUFFER: &[u8; 0] = &[];

//  ---------- Zero boundary not found
// error: incomplete
// pub(crate) const FIXED_FORCE_ZERO_LITTLE_NO_ZERO_INSIDE — length is specified in format
// pub(crate) const FIXED_ALLOW_NO_ZERO_LITTLE_NO_ZERO_INSIDE — length is specified in format
// pub(crate) const PREFIX_U8_LITTLE_NO_ZERO_INSIDE: &[u8; 1] — length is specified in format
// pub(crate) const PREFIX_U16_LITTLE_NO_ZERO_INSIDE: &[u8; 2] — length is specified in format
// pub(crate) const PREFIX_U32_LITTLE_NO_ZERO_INSIDE: &[u8; 4] — length is specified in format
pub(crate) const UTF_32_ZERO_ENDED_LITTLE_NO_ZERO_INSIDE: &[u8; 34] =
    b"i\x00n\x00v\x00a\x00l\x00i\x00d\x00 \x00t\x00e\x00s\x00t\x00 \x00c\x00a\x00s\x00e\x00";

// pub(crate) const FIXED_FORCE_ZERO_BIG_NO_ZERO_INSIDE — length is specified in format
// pub(crate) const FIXED_ALLOW_NO_ZERO_BIG_NO_ZERO_INSIDE — length is specified in format
// pub(crate) const PREFIX_U8_BIG_NO_ZERO_INSIDE: &[u8; 1] — length is specified in format
// pub(crate) const PREFIX_U16_BIG_NO_ZERO_INSIDE: &[u8; 2] — length is specified in format
// pub(crate) const PREFIX_U32_BIG_NO_ZERO_INSIDE: &[u8; 4] — length is specified in format
pub(crate) const UTF_32_ZERO_ENDED_BIG_NO_ZERO_INSIDE: &[u8; 34] =
    b"\x00i\x00n\x00v\x00a\x00l\x00i\x00d\x00 \x00t\x00e\x00s\x00t\x00 \x00c\x00a\x00s\x00e";
