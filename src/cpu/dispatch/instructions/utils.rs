//! Utility functions used in a variety of instructions

use crate::{
    cpu::{Cpu, StatusFlags},
    memory::Memory,
};

/// Get the X index register, used as an argument to templates
pub fn get_x_index(cpu: &Cpu) -> u8 {
    cpu.x_index
}

/// Get the Y index register, used as an argument to templates
pub fn get_y_index(cpu: &Cpu) -> u8 {
    cpu.y_index
}

/// Add two numbers and a carry bit
pub fn add_with_carry(a: u8, b: u8, carry: bool) -> (u8, bool) {
    // this is equivalent to the implementation of the unstable `carrying_add`
    let (sum1, carry1) = a.overflowing_add(b);
    let (sum2, carry2) = sum1.overflowing_add(carry as u8);
    // note: no intermediate overflow is required (https://github.com/rust-lang/rust/issues/85532#issuecomment-1032214946).
    (sum2, carry1 | carry2)
}

/// Check if adding two numbers and a carry bit would result in signed overflow
pub fn add_would_overflow(a: i8, b: i8, carry: bool) -> bool {
    let (sum1, overflow1) = a.overflowing_add(b);
    let (_, overflow2) = sum1.overflowing_add(carry as i8);

    // note: no intermediate overflow is required (https://github.com/rust-lang/rust/issues/85532#issuecomment-1032214946).
    // the behavior described in the comment above can be observed at http://www.visual6502.org/JSSim/expert.html
    overflow1 != overflow2
}

/// Subtract two numbers and a carry bit, which acts as an inverse of the borrow flag
pub fn sub_with_carry(a: u8, b: u8, carry: bool) -> (u8, bool) {
    // see https://www.reddit.com/r/EmuDev/comments/k5hzuo/comment/gegnkq5/
    add_with_carry(a, !b, carry)
}

/// Check if subtracting two numbers and a carry bit would result in signed overflow
pub fn sub_would_overflow(a: i8, b: i8, carry: bool) -> bool {
    add_would_overflow(a, !b, carry)
}

/// Fetch a value from the address pointed to by the Program Counter and increment Program Counter
pub fn fetch_from_pc<M: Memory>(cpu: &mut Cpu, memory: &mut M) -> u8 {
    let value = memory.load(cpu.program_counter);
    cpu.program_counter = cpu.program_counter.wrapping_add(1);

    value
}

/// Set a register to a value and also correctly set the NEGATIVE and ZERO flags
pub fn set_register_with_flags(register: &mut u8, value: u8, flags: &mut StatusFlags) {
    *register = value;
    flags.set(StatusFlags::NEGATIVE, (value as i8).is_negative());
    flags.set(StatusFlags::ZERO, value == 0);
}

/// Automatically implement addressing modes
///
/// # Syntax
/// ```ignore
/// impl_addressing_modes! {
///     common: /*[common_fn]*/,
///     instruction_type: /*[instruction_type]*/,
///     modes: [
///         /*mode1*/,
///         /*mode2*/,
///         /*...*/,
///     ],
/// }
/// ```
/// where:
/// - `common_fn` - function to pass to every addressing mode template
/// - `instruction_type` - type of the instruction (read, rwm,  write, etc...)
/// - `mode1`, `mode2`, ... - addressing modes the instruction supports
/// # Output
/// for every mode will create a function called `mode`
/// which calls the template function `{instruction_type}::{mode}` with the provided common function
macro_rules! impl_addressing_modes {
    {
        common: $common:expr,
        instruction_type: $instruction_type:ident,
        modes: [
            $($mode:ident),*
            $(,)?   // allow trailing comma
        ] $(,)?     // allow trailing comma
    } => {
        $(
            pub fn $mode<M: $crate::memory::Memory>(
                cpu: &mut $crate::cpu::Cpu,
                memory: &mut M
            ) -> ControlFlow<()>
            {
               $crate::cpu::dispatch::instructions::templates::$instruction_type::$mode(cpu, memory, $common)
            }
        )*
    };
}
