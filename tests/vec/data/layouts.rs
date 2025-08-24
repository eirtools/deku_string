use deku_string::{Encoding, Size, StringLayout, VecLayout};

pub const FIXED_LENGTH: usize = 3;

pub const LAYOUT_FIXED0: VecLayout = VecLayout::FixedLength(0);

pub const LAYOUT_FIXED: VecLayout = VecLayout::FixedLength(FIXED_LENGTH);

pub const LAYOUT_PREFIX_U8: VecLayout = VecLayout::LengthPrefix(Size::U8);
pub const LAYOUT_PREFIX_U16: VecLayout = VecLayout::LengthPrefix(Size::U16);
pub const LAYOUT_PREFIX_U32: VecLayout = VecLayout::LengthPrefix(Size::U32);
pub const LAYOUT_PREFIX_U32_7BIT: VecLayout = VecLayout::LengthPrefix(Size::U32_7Bit);

pub const LAYOUT_END: VecLayout = VecLayout::End;

pub const STRING_CTX: (Encoding, StringLayout) =
    (Encoding::Utf8, StringLayout::LengthPrefix(Size::U16));
