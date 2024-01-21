use std::fmt::{self, Debug};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkId([u8; 4]);

impl ChunkId {
    #[inline]
    pub const fn new(inner: [u8; 4]) -> Self {
        Self(inner)
    }
}

impl fmt::Debug for ChunkId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0.is_ascii() {
            // SAFETY: it's ASCII
            let s = unsafe { std::str::from_utf8_unchecked(&self.0) };
            Debug::fmt(s, f)
        } else if f.alternate() {
            write!(f, "{:#08X}", u32::from_be_bytes(self.0))
        } else {
            write!(f, "{:08X}", u32::from_be_bytes(self.0))
        }
    }
}

impl fmt::Display for ChunkId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0.is_ascii() {
            // SAFETY: it's ASCII
            let s = unsafe { std::str::from_utf8_unchecked(&self.0) };
            f.write_str(s)
        } else if f.alternate() {
            write!(f, "{:#08X}", u32::from_be_bytes(self.0))
        } else {
            write!(f, "{:08X}", u32::from_be_bytes(self.0))
        }
    }
}

impl AsRef<[u8]> for ChunkId {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

#[cfg(test)]
mod tests;
