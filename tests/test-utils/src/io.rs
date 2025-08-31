//! Faulty writer buffers
#![allow(clippy::cast_possible_wrap, reason = "Checked")]
#![allow(clippy::cast_sign_loss, reason = "Checked")]
#![allow(clippy::cast_possible_truncation, reason = "usize <= u64")]

use std::io::{ErrorKind, Result}; // Deku doesn't re-import these two.

use deku::no_std_io::{Seek, SeekFrom, Write};

/// Write buffer which have only `byte_breaks` bytes to be written.
///
/// All data is discarded, only position is changed.
///
/// [`ErrorKind::BrokenPipe`] error is returned.
#[must_use]
pub struct FaultyWriteBuffer {
    /// byte position at after which error will be returned.
    byte_breaks: u64,

    /// Current position.
    pos: u64,
}

impl FaultyWriteBuffer {
    #[inline]
    pub const fn new(byte_breaks: u64) -> Self {
        Self {
            byte_breaks,
            pos: 0,
        }
    }
}

impl Write for FaultyWriteBuffer {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        #[allow(clippy::as_conversions, reason = "Assume usize <= u64")]
        let to_write = buf.len() as u64;

        self.pos += to_write;

        if self.pos >= self.byte_breaks {
            Err(ErrorKind::BrokenPipe.into())
        } else {
            Ok(buf.len())
        }
    }

    #[inline]
    fn flush(&mut self) -> Result<()> {
        // For the sake of simplicity, we'll just flush as a no-op
        Ok(())
    }
}

impl Seek for FaultyWriteBuffer {
    #[inline]
    fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        match pos {
            SeekFrom::Start(offset) => {
                if offset <= self.byte_breaks {
                    self.pos = offset;
                    Ok(offset)
                } else {
                    Err(ErrorKind::BrokenPipe.into())
                }
            }
            SeekFrom::Current(offset) => {
                #[allow(
                    clippy::as_conversions,
                    reason = "Assume that current position is less than i64"
                )]
                let new_pos = self.pos as i64 + offset;

                #[allow(clippy::as_conversions, reason = "Checked")]
                if new_pos >= 0 && new_pos as u64 <= self.byte_breaks {
                    self.pos = new_pos as u64;
                    return Ok(self.pos);
                }

                Err(ErrorKind::BrokenPipe.into())
            }
            SeekFrom::End(offset) => {
                if offset > 0 {
                    return Err(ErrorKind::BrokenPipe.into());
                }

                #[allow(clippy::as_conversions, reason = "Checked")]
                let positive_offset = (-offset) as u64;

                self.pos = self.byte_breaks - positive_offset;

                if positive_offset > self.byte_breaks {
                    return Err(ErrorKind::BrokenPipe.into());
                }

                Ok(self.pos)
            }
        }
    }
}
