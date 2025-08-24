pub const ASSERTION_NO_ZERO_INSIDE: &str = "zero?";

pub const ASSERTION_ZERO_INSIDE_STR: &str = "va\x00id";

pub const IO_DATA: &str = "text";

pub const IO_FIXED_FORCE_ZERO_DATA_SIZE: u64 = 0;
pub const IO_FIXED_ALLOW_NO_ZERO_DATA_SIZE: u64 = 0;
pub const IO_PREFIX_U8_DATA_SIZE: u64 = 3;
pub const IO_PREFIX_U16_DATA_SIZE: u64 = 4;
pub const IO_PREFIX_U32_DATA_SIZE: u64 = 5;
pub const IO_PREFIX_U32_7BIT_DATA_SIZE: u64 = 3;
pub const IO_ZERO_ENDED_DATA_SIZE: u64 = 0;

pub const IO_FIXED_END: &str = "text";

pub const IO_FIXED_FORCE_ZERO_FIXED_END_SIZE: u64 = 4;
pub const IO_FIXED_ALLOW_NO_ZERO_FIXED_END_SIZE: u64 = 4;

pub const IO_PREFIX: &str = "value";

//pub const IO_FIXED_PREFIX_SIZE — no prefix
pub const IO_PREFIX_U8_PREFIX_SIZE: u64 = 0;
pub const IO_PREFIX_U16_PREFIX_SIZE: u64 = 0;
pub const IO_PREFIX_U32_PREFIX_SIZE: u64 = 0;
pub const IO_PREFIX_U32_7BIT_PREFIX_SIZE: u64 = 0;
// pub const IO_ZERO_ENDED_PREFIX_SIZE -- no prefix

pub const IO_SUFFIX: &str = "";
pub const IO_ZERO_ENDED_SUFFIX_SIZE: u64 = 0;
pub const IO_FIXED_FORCE_ZERO_SUFFIX_SIZE: u64 = 2;
pub const IO_FIXED_ALLOW_NO_ZERO_SUFFIX_SIZE: u64 = 2;

/// A 256 bytes long string to test out-of-bound checks on write.
pub const ASSERTION_TOO_BIG_DATA: &str = include_str!("../../data/data_256");
