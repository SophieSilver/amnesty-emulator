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
//! continue executing in the next cycle
//! - - `Break` means the instruction is finished, the current cycle will be reset
//! and the next opcode will be fetched by the dispatch
//!
//! ## Notes
//! Cycle 0 is always fetching the opcode, every match should start with cycle 1
//! Last match arm should always return ControlFlow::Break(());

use helpers::{fetch_from_pc, set_register};

use crate::{
    cpu::{CpuState, StatusFlags},
    memory::MemoryMapping,
};
use std::ops::ControlFlow;

pub(in crate::cpu) mod helpers;

// LDX

pub fn ldx_immediate(cpu_state: &mut CpuState, memory: &mut MemoryMapping) -> ControlFlow<()> {
    match cpu_state.current_cycle {
        1 => {
            let value = fetch_from_pc(cpu_state, memory);
            set_register(&mut cpu_state.x_index, value, &mut cpu_state.flags);

            ControlFlow::Break(())
        }
        _ => unreachable!(),
    }
}

pub fn ldx_zeropage(cpu_state: &mut CpuState, memory: &mut MemoryMapping) -> ControlFlow<()> {
    match cpu_state.current_cycle {
        1 => {
            cpu_state.effective_address = fetch_from_pc(cpu_state, memory) as u16;
        }
        2 => {
            let value = memory.load(cpu_state.effective_address);
            set_register(&mut cpu_state.x_index, value, &mut cpu_state.flags);

            return ControlFlow::Break(());
        }
        _ => unreachable!(),
    };

    ControlFlow::Continue(())
}

pub fn ldx_zeropage_y(cpu_state: &mut CpuState, memory: &mut MemoryMapping) -> ControlFlow<()> {
    match cpu_state.current_cycle {
        1 => {
            cpu_state.effective_address = fetch_from_pc(cpu_state, memory) as u16;
        }
        2 => {
            // dummy read coz every cycle is a read or a write
            let _ = memory.load(cpu_state.effective_address);
            cpu_state.effective_address += cpu_state.y_index as u16;
            // upper byte is always 0, page overflow is ignored
            cpu_state.effective_address &= 0xFF;
        }
        3 => {
            let value = memory.load(cpu_state.effective_address);
            set_register(&mut cpu_state.x_index, value, &mut cpu_state.flags);

            return ControlFlow::Break(());
        }
        _ => unreachable!(),
    };

    ControlFlow::Continue(())
}

pub fn ldx_absolute(cpu_state: &mut CpuState, memory: &mut MemoryMapping) -> ControlFlow<()> {
    match cpu_state.current_cycle {
        1 => {
            cpu_state.effective_address = fetch_from_pc(cpu_state, memory) as u16;
        }
        2 => {
            cpu_state.effective_address |= (fetch_from_pc(cpu_state, memory) as u16) << 8;
        }
        3 => {
            let value = memory.load(cpu_state.effective_address);
            set_register(&mut cpu_state.x_index, value, &mut cpu_state.flags);
            return ControlFlow::Break(());
        }
        _ => unreachable!(),
    };

    ControlFlow::Continue(())
}

pub fn ldx_absolute_y(cpu_state: &mut CpuState, memory: &mut MemoryMapping) -> ControlFlow<()> {
    match cpu_state.current_cycle {
        1 => {
            cpu_state.effective_address = fetch_from_pc(cpu_state, memory) as u16;
        }
        2 => {
            let address_high_byte = fetch_from_pc(cpu_state, memory);

            let (address_low_byte, carry) =
                (cpu_state.effective_address as u8).overflowing_add(cpu_state.y_index);

            cpu_state.effective_address = (address_high_byte as u16) << 8 | address_low_byte as u16;
            // store information about the carry.
            // the 6502 will first read from the page-wrapped address, and then,
            // if the carry was set, will fix up the address and do a second read.
            // Most importantly, we can't use the CARRY flag, since that
            // is documented not to change, the real CPU uses the ALU output, we don't
            // simulate an ALU, so we use the IGNORED_FLAG flag to persist a single bit of information
            cpu_state.flags.set(StatusFlags::IGNORED_FLAG, carry);
        }
        3 => {
            let value = memory.load(cpu_state.effective_address);
            set_register(&mut cpu_state.x_index, value, &mut cpu_state.flags);

            if cpu_state.flags.contains(StatusFlags::IGNORED_FLAG) {
                // oops, blown through a page, fix up the effective address
                cpu_state.effective_address = cpu_state.effective_address.wrapping_add(1 << 8);
            } else {
                return ControlFlow::Break(());
            }
        }
        4 => {
            // do another read if the previous address was wrong
            let value = memory.load(cpu_state.effective_address);
            set_register(&mut cpu_state.x_index, value, &mut cpu_state.flags);
            return ControlFlow::Break(());
        }
        _ => unreachable!(),
    };

    ControlFlow::Continue(())
}
