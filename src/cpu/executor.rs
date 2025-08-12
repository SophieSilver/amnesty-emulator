use num_enum::FromPrimitive;

use super::Cpu;
use crate::{
    cpu::instructions::{self, opcode::OpCode},
    memory::Memory,
};

/// CPU bundled together with memory
///
/// Used as a convenience wrapper for executing instructions
pub struct Executor<'a, M> {
    pub cpu: &'a mut Cpu,
    pub memory: &'a mut M,
}

impl<'a, M: Memory> Executor<'a, M> {
    pub fn execute_next_instruction(&mut self) {
        let opcode = self.fetch_from_pc_cycle();
        let opcode = OpCode::from_primitive(opcode);

        instructions::execute_opcode(self, opcode);
    }

    /// Fetch a value from the address PC points to and increment it
    ///
    /// Also correctly increment the `clock_cycle_count`
    pub fn fetch_from_pc_cycle(&mut self) -> u8 {
        let value = self.read_cycle(self.cpu.pc);
        self.cpu.pc = self.cpu.pc.wrapping_add(1);

        value
    }

    /// Perform a read cycle
    ///
    /// Reads from the address and increments the `clock_cycle_count`
    ///
    /// Every clock cycle a 6502 CPU either reads or writes to memory,
    /// as such this function or `write_cycle` should be called every time the clock cycle would be incremented
    /// in order to create the expected side effects of interacting with memory mapped hardware
    pub fn read_cycle(&mut self, addr: u16) -> u8 {
        let result = self.memory.load(addr);

        self.cpu.clock_cycle_count = self
            .cpu
            .clock_cycle_count
            .checked_add(1)
            .expect("clock_cycle can't overflow");

        result
    }

    /// Perform a write cycle
    ///
    /// Writes a value to the address and increments the `clock_cycle_count`
    ///
    /// Every clock cycle a 6502 CPU either reads or writes to memory,
    /// as such this function or `read_cycle` should be called every time the clock cycle would be incremented
    /// in order to create the expected side effects of interacting with memory mapped hardware
    pub fn write_cycle(&mut self, addr: u16, value: u8) {
        self.memory.store(addr, value);

        self.cpu.clock_cycle_count = self
            .cpu
            .clock_cycle_count
            .checked_add(1)
            .expect("clock_cycle can't overflow");
    }
}
