//!
//! Rejected write tests
//!
mod test_gen_macro;

use pastey::paste;
use rstest::rstest;

use deku::ctx::Endian;
use deku::writer::Writer;
use deku::{no_std_io, DekuWriter as _};

use deku_string::{Encoding, Layout, Size, StringDeku};

const FIXED_LENGTH: usize = 25;
const FIXED_NO_ZERO_INSIDE: &str = "invalid test case no zero";

const ZERO_INSIDE_STR: &str = "invalid test\x00with garbage";
// 16 * 4096 = 65536 = u16::MAX + 1
const TOO_LONG_64K: &str = rep64k!("________________", 4096);

create_test_impl_write_rejected! {
    fixed_force_zero_no_null,
    Layout::FixedLength {
        size: FIXED_LENGTH,
        allow_no_null:false,
    },
    (no_null_inside, FIXED_NO_ZERO_INSIDE),
}

create_test_impl_write_rejected! {
    fixed_force_zero,
    Layout::FixedLength {
        size: FIXED_LENGTH,
        allow_no_null:false,
    },
    (not_zero_ended, ZERO_INSIDE_STR),
    (too_long, TOO_LONG_64K),
}

create_test_impl_write_rejected! {
    fixed_allow_zero,
    Layout::FixedLength {
        size: FIXED_LENGTH,
        allow_no_null:true,
    },
    (not_zero_ended, ZERO_INSIDE_STR),
    (too_long, TOO_LONG_64K),
}

create_test_impl_write_rejected! {
    prefix_u8,
    Layout::LengthPrefix(Size::U8),
    (not_zero_ended, ZERO_INSIDE_STR),
    (too_long, TOO_LONG_64K),
}

create_test_impl_write_rejected! {
    prefix_u16,
    Layout::LengthPrefix(Size::U16),
    (not_zero_ended, ZERO_INSIDE_STR),
    (too_long, TOO_LONG_64K),
}

create_test_impl_write_rejected! {
    prefix_u32,
    Layout::LengthPrefix(Size::U32),
    (not_zero_ended, ZERO_INSIDE_STR),
    // it's not worth a while to test this due requirements to save 4GB string
    // (too_long, TOO_LONG_64K),
}

create_test_impl_write_rejected! {
    zero_ended,
    Layout::ZeroEnded,
    (not_zero_ended, ZERO_INSIDE_STR),
}

// -------

create_test_impl_write_io_rejected! {
    fixed_force_zero,
    Layout::FixedLength {
        size: FIXED_LENGTH,
        allow_no_null:false,
    },
}

create_test_impl_write_io_rejected! {
    fixed_allow_zero,
    Layout::FixedLength {
        size: FIXED_LENGTH,
        allow_no_null:true,
    },
}

create_test_impl_write_io_rejected! {
    prefix_u8,
    Layout::LengthPrefix(Size::U8),
}

create_test_impl_write_io_rejected! {
    prefix_u16,
    Layout::LengthPrefix(Size::U16),
}

create_test_impl_write_io_rejected! {
    prefix_u32,
    Layout::LengthPrefix(Size::U32),
}

create_test_impl_write_io_rejected! {
    zero_ended,
    Layout::ZeroEnded,
}

struct InvalidBufferType {}

impl std::io::Write for InvalidBufferType {
    fn write(&mut self, _buf: &[u8]) -> std::io::Result<usize> {
        Err(std::io::ErrorKind::BrokenPipe.into())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Err(std::io::ErrorKind::BrokenPipe.into())
    }
}

impl std::io::Seek for InvalidBufferType {
    fn seek(&mut self, _pos: std::io::SeekFrom) -> std::io::Result<u64> {
        Err(std::io::ErrorKind::BrokenPipe.into())
    }
}
