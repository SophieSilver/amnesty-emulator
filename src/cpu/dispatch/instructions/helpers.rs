//! Helper functions used in a variety of instructions

use crate::{
    cpu::{CpuState, StatusFlags},
    memory::MemoryMapping,
};

pub fn fetch_from_pc(cpu_state: &mut CpuState, memory: &mut MemoryMapping) -> u8 {
    let value = memory.load(cpu_state.program_counter);
    cpu_state.program_counter = cpu_state.program_counter.wrapping_add(1);

    value
}

/// Set a register to a value and also correctly set the Negative and Zero flags
pub fn set_register(register: &mut u8, value: u8, flags: &mut StatusFlags) {
    *register = value;
    flags.set(StatusFlags::NEGATIVE, (value as i8).is_negative());
    flags.set(StatusFlags::ZERO, value == 0);
}
