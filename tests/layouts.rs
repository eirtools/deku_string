use deku_string::{Layout, Size};

/// How many bytes (for UTF-8) or u16 (for UTF-16) should be present in the buffer
pub const FIXED_LENGTH: usize = 23;

/// Layout for fixed string buffer with enforced zero byte presence
pub const LAYOUT_FIXED_FORCE_ZERO: Layout = Layout::FixedLength {
    size: FIXED_LENGTH,
    allow_no_null: false,
};

/// Layout for fixed string buffer with non-enforced zero byte presence
pub const LAYOUT_FIXED_ALLOW_NO_ZERO: Layout = Layout::FixedLength {
    size: FIXED_LENGTH,
    allow_no_null: true,
};

/// Layout for Pascal-like string with 1 byte of length
pub const LAYOUT_PREFIX_U8: Layout = Layout::LengthPrefix(Size::U8);

/// Layout for Pascal-like string with 2 bytes of length
pub const LAYOUT_PREFIX_U16: Layout = Layout::LengthPrefix(Size::U16);

/// Layout for Pascal-like string with 4 bytes of length
pub const LAYOUT_PREFIX_U32: Layout = Layout::LengthPrefix(Size::U32);

/// Layout for C-like string
pub const LAYOUT_ZERO_ENDED: Layout = Layout::ZeroEnded;
