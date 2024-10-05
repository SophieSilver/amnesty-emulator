use bitflags::bitflags;
use dispatch::OpCode;

pub mod dispatch;

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
        const UNUSED = 1 << 5;
        const OVERFLOW = 1 << 6;
        const NEGATIVE = 1 << 7;
    }
}

impl Default for StatusFlags {
    fn default() -> Self {
        Self::UNUSED | Self::INTERRUPT_DISABLE
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CpuState {
    /// The currently executed instruction
    pub current_opcode: OpCode,

    /// Which cycle we're on within the current instruction
    pub current_cycle: u8,

    /// first operand if present
    pub operand1: u8,

    /// second operand if present
    pub operand2: u8,

    pub effective_address: u16,

    /// Accumulator register
    pub accumulator: u8,

    /// Index Register X
    pub x_index: u8,

    /// Index Register Y
    pub y_index: u8,

    /// Program Counter
    pub pc: u16,

    /// Stack Pointer
    pub stack_ptr: u8,

    /// Processor Status
    pub flags: StatusFlags,
}

impl CpuState {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for CpuState {
    fn default() -> Self {
        CpuState {
            current_opcode: OpCode::Unimplemented,
            current_cycle: 0,
            operand1: 0,
            operand2: 0,
            effective_address: 0,
            accumulator: 0,
            x_index: 0,
            y_index: 0,
            pc: 0,
            stack_ptr: 0,
            flags: StatusFlags::default(),
        }
    }
}
