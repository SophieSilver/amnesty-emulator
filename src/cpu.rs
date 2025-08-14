#![warn(clippy::arithmetic_side_effects)]

use bitflags::bitflags;

use crate::{cpu::executor::Executor, memory::Memory};

mod arithmetic;
mod executor;
mod instructions;

#[cfg(test)]
mod tests;

bitflags! {
    /// Status Flags used by the Processor Status register
    ///
    /// The description of the flags and their meanings can be found at
    /// [http://www.6502.org/users/obelisk/6502/registers.html] and
    /// https://www.nesdev.org/wiki/Status_flags
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct StatusFlags: u8 {
        const CARRY = 1;
        const ZERO = 1 << 1;
        const INTERRUPT_DISABLE = 1 << 2;
        const DECIMAL = 1 << 3;
        const BREAK = 1 << 4;
        const IGNORED = 1 << 5;
        const OVERFLOW = 1 << 6;
        const NEGATIVE = 1 << 7;
    }
}

impl Default for StatusFlags {
    fn default() -> Self {
        Self::INTERRUPT_DISABLE
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Cpu {
    /// Counts how many clock cycles have been executed
    pub clock_cycle_count: u64,

    /// A (Accumulator) Register
    pub a: u8,

    /// X (Index) Register
    pub x: u8,

    /// Y (Index) Register
    pub y: u8,

    /// PC (Program Counter) Register
    ///
    /// Sometimes also Called IP or Instruction Pointer
    pub pc: u16,

    /// SP (Stack Pointer) Register
    pub sp: u8,

    /// Processor Status Register
    pub flags: StatusFlags,
}

impl Cpu {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn execute_next_instruction<M: Memory>(&mut self, memory: &mut M) {
        let mut executor = Executor { cpu: self, memory };
        executor.execute_next_instruction();
    }

    fn set_register_with_flags(
        &mut self,
        get_register: impl FnOnce(&mut Cpu) -> &mut u8,
        value: u8,
    ) {
        *get_register(self) = value;

        self.flags
            .set(StatusFlags::NEGATIVE, (value as i8).is_negative());
        self.flags.set(StatusFlags::ZERO, value == 0);
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Cpu {
            a: 0,
            x: 0,
            y: 0,
            pc: 0,
            sp: 0xFF,
            flags: StatusFlags::default(),
            clock_cycle_count: 0,
        }
    }
}

mod register_getters {
    use crate::cpu::Cpu;

    pub fn a_register(cpu: &mut Cpu) -> &mut u8 {
        &mut cpu.a
    }

    pub fn x_register(cpu: &mut Cpu) -> &mut u8 {
        &mut cpu.x
    }
    pub fn y_register(cpu: &mut Cpu) -> &mut u8 {
        &mut cpu.y
    }
}
