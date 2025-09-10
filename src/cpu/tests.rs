#![allow(clippy::arithmetic_side_effects)]
use crate::memory::{Memory, ram::Ram};

mod addressing_modes;
mod flags;
mod test_args;

mod instructions;

#[derive(Debug, Clone)]
struct TestMemory {
    ram: Ram,
}

impl TestMemory {
    pub fn new() -> Self {
        Self { ram: Ram::new() }
    }
}

impl Memory for TestMemory {
    fn load(&mut self, address: u16) -> u8 {
        self.ram.load(address)
    }

    fn store(&mut self, address: u16, value: u8) {
        self.ram.store(address, value)
    }
}
