//! Common utility structures and functions designed for testing
#![no_std]
extern crate alloc;

mod io;
mod read;
mod write;

pub use write::{assert_model_write, assert_model_write_ctx, assert_model_write_error};

pub use io::FaultyWriteBuffer;
pub use read::{assert_model_read, assert_model_read_ctx, assert_model_read_error};
