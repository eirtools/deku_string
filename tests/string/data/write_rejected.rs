// ----------------------------------------------------------------
// ---------- No null character inside output buffer --------------
// ----------------------------------------------------------------
//
// Error: assertion
//
// `StringLayout::Fixed((allow_no_null = true)` — is accepted
// `StringLayout::LengthPrefix` — is required
// `StringLayout::ZeroEnded` — is required

// ---- input data -----
pub const ASSERTION_NO_ZERO_INSIDE: &str = "zero?";

// -------------%<-------------%<------|------->%------------->%-------------

// ----------------------------------------------------------------
// ---------- No null character inside output buffer --------------
// ----------------------------------------------------------------
//
// Error: assertion
//
// `StringLayout::Fixed((allow_no_null = true)` — is accepted
// `StringLayout::LengthPrefix` — is required
// `StringLayout::ZeroEnded` — is required

// ---- input data -----
pub const ASSERTION_ZERO_INSIDE_STR: &str = "va\x00id";

// -------------%<-------------%<------|------->%------------->%-------------

// --------------------------------------------------
// ---------- Error while writing data --------------
// --------------------------------------------------
//
// Error: io

// ---- input data -----
pub const IO_DATA: &str = "text";

// ---- io break -----
pub const IO_FIXED_FORCE_ZERO_DATA_SIZE: u64 = 0;
pub const IO_FIXED_ALLOW_NO_ZERO_DATA_SIZE: u64 = 0;
pub const IO_PREFIX_U8_DATA_SIZE: u64 = 3;
pub const IO_PREFIX_U16_DATA_SIZE: u64 = 4;
pub const IO_PREFIX_U32_DATA_SIZE: u64 = 5;
pub const IO_PREFIX_U32_7BIT_DATA_SIZE: u64 = 3;
pub const IO_ZERO_ENDED_DATA_SIZE: u64 = 0;

// -------------%<-------------%<------|------->%------------->%-------------

// -----------------------------------------------------------
// ---------- Error while writing padding start --------------
// -----------------------------------------------------------
//
// Error: io
//
// `StringLayout::LengthPrefix` — no padding

// ---- input data -----
pub const IO_FIXED_END: &str = "text";

// ---- io break -----
pub const IO_FIXED_FORCE_ZERO_FIXED_END_SIZE: u64 = 4;
pub const IO_FIXED_ALLOW_NO_ZERO_FIXED_END_SIZE: u64 = 4;
pub const IO_ZERO_ENDED_FIXED_END_SIZE: u64 = 4;

// -------------%<-------------%<------|------->%------------->%-------------

// -----------------------------------------------------
// ---------- Error while writing padding end ----------
// -----------------------------------------------------
//
// Error: io
//
// `StringLayout::LengthPrefix` — no padding/terminating zeros

// ---- input data -----
pub const IO_SUFFIX: &str = "";

// ---- io break -----
pub const IO_ZERO_ENDED_SUFFIX_SIZE: u64 = 0;
pub const IO_FIXED_FORCE_ZERO_SUFFIX_SIZE: u64 = 2;
pub const IO_FIXED_ALLOW_NO_ZERO_SUFFIX_SIZE: u64 = 2;

// -------------%<-------------%<------|------->%------------->%-------------

// -----------------------------------------------------------
// ---------- Error while writing prefix length --------------
// -----------------------------------------------------------
//
// Error: io
//
// `StringLayout::FixedLength` — no prefix length
// `StringLayout::ZeroEnded` — no prefix length

// ---- input data -----
pub const IO_PREFIX: &str = "value";

// ---- io break -----
pub const IO_PREFIX_U8_PREFIX_SIZE: u64 = 0;
pub const IO_PREFIX_U16_PREFIX_SIZE: u64 = 0;
pub const IO_PREFIX_U32_PREFIX_SIZE: u64 = 0;
pub const IO_PREFIX_U32_7BIT_PREFIX_SIZE: u64 = 0;

// -------------%<-------------%<------|------->%------------->%-------------

// -----------------------------------------------------------
// ---------- Error while writing too much data --------------
// -----------------------------------------------------------
//
// Error: assertion
//
// `StringLayout::ZeroEnded` — dynamic length
// `StringLayout::LengthPrefix` (not u8) — same as u8, but harder to handle data

// ---- input data -----
/// A 256 bytes long string to test out-of-bound checks on write.
pub const ASSERTION_TOO_BIG_DATA: &str = include_str!("../../data/data_256");

// -------------%<-------------%<------|------->%------------->%-------------
