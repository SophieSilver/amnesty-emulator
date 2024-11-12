//! Template functions for implementing instructions
//!
//! Most instructions have commonalities, such as memory access patterns
//!
//! This module defines template instructions that implement the common functionality between instructions
//! by taking a closure as an argument that performs the actual work after the addressing is done

use std::ops::ControlFlow;

use crate::{
    cpu::{Cpu, InternalFlags},
    memory::Memory,
};

use super::{fetch_from_pc, get_x_index, get_y_index};

pub mod read;
pub mod write;

pub fn implied<M: Memory, F: FnOnce(&mut Cpu)>(
    cpu: &mut Cpu,
    memory: &mut M,
    f: F,
) -> ControlFlow<()> //
{
    match cpu.current_cycle {
        1 => {
            _ = memory.load(cpu.program_counter);
            f(cpu);

            ControlFlow::Break(())
        }

        _ => unreachable!(),
    }
}
