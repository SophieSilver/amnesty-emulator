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
        /// Flag that isn't actually there in the hardware
        ///
        /// The emulator uses this flag in some places where it needs to store persist some simple
        /// boolean state between instruction cycles (i.e. absolute indexed addressing, where
        /// we add an additional cycle when a page boundary is crossed)
        ///
        /// Always pushed to the stack as 1
        const IGNORED_FLAG = 1 << 5;
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
pub struct CpuState {
    /// The currently executed instruction
    current_opcode: OpCode,

    /// Which cycle we're on within the current instruction
    current_cycle: u8,

    /// Used in various instructions that calculate the address to dereference
    effective_address: u16,

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

impl CpuState {
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

impl Default for CpuState {
    fn default() -> Self {
        CpuState {
            current_opcode: OpCode::Unimplemented,
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
