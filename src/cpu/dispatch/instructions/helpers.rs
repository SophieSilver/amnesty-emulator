//! Helper functions used in a variety of instructions

use crate::{
    cpu::{Cpu, StatusFlags},
    memory::MemoryMapping,
};

/// Add 2 numbers and a carry bit
pub fn add_with_carry(a: u8, b: u8, carry: bool) -> (u8, bool) {
    // this is equivalent to the implementation of the unstable `carrying_add`
    let (sum1, carry1) = a.overflowing_add(b);
    let (sum2, carry2) = sum1.overflowing_add(carry as u8);
    // note: no intermediate overflow is required (https://github.com/rust-lang/rust/issues/85532#issuecomment-1032214946).
    (sum2, carry1 | carry2)
}

pub fn add_would_overflow(a: i8, b: i8, carry: bool) -> bool {
    let (sum1, overflow1) = a.overflowing_add(b);
    let (_, overflow2) = sum1.overflowing_add(carry as i8);

    // note: no intermediate overflow is required (https://github.com/rust-lang/rust/issues/85532#issuecomment-1032214946).
    // the behavior described in the comment above can be observed at http://www.visual6502.org/JSSim/expert.html
    overflow1 != overflow2
}

pub fn fetch_from_pc(cpu: &mut Cpu, memory: &mut MemoryMapping) -> u8 {
    let value = memory.load(cpu.program_counter);
    cpu.program_counter = cpu.program_counter.wrapping_add(1);

    value
}

/// Set a register to a value and also correctly set the Negative and Zero flags
pub fn set_register(register: &mut u8, value: u8, flags: &mut StatusFlags) {
    *register = value;
    flags.set(StatusFlags::NEGATIVE, (value as i8).is_negative());
    flags.set(StatusFlags::ZERO, value == 0);
}
