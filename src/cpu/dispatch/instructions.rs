use std::ops::ControlFlow;

use microops::load_operand1;

use crate::{cpu::CpuState, memory::MemoryMapping};

mod microops;

pub fn ldx_immediate(cpu_state: &mut CpuState, memory: &mut MemoryMapping) -> ControlFlow<(), ()> {
    match cpu_state.current_cycle {
        0 => {
           load_operand1(cpu_state, memory);
           ControlFlow::Continue(())
        }
        1 => {
            cpu_state.x_index = cpu_state.operand1;
            ControlFlow::Break(())
        }
        _ => unreachable!(),
    }
}
