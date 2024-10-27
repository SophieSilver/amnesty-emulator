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

fn get_x_index(cpu: &Cpu) -> u8 {
    cpu.x_index
}

fn get_y_index(cpu: &Cpu) -> u8 {
    cpu.y_index
}

pub mod adc;
pub mod and;
pub mod cmp;
pub mod eor;
pub mod lda;
pub mod ldx;
pub mod ldy;
