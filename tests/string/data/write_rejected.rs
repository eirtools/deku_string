pub const ASSERTION_NO_ZERO_INSIDE: &str = "invalid test case zero?";

pub const ASSERTION_ZERO_INSIDE_STR: &str = "invalid test\x00with garbage";
///
/// A 65536+ bytes (64KB)-long string to test out-of-bound checks on write.
pub const ASSERTION_TOO_BIG_DATA: &str = include_str!("../../64k_data");

pub const IO_DATA: &str = "valid test case w zero";

pub const IO_FIXED_FORCE_ZERO_DATA_SIZE: u64 = 0;
pub const IO_FIXED_ALLOW_NO_ZERO_DATA_SIZE: u64 = 0;
pub const IO_PREFIX_U8_DATA_SIZE: u64 = 3;
pub const IO_PREFIX_U16_DATA_SIZE: u64 = 4;
pub const IO_PREFIX_U32_DATA_SIZE: u64 = 5;
pub const IO_ZERO_ENDED_DATA_SIZE: u64 = 0;

pub const IO_PREFIX: &str = "value";

//pub const IO_FIXED_PREFIX_SIZE — no prefix
pub const IO_PREFIX_U8_PREFIX_SIZE: u64 = 0;
pub const IO_PREFIX_U16_PREFIX_SIZE: u64 = 0;
pub const IO_PREFIX_U32_PREFIX_SIZE: u64 = 0;
// pub const IO_ZERO_ENDED_PREFIX_SIZE -- no prefix

pub const IO_SUFFIX: &str = "valid test case w zero";
pub const IO_ZERO_ENDED_SUFFIX_SIZE: u64 = 23;
