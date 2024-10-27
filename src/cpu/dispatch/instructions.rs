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

// AND
fn and_common(cpu: &mut Cpu, value: u8) {
    let new_accumulator = cpu.accumulator & value;
    set_register(&mut cpu.accumulator, new_accumulator, &mut cpu.flags);
}

pub fn and_immediate(cpu: &mut Cpu, memory: &mut MemoryMapping) -> ControlFlow<()> {
    read_immediate(cpu, memory, and_common)
}

pub fn and_zeropage(cpu: &mut Cpu, memory: &mut MemoryMapping) -> ControlFlow<()> {
    read_zeropage(cpu, memory, and_common)
}

pub fn and_zeropage_x(cpu: &mut Cpu, memory: &mut MemoryMapping) -> ControlFlow<()> {
    read_zeropage_indexed(cpu, memory, get_x_index, and_common)
}

pub fn and_absolute(cpu: &mut Cpu, memory: &mut MemoryMapping) -> ControlFlow<()> {
    read_absolute(cpu, memory, and_common)
}

pub fn and_absolute_x(cpu: &mut Cpu, memory: &mut MemoryMapping) -> ControlFlow<()> {
    read_absolute_indexed(cpu, memory, get_x_index, and_common)
}

pub fn and_absolute_y(cpu: &mut Cpu, memory: &mut MemoryMapping) -> ControlFlow<()> {
    read_absolute_indexed(cpu, memory, get_y_index, and_common)
}

pub fn and_indirect_x(cpu: &mut Cpu, memory: &mut MemoryMapping) -> ControlFlow<()> {
    read_indirect_x(cpu, memory, and_common)
}

pub fn and_indirect_y(cpu: &mut Cpu, memory: &mut MemoryMapping) -> ControlFlow<()> {
    read_indirect_y(cpu, memory, and_common)
}
// ADC
fn adc_common(cpu: &mut Cpu, value: u8) {
    let (sum, carry) = add_with_carry(
        cpu.accumulator,
        value,
        cpu.flags.contains(StatusFlags::CARRY),
    );

    let overflow = add_would_overflow(
        cpu.accumulator as i8,
        value as i8,
        cpu.flags.contains(StatusFlags::CARRY),
    );

    cpu.flags.set(StatusFlags::CARRY, carry);
    cpu.flags.set(StatusFlags::OVERFLOW, overflow);

    set_register(&mut cpu.accumulator, sum, &mut cpu.flags);
}

pub fn adc_immediate(cpu: &mut Cpu, memory: &mut MemoryMapping) -> ControlFlow<()> {
    read_immediate(cpu, memory, adc_common)
}

pub fn adc_zeropage(cpu: &mut Cpu, memory: &mut MemoryMapping) -> ControlFlow<()> {
    read_zeropage(cpu, memory, adc_common)
}

pub fn adc_zeropage_x(cpu: &mut Cpu, memory: &mut MemoryMapping) -> ControlFlow<()> {
    read_zeropage_indexed(cpu, memory, get_x_index, adc_common)
}

pub fn adc_absolute(cpu: &mut Cpu, memory: &mut MemoryMapping) -> ControlFlow<()> {
    read_absolute(cpu, memory, adc_common)
}

pub fn adc_absolute_x(cpu: &mut Cpu, memory: &mut MemoryMapping) -> ControlFlow<()> {
    read_absolute_indexed(cpu, memory, get_x_index, adc_common)
}

pub fn adc_absolute_y(cpu: &mut Cpu, memory: &mut MemoryMapping) -> ControlFlow<()> {
    read_absolute_indexed(cpu, memory, get_y_index, adc_common)
}

pub fn adc_indirect_x(cpu: &mut Cpu, memory: &mut MemoryMapping) -> ControlFlow<()> {
    read_indirect_x(cpu, memory, adc_common)
}

pub fn adc_indirect_y(cpu: &mut Cpu, memory: &mut MemoryMapping) -> ControlFlow<()> {
    read_indirect_y(cpu, memory, adc_common)
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

// LDY
// ==========================================

fn ldy_common(cpu: &mut Cpu, value: u8) {
    set_register(&mut cpu.y_index, value, &mut cpu.flags);
}

pub fn ldy_immediate(cpu: &mut Cpu, memory: &mut MemoryMapping) -> ControlFlow<()> {
    read_immediate(cpu, memory, ldy_common)
}

pub fn ldy_zeropage(cpu: &mut Cpu, memory: &mut MemoryMapping) -> ControlFlow<()> {
    read_zeropage(cpu, memory, ldy_common)
}

pub fn ldy_zeropage_x(cpu: &mut Cpu, memory: &mut MemoryMapping) -> ControlFlow<()> {
    read_zeropage_indexed(cpu, memory, get_x_index, ldy_common)
}

pub fn ldy_absolute(cpu: &mut Cpu, memory: &mut MemoryMapping) -> ControlFlow<()> {
    read_absolute(cpu, memory, ldy_common)
}

pub fn ldy_absolute_x(cpu: &mut Cpu, memory: &mut MemoryMapping) -> ControlFlow<()> {
    read_absolute_indexed(cpu, memory, get_x_index, ldy_common)
}
