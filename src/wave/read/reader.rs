use super::ChunkId;
use std::io::{Read, Result};

pub(crate) struct CountingReader<R: Read> {
    inner: R,
    pub(crate) offset: u32,
    pub(crate) prev: u32,
}

impl<R: Read> CountingReader<R> {
    #[inline]
    pub(crate) const fn new(read: R) -> Self {
        Self {
            inner: read,
            offset: 0,
            prev: 0,
        }
    }

    #[inline]
    pub(crate) fn read_exact(&mut self, buf: &mut [u8]) -> Result<()> {
        self.inner.read_exact(buf)?;
        self.prev = self.offset;
        self.offset += buf.len() as u32;
        Ok(())
    }

    #[inline]
    pub(crate) fn read_chunk_id(&mut self) -> Result<ChunkId> {
        let mut buf = [0; 4];
        self.read_exact(&mut buf)?;
        Ok(ChunkId::new(buf))
    }

    #[inline]
    pub(crate) fn read_u32(&mut self) -> Result<u32> {
        let mut buf = [0; 4];
        self.read_exact(&mut buf)?;
        Ok(u32::from_le_bytes(buf))
    }

    #[inline]
    pub(crate) fn read_u16(&mut self) -> Result<u16> {
        let mut buf = [0; 2];
        self.read_exact(&mut buf)?;
        Ok(u16::from_le_bytes(buf))
    }

    #[inline]
    pub(crate) fn read_u8(&mut self) -> Result<u8> {
        let mut buf = [0; 1];
        self.read_exact(&mut buf)?;
        Ok(u8::from_le_bytes(buf))
    }
}
