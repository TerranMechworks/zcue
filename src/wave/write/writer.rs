use std::io::{Result, Write};

pub(crate) struct CountingWriter<W: Write> {
    inner: W,
    pub(crate) offset: usize,
}

impl<W: Write> CountingWriter<W> {
    #[inline]
    pub const fn new(write: W) -> Self {
        Self {
            inner: write,
            offset: 0,
        }
    }

    #[inline]
    pub fn into_inner(self) -> W {
        self.inner
    }

    #[inline(always)]
    pub fn write_all(&mut self, buf: &[u8]) -> Result<()> {
        self.offset += buf.len();
        self.inner.write_all(buf)
    }

    #[inline]
    pub fn write_u32(&mut self, value: u32) -> Result<()> {
        let buf = value.to_le_bytes();
        self.write_all(&buf)
    }

    #[inline]
    pub fn write_u16(&mut self, value: u16) -> Result<()> {
        let buf = value.to_le_bytes();
        self.write_all(&buf)
    }

    #[inline]
    pub fn write_u8(&mut self, value: u8) -> Result<()> {
        let buf = value.to_le_bytes();
        self.write_all(&buf)
    }
}
