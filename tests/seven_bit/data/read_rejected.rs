// ------------------------------------------------
// ---------- No bytes in the middle --------------
// ------------------------------------------------
//
// Error: incomplete

pub const S7_U8_INCOMPLETE_MIDDLE: [u8; 1] = [0xff];
pub const S7_U16_INCOMPLETE_MIDDLE: [u8; 1] = [0xff];
pub const S7_U32_INCOMPLETE_MIDDLE: [u8; 1] = [0xff];
pub const S7_U64_INCOMPLETE_MIDDLE: [u8; 1] = [0xff];
pub const S7_U128_INCOMPLETE_MIDDLE: [u8; 1] = [0xff];

// -------------%<-------------%<------|------->%------------->%-------------

// ---------------------------------------------
// ---------- No bytes at the end --------------
// ---------------------------------------------
//
// Error: incomplete

pub const S7_U8_INCOMPLETE_END: [u8; 1] = [0xff];
pub const S7_U16_INCOMPLETE_END: [u8; 2] = [0xff, 0xff];
pub const S7_U32_INCOMPLETE_END: [u8; 4] = [0xff, 0xff, 0xff, 0xff];
pub const S7_U64_INCOMPLETE_END: [u8; 9] =
    [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff];
pub const S7_U128_INCOMPLETE_END: [u8; 18] = [
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff,
];

// -------------%<-------------%<------|------->%------------->%-------------

// ------------------------------------------
// ---------- Integer overflow --------------
// ------------------------------------------
//
// Error: assertion

pub const S7_U8_ASSERTION_OVERFLOW: [u8; 2] = [0xff, 0x10];
pub const S7_U16_ASSERTION_OVERFLOW: [u8; 3] = [0xff, 0xff, 0x04];
pub const S7_U32_ASSERTION_OVERFLOW: [u8; 5] = [0xff, 0xff, 0xff, 0xff, 0x10];
pub const S7_U64_ASSERTION_OVERFLOW: [u8; 10] =
    [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x02];
pub const S7_U128_ASSERTION_OVERFLOW: [u8; 19] = [
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0x04,
];

// -------------%<-------------%<------|------->%------------->%-------------
