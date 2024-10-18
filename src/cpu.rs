#![warn(clippy::arithmetic_side_effects)]

use std::ops::ControlFlow;

use bitflags::bitflags;
use dispatch::{dispatch_current_opcode, OpCode};

use crate::memory::MemoryMapping;

mod dispatch;
#[cfg(test)]
mod tests;

bitflags! {
    /// Status Flags used by the Processor Status register
    ///
    /// The description of the flags and their meanings can be found at
    /// [http://www.6502.org/users/obelisk/6502/registers.html] and
    /// https://www.nesdev.org/wiki/Status_flags
    #[derive(Debug, Clone, Copy)]
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

    #[derive(Debug, Clone, Copy)]
    struct InternalFlags: u8 {
        const EFFECTIVE_ADDR_CARRY = 1;
    }
}

impl Default for StatusFlags {
    fn default() -> Self {
        Self::INTERRUPT_DISABLE
    }
}

impl Default for InternalFlags {
    fn default() -> Self {
        Self::empty()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Cpu {
    /// The currently executed instruction
    current_opcode: OpCode,

    /// Which cycle we're on within the current instruction
    current_cycle: u8,

    /// The address of a pointer to dereference in indirect addressing modes
    pointer_address: u8,

    /// The address of a value to fetch in absolute and indirect addressing modes
    effective_address: u16,

    /// Used to keep simple boolean state internal to the emulator
    internal_flags: InternalFlags,

    /// Accumulator register
    pub accumulator: u8,

    /// Index Register X
    pub x_index: u8,

    /// Index Register Y
    pub y_index: u8,

    /// Program Counter
    pub program_counter: u16,

    /// Stack Pointer
    pub stack_ptr: u8,

    /// Processor Status
    pub flags: StatusFlags,
}

impl Cpu {
    pub fn new() -> Self {
        Self::default()
    }

    /// Advances the CPU state one clock cycle forward
    pub fn run_cycle(&mut self, memory: &mut MemoryMapping) {
        let instruction_status = dispatch_current_opcode(self, memory);

        match instruction_status {
            ControlFlow::Continue(()) => {
                self.current_cycle = self.current_cycle.wrapping_add(1);
            }
            ControlFlow::Break(_) => {
                self.current_cycle = 0;
            }
        }
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Cpu {
            current_opcode: OpCode::Unimplemented,
            internal_flags: InternalFlags::default(),
            pointer_address: 0,
            current_cycle: 0,
            effective_address: 0,
            accumulator: 0,
            x_index: 0,
            y_index: 0,
            program_counter: 0,
            stack_ptr: 0,
            flags: StatusFlags::default(),
        }
    }
}
