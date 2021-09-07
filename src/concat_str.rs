#[cfg(test)]
mod unit_tests;

use core::cmp::min;
use core::{
    fmt::{Arguments as FmtArguments, Error as FmtError, Result as FmtResult, Write as FmtWrite},
    str::from_utf8_unchecked,
};

// Create a structure to allow concatenation of `str`s in a `#[no_std]` environment
#[derive(Debug)]
pub struct ConcatStr<'b> {
    buffer: &'b mut [u8],
    concatted_len: usize,
}

impl<'b> ConcatStr<'b> {
    pub fn new(buffer: &'b mut [u8]) -> Self {
        Self {
            buffer,
            concatted_len: 0,
        }
    }

    #[inline(always)]
    pub fn capacity(&self) -> usize {
        self.buffer.len()
    }

    pub fn into_str(self) -> Option<&'b str> {
        // `str` is only guaranteed to be valid UTF8 if `format` did not truncate
        match self.concatted_len > self.capacity() {
            true => None,
            false => Some(unsafe { from_utf8_unchecked(&self.buffer[..self.concatted_len]) }),
        }
    }

    // Idiomatic method on a container type
    #[allow(dead_code)]
    #[inline(always)]
    pub fn len(&self) -> usize {
        min(self.buffer.len(), self.concatted_len)
    }
}

impl FmtWrite for ConcatStr<'_> {
    fn write_str(&mut self, s: &str) -> FmtResult {
        match self.concatted_len > self.capacity() {
            true => Err(FmtError),
            false => {
                let unused_buffer = &mut self.buffer[self.concatted_len..];
                let byte_str = s.as_bytes();
                let len_to_write = min(byte_str.len(), unused_buffer.len());
                unused_buffer[..len_to_write].copy_from_slice(&byte_str[..len_to_write]);
                self.concatted_len += len_to_write;
                match len_to_write < byte_str.len() {
                    true => Err(FmtError),
                    false => Ok(()),
                }
            }
        }
    }
}

pub fn show<'b>(buffer: &'b mut [u8], args: FmtArguments) -> Result<&'b str, FmtError> {
    let mut cb = ConcatStr::new(buffer);
    write!(&mut cb, "{}", args)?;
    cb.into_str().ok_or(FmtError)
}
