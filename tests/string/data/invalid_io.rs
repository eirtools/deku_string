use std::io::ErrorKind;
use std::io::Result;

use deku::no_std_io::Seek;
use deku::no_std_io::SeekFrom;
use deku::no_std_io::Write;

pub struct InvalidBufferType {
    size: u64,
    pos: u64,
}

impl InvalidBufferType {
    pub fn new(size: u64) -> Self {
        InvalidBufferType { size, pos: 0 }
    }
}

impl Write for InvalidBufferType {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let max_bytes = self.size - self.pos;
        let to_write = buf.len().min(max_bytes as usize) as u64; // We will only write up to the allowed size
        self.pos += to_write; // Decrease the remaining allowed bytes

        if self.pos >= self.size {
            Err(ErrorKind::BrokenPipe.into())
        } else {
            Ok(to_write as usize)
        }
    }

    fn flush(&mut self) -> Result<()> {
        // For the sake of simplicity, we'll just flush as a no-op
        Ok(())
    }
}

impl Seek for InvalidBufferType {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        match pos {
            SeekFrom::Start(offset) => {
                if offset <= self.size {
                    self.pos = offset;
                    Ok(offset)
                } else {
                    Err(ErrorKind::BrokenPipe.into())
                }
            }
            SeekFrom::Current(offset) => {
                let new_pos = self.pos as i64 + offset;

                if new_pos >= 0 && new_pos as u64 <= self.size {
                    self.pos = new_pos as u64;
                    return Ok(self.pos)
                }
                Err(ErrorKind::BrokenPipe.into())
            }
            SeekFrom::End(offset) => {
                if offset > 0 {
                    return Err(ErrorKind::BrokenPipe.into())
                }

                let offset =  (-offset) as u64;
                if offset > self.size {
                    return Err(ErrorKind::BrokenPipe.into())
                }
                self.pos = self.size - offset;
                Ok(offset)
            }
        }
    }
}
