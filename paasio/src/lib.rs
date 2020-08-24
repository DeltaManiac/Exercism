use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    wrapped: R,
    read: usize,
    count: usize,
}

impl<R: Read> ReadStats<R> {
    // _wrapped is ignored because R is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: R) -> ReadStats<R> {
        ReadStats {
            wrapped,
            read: 0,
            count: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.read
    }

    pub fn reads(&self) -> usize {
        self.count
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.count += 1;
        let read = self.wrapped.read(buf)?;
        self.read += read;
        Ok(read)
    }
}
pub struct WriteStats<W> {
    wrapped: W,
    written: usize,
    count: usize,
}

impl<W: Write> WriteStats<W> {
    pub fn new(wrapped: W) -> WriteStats<W> {
        WriteStats {
            wrapped,
            written: 0,
            count: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.written
    }

    pub fn writes(&self) -> usize {
        self.count
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.count += 1;
        let written = self.wrapped.write(buf)?;
        self.written += written;
        Ok(written)
    }

    fn flush(&mut self) -> Result<()> {
        self.wrapped.flush()
    }
}
