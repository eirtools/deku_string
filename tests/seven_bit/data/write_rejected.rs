// ------------------------------------------------------
// ---------- Write error at the beginning --------------
// ------------------------------------------------------

pub const S7_U32_DATA: u32 = 0x61f6;
pub const S7_U64_DATA: u64 = 0x17ef_d671_864f_c1cb;
pub const S7_U128_DATA: u128 = 0x3da8_37a5_dc61_1b45_2357_0b06_dfe0_194f;

pub const S7_U32_DATA_SIZE: u64 = 0;
pub const S7_U64_DATA_SIZE: u64 = 0;
pub const S7_U128_DATA_SIZE: u64 = 0;

// ------------------------------------------------
// ---------- Write error at the end --------------
// ------------------------------------------------

pub const S7_U32_DATA_END: u32 = 0x61f6;
pub const S7_U64_DATA_END: u64 = 0x17ef_d671_864f_c1cb;
pub const S7_U128_DATA_END: u128 = 0x3da8_37a5_dc61_1b45_2357_0b06_dfe0_194f;

pub const S7_U32_DATA_END_SIZE: u64 = 2;
pub const S7_U64_DATA_END_SIZE: u64 = 9;
pub const S7_U128_DATA_END_SIZE: u64 = 18;
