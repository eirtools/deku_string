use deku_string::{Encoding, Size, StringLayout, VecLayout};

/// Elements in a vector with fixed layout
pub const FIXED_LENGTH: usize = 3;

/// Vector of very small size.
///
/// Should it even be allowed?
pub const LAYOUT_FIXED0: VecLayout = VecLayout::FixedLength(0);

/// Vector of fixed size
pub const LAYOUT_FIXED: VecLayout = VecLayout::FixedLength(FIXED_LENGTH);

/// Layout for Pascal-like string with 1 byte of length
pub const LAYOUT_PREFIX_U8: VecLayout = VecLayout::LengthPrefix(Size::U8);

/// Layout for Pascal-like string with 2 bytes of length
pub const LAYOUT_PREFIX_U16: VecLayout = VecLayout::LengthPrefix(Size::U16);

/// Layout for Pascal-like string with 4 bytes of length
pub const LAYOUT_PREFIX_U32: VecLayout = VecLayout::LengthPrefix(Size::U32);

/// Layout for .Net-like string with u32 length
pub const LAYOUT_PREFIX_U32_7BIT: VecLayout = VecLayout::LengthPrefix(Size::U32_7Bit);

/// Layout for C-like string
pub const LAYOUT_END: VecLayout = VecLayout::End;

/// Context to write [`deku_string::StringDeku`].
pub const STRING_CTX: (Encoding, StringLayout) =
    (Encoding::Utf8, StringLayout::LengthPrefix(Size::U16));
