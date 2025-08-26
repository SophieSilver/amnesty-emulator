#![allow(clippy::arithmetic_side_effects)]
use utils::*;

use crate::{
    cpu::instructions::opcode::OpCode,
    memory::{ram::Ram, Memory},
};

use super::StatusFlags;

#[allow(dead_code)]
mod addrs;
#[allow(dead_code)]
mod utils;

mod addressing_modes;
mod test_args;

mod adc;
mod and;
mod bit;
mod cmp;
mod eor;
mod lda;
mod ldx;
mod ldy;
mod ora;
mod sbc;
mod sta;
mod stx;
mod sty;

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

mod implied;
