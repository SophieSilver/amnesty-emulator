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

use crate::{cpu::Cpu, memory::MemoryMapping};
use std::ops::ControlFlow;

pub(in crate::cpu) mod helpers;
mod templates;
use helpers::*;
use templates::*;

fn get_x_index(cpu: &Cpu) -> u8 {
    cpu.x_index
}

fn get_y_index(cpu: &Cpu) -> u8 {
    cpu.y_index
}

// LDA
// ==========================================
fn lda_common(cpu: &mut Cpu, value: u8) {
    set_register(&mut cpu.accumulator, value, &mut cpu.flags);
}

pub fn lda_immediate(cpu: &mut Cpu, memory: &mut MemoryMapping) -> ControlFlow<()> {
    read_immediate(cpu, memory, lda_common)
}

pub fn lda_zeropage(cpu: &mut Cpu, memory: &mut MemoryMapping) -> ControlFlow<()> {
    read_zeropage(cpu, memory, lda_common)
}

pub fn lda_zeropage_x(cpu: &mut Cpu, memory: &mut MemoryMapping) -> ControlFlow<()> {
    read_zeropage_indexed(cpu, memory, get_x_index, lda_common)
}

pub fn lda_absolute(cpu: &mut Cpu, memory: &mut MemoryMapping) -> ControlFlow<()> {
    read_absolute(cpu, memory, lda_common)
}

pub fn lda_absolute_x(cpu: &mut Cpu, memory: &mut MemoryMapping) -> ControlFlow<()> {
    read_absolute_indexed(cpu, memory, get_x_index, lda_common)
}

pub fn lda_absolute_y(cpu: &mut Cpu, memory: &mut MemoryMapping) -> ControlFlow<()> {
    read_absolute_indexed(cpu, memory, get_y_index, lda_common)
}

pub fn lda_indirect_x(cpu: &mut Cpu, memory: &mut MemoryMapping) -> ControlFlow<()> {
    read_indirect_x(cpu, memory, lda_common)
}

pub fn lda_indirect_y(cpu: &mut Cpu, memory: &mut MemoryMapping) -> ControlFlow<()> {
    read_indirect_y(cpu, memory, lda_common)
}

// LDX
// ==========================================
/// Closure passed into LDX templates
fn ldx_common(cpu: &mut Cpu, value: u8) {
    set_register(&mut cpu.x_index, value, &mut cpu.flags);
}

pub fn ldx_immediate(cpu: &mut Cpu, memory: &mut MemoryMapping) -> ControlFlow<()> {
    read_immediate(cpu, memory, ldx_common)
}

pub fn ldx_zeropage(cpu: &mut Cpu, memory: &mut MemoryMapping) -> ControlFlow<()> {
    read_zeropage(cpu, memory, ldx_common)
}

pub fn ldx_zeropage_y(cpu: &mut Cpu, memory: &mut MemoryMapping) -> ControlFlow<()> {
    read_zeropage_indexed(cpu, memory, get_y_index, ldx_common)
}

pub fn ldx_absolute(cpu: &mut Cpu, memory: &mut MemoryMapping) -> ControlFlow<()> {
    read_absolute(cpu, memory, ldx_common)
}

pub fn ldx_absolute_y(cpu: &mut Cpu, memory: &mut MemoryMapping) -> ControlFlow<()> {
    read_absolute_indexed(cpu, memory, get_y_index, ldx_common)
}
