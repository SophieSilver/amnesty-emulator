use std::{
    fmt::{Debug, Formatter},
    ops::{Index, IndexMut},
};

const RAM_SIZE: usize = 2048;

#[derive(Clone)]
pub struct Ram {
    buf: Box<[u8; RAM_SIZE]>,
}

impl Ram {
    pub fn new() -> Self {
        Self {
            buf: Box::new([0; RAM_SIZE]),
        }
    }

    #[must_use]
    pub fn load(&self, addr: u16) -> Option<u8> {
        self.buf.get(addr as usize).copied()
    }

    /// Write a byte to an address
    pub fn store(&mut self, addr: u16, new_value: u8) -> Option<u8> {
        let result = self.load(addr);
        if result.is_some() {
            // index is in bounds, safe to look up
            self.buf[addr as usize] = new_value;
        }

        result
    }
}

impl Index<u16> for Ram {
    type Output = u8;
    fn index(&self, index: u16) -> &Self::Output {
        &self.buf[index as usize]
    }
}

impl IndexMut<u16> for Ram {
    fn index_mut(&mut self, index: u16) -> &mut Self::Output {
        &mut self.buf[index as usize]
    }
}

impl Debug for Ram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let write_slice = |f: &mut Formatter, slice: &[u8]| {
            for (i, n) in slice.iter().copied().enumerate() {
                write!(f, "{n:0>2X}{}", if i == slice.len() - 1 { "" } else { " " })?
            }
            writeln!(f)?;
            Ok(())
        };

        writeln!(f, "[")?;
        write!(f, "{:>8} | ", "")?;
        write_slice(f, &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15])?;
        writeln!(f, "{empty:->8}-+-{empty:->47}", empty = "")?;
        for (i, chunk) in self.buf.chunks(16).enumerate() {
            let offset = i * 16;
            write!(f, "{offset:0>8X} | ")?;
            write_slice(f, chunk)?;
        }
        writeln!(f, "]")?;

        Ok(())
    }
}

impl Default for Ram {
    fn default() -> Self {
        Self::new()
    }
}
