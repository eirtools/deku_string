// ---------------------------------------
// ---------- Data too short -------------
// ---------------------------------------
//
// Error: incomplete
//
// `VecLayout::End` has variable length

// --- little endian ---
pub const INCOMPLETE_FIXED_LITTLE_SHORT_DATA: &[u8; 0] = b"";
pub const INCOMPLETE_PREFIX_U8_LITTLE_SHORT_DATA: &[u8; 0] = b"";
pub const INCOMPLETE_PREFIX_U16_LITTLE_SHORT_DATA: &[u8; 0] = b"";
pub const INCOMPLETE_PREFIX_U32_LITTLE_SHORT_DATA: &[u8; 0] = b"";
pub const INCOMPLETE_PREFIX_U32_7BIT_LITTLE_SHORT_DATA: &[u8; 0] = b"";

// --- big endian ---
pub const INCOMPLETE_FIXED_BIG_SHORT_DATA: &[u8; 0] = b"";
pub const INCOMPLETE_PREFIX_U8_BIG_SHORT_DATA: &[u8; 0] = b"";
pub const INCOMPLETE_PREFIX_U16_BIG_SHORT_DATA: &[u8; 0] = b"";
pub const INCOMPLETE_PREFIX_U32_BIG_SHORT_DATA: &[u8; 0] = b"";
pub const INCOMPLETE_PREFIX_U32_7BIT_BIG_SHORT_DATA: &[u8; 0] = b"";

// -------------%<-------------%<------|------->%------------->%-------------

// -------------------------------------------------
// ---------- Prefixed length too long -------------
// -------------------------------------------------
//
// Error: incomplete
//
// `VecLayout::FixedLength` — length specified in format
// `VecLayout::End` — length is dynamic

// --- little endian ---
pub const INCOMPLETE_PREFIX_U8_LITTLE_SHORT_LENGTH: &[u8; 1] = b"\xFF";
pub const INCOMPLETE_PREFIX_U16_LITTLE_SHORT_LENGTH: &[u8; 2] = b"\xFF\x00";
pub const INCOMPLETE_PREFIX_U32_LITTLE_SHORT_LENGTH: &[u8; 4] = b"\xFF\x00\x00\x00";
pub const INCOMPLETE_PREFIX_U32_7BIT_LITTLE_SHORT_LENGTH: &[u8; 1] = b"\x7F";

// --- big endian ---
pub const INCOMPLETE_PREFIX_U8_BIG_SHORT_LENGTH: &[u8; 1] = b"\xFF";
pub const INCOMPLETE_PREFIX_U16_BIG_SHORT_LENGTH: &[u8; 2] = b"\x00\xFF";
pub const INCOMPLETE_PREFIX_U32_BIG_SHORT_LENGTH: &[u8; 4] = b"\x00\x00\x00\xFF";
pub const INCOMPLETE_PREFIX_U32_7BIT_BIG_SHORT_LENGTH: &[u8; 1] = b"\x7F";

// -------------%<-------------%<------|------->%------------->%-------------
