// ------------------------------------
// ---------- Value of 0 --------------
// ------------------------------------

pub const S7_U32_ZERO_IN: u32 = 0;
pub const S7_U64_ZERO_IN: u64 = 0;
pub const S7_U128_ZERO_IN: u128 = 0;

pub const S7_U32_ZERO_OUT: [u8; 1] = [0];
pub const S7_U64_ZERO_OUT: [u8; 1] = [0];
pub const S7_U128_ZERO_OUT: [u8; 1] = [0];

// ---------------------------------------------
// ---------- Value less than 128 --------------
// ---------------------------------------------

pub const S7_U32_LESS_128_IN: u32 = 0x2a;
pub const S7_U64_LESS_128_IN: u64 = 0x2a;
pub const S7_U128_LESS_128_IN: u128 = 0x2a;

pub const S7_U32_LESS_128_OUT: [u8; 1] = [0x2a];
pub const S7_U64_LESS_128_OUT: [u8; 1] = [0x2a];
pub const S7_U128_LESS_128_OUT: [u8; 1] = [0x2a];

// -----------------------------------------
// ---------- Value equal 127 --------------
// -----------------------------------------

pub const S7_U32_EQ_127_IN: u32 = 0x7f;
pub const S7_U64_EQ_127_IN: u64 = 0x7f;
pub const S7_U128_EQ_127_IN: u128 = 0x7f;

pub const S7_U32_EQ_127_OUT: [u8; 1] = [0x7f];
pub const S7_U64_EQ_127_OUT: [u8; 1] = [0x7f];
pub const S7_U128_EQ_127_OUT: [u8; 1] = [0x7f];

// ---------------------------------------------
// ---------- Value more than 127 --------------
// ---------------------------------------------

pub const S7_U32_GREATER_127_IN: u32 = 0x61f6;
pub const S7_U64_GREATER_127_IN: u64 = 0x17ef_d671_864f_c1cb;
pub const S7_U128_GREATER_127_IN: u128 = 0x3da8_37a5_dc61_1b45_2357_0b06_dfe0_194f;

pub const S7_U32_GREATER_127_OUT: [u8; 3] = [0xf6, 0xc3, 0x01];
pub const S7_U64_GREATER_127_OUT: [u8; 9] =
    [0xcb, 0x83, 0xbf, 0xb2, 0x98, 0xce, 0xf5, 0xf7, 0x17];
pub const S7_U128_GREATER_127_OUT: [u8; 18] = [
    0xcf, 0xb2, 0x80, 0xff, 0xed, 0xe0, 0xc2, 0xab, 0xa3, 0x8a, 0xed, 0x88, 0xc6, 0xbb,
    0xe9, 0x9b, 0xa8, 0x7b,
];

// ---------------------------------------
// ---------- Maximum value --------------
// ---------------------------------------

pub const S7_U32_MAX_IN: u32 = u32::MAX;
pub const S7_U64_MAX_IN: u64 = u64::MAX;
pub const S7_U128_MAX_IN: u128 = u128::MAX;

pub const S7_U32_MAX_OUT: [u8; 5] = [0xff, 0xff, 0xff, 0xff, 0x0F];
pub const S7_U64_MAX_OUT: [u8; 10] =
    [0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x01];
pub const S7_U128_MAX_OUT: [u8; 19] = [
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0x03,
];
