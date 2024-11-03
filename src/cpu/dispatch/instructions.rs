//! Implementes instructions that interract with the cpu and the memory
//!
//! # The way this all works
//!
//! All instructions takes a number of cycles to execute, so there is a scheme
//! which allows us to persist the instruction state through multiple cycles
//!
//! Every instruction follows the same basic template:
//! - it matches on the current cycle within the instruction and performs some action
//! - returns a `ControlFlow`:
//! - - `Continue` means to increment the current cycle, the current instruction will
//!     continue executing in the next cycle
//! - - `Break` means the instruction is finished, the current cycle will be reset
//!     and the next opcode will be fetched by the dispatch
//!
//! ## Notes
//! Cycle 0 is always fetching the opcode, every match should start with cycle 1
//! Last match arm should always return ControlFlow::Break(());

use crate::{
    cpu::{Cpu, StatusFlags},
    memory::MemoryMapping,
};
use std::ops::ControlFlow;

#[macro_use]
pub(in crate::cpu) mod helpers;
mod templates;
use helpers::*;
use templates::implied;

pub mod adc;
pub mod and;
pub mod bit;
pub mod cmp;
pub mod eor;
pub mod lda;
pub mod ldx;
pub mod ldy;
pub mod ora;
pub mod sbc;

// implied instructions

pub fn clc(cpu: &mut Cpu, memory: &mut MemoryMapping) -> ControlFlow<()> {
    implied(cpu, memory, |cpu| {
        cpu.flags.remove(StatusFlags::CARRY);
    })
}

pub fn cld(cpu: &mut Cpu, memory: &mut MemoryMapping) -> ControlFlow<()> {
    implied(cpu, memory, |cpu| {
        cpu.flags.remove(StatusFlags::DECIMAL);
    })
}

pub fn cli(cpu: &mut Cpu, memory: &mut MemoryMapping) -> ControlFlow<()> {
    implied(cpu, memory, |cpu| {
        cpu.flags.remove(StatusFlags::INTERRUPT_DISABLE);
    })
}

pub fn clv(cpu: &mut Cpu, memory: &mut MemoryMapping) -> ControlFlow<()> {
    implied(cpu, memory, |cpu| {
        cpu.flags.remove(StatusFlags::OVERFLOW);
    })
}

pub fn sec(cpu: &mut Cpu, memory: &mut MemoryMapping) -> ControlFlow<()> {
    implied(cpu, memory, |cpu| {
        cpu.flags.insert(StatusFlags::CARRY);
    })
}

pub fn sed(cpu: &mut Cpu, memory: &mut MemoryMapping) -> ControlFlow<()> {
    implied(cpu, memory, |cpu| {
        cpu.flags.insert(StatusFlags::DECIMAL);
    })
}

pub fn sei(cpu: &mut Cpu, memory: &mut MemoryMapping) -> ControlFlow<()> {
    implied(cpu, memory, |cpu| {
        cpu.flags.insert(StatusFlags::INTERRUPT_DISABLE);
    })
}

pub fn nop(cpu: &mut Cpu, memory: &mut MemoryMapping) -> ControlFlow<()> {
    implied(cpu, memory, |_| {})
}
