pub const IO_U8_DATA: &[u8; 3] = b"val";
pub const IO_STR_DATA: &[&str; 1] = &["val"];

pub const IO_FIXED_DATA_SIZE: u64 = 0;
pub const IO_PREFIX_U8_DATA_SIZE: u64 = 3;
pub const IO_PREFIX_U16_DATA_SIZE: u64 = 4;
pub const IO_PREFIX_U32_DATA_SIZE: u64 = 5;
pub const IO_PREFIX_U32_7BIT_DATA_SIZE: u64 = 3;
pub const IO_END_DATA_SIZE: u64 = 0;

pub const IO_U8_PREFIX: &[u8; 3] = b"val";
pub const IO_STR_PREFIX: &[&str; 1] = &["val"];

//pub const IO_FIXED_PREFIX_SIZE — no prefix
pub const IO_PREFIX_U8_PREFIX_SIZE: u64 = 0;
pub const IO_PREFIX_U16_PREFIX_SIZE: u64 = 0;
pub const IO_PREFIX_U32_PREFIX_SIZE: u64 = 0;
pub const IO_PREFIX_U32_7BIT_PREFIX_SIZE: u64 = 0;
// pub const IO_END_PREFIX_SIZE -- no prefix

pub const IO_U8_LESS_DATA: &[u8; 2] = b"va";
pub const IO_STR_LESS_DATA: &[&str; 1] = &["va"];

pub const IO_FIXED_LESS_DATA_SIZE: u64 = 0;

pub const IO_U8_LESS_REST: &[u8; 2] = b"va";
pub const IO_STR_LESS_REST: &[&str; 1] = &["va"];

pub const IO_FIXED_LESS_REST_SIZE: u64 = 3;

pub const ASSERTION_U8_TOO_BIG_DATA: &[u8; 256] = include_bytes!("../../data/data_256");
