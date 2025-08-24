// FIXED has no prefix
pub const INCOMPLETE_PREFIX_U8_LITTLE_PREFIX: &[u8; 0] = b"";
pub const INCOMPLETE_PREFIX_U8_BIG_PREFIX: &[u8; 0] = b"";
pub const INCOMPLETE_PREFIX_U16_LITTLE_PREFIX: &[u8; 0] = b"";
pub const INCOMPLETE_PREFIX_U16_BIG_PREFIX: &[u8; 0] = b"";
pub const INCOMPLETE_PREFIX_U32_LITTLE_PREFIX: &[u8; 0] = b"";
pub const INCOMPLETE_PREFIX_U32_BIG_PREFIX: &[u8; 0] = b"";
pub const INCOMPLETE_PREFIX_U32_7BIT_LITTLE_PREFIX: &[u8; 0] = b"";
pub const INCOMPLETE_PREFIX_U32_7BIT_BIG_PREFIX: &[u8; 0] = b"";
// END has no prefix

pub const INCOMPLETE_FIXED_LITTLE_DATA: &[u8; 0] = b"";
pub const INCOMPLETE_FIXED_BIG_DATA: &[u8; 0] = b"";
pub const INCOMPLETE_PREFIX_U8_LITTLE_DATA: &[u8; 1] = b"\xFF";
pub const INCOMPLETE_PREFIX_U8_BIG_DATA: &[u8; 1] = b"\xFF";
pub const INCOMPLETE_PREFIX_U16_LITTLE_DATA: &[u8; 2] = b"\xFF\x00";
pub const INCOMPLETE_PREFIX_U16_BIG_DATA: &[u8; 2] = b"\x00\xFF";
pub const INCOMPLETE_PREFIX_U32_LITTLE_DATA: &[u8; 4] = b"\xFF\x00\x00\x00";
pub const INCOMPLETE_PREFIX_U32_BIG_DATA: &[u8; 4] = b"\x00\x00\x00\xFF";
pub const INCOMPLETE_PREFIX_U32_7BIT_LITTLE_DATA: &[u8; 1] = b"\x7F";
pub const INCOMPLETE_PREFIX_U32_7BIT_BIG_DATA: &[u8; 1] = b"\x7F";
