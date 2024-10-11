//! Template functions for implementing instructions
//!
//! Most instructions have commonalities, such as memory access patterns
//!
//! This module defines template instructions that implement the common functionality between instructions
//! by taking a closure as an argument that performs the actual work after the addressing is done

use std::ops::ControlFlow;

use crate::{
    cpu::{CpuState, InternalFlags},
    memory::MemoryMapping,
};

use super::fetch_from_pc;

pub fn read_immediate<F: FnOnce(&mut CpuState, u8)>(
    cpu_state: &mut CpuState,
    memory: &mut MemoryMapping,
    f: F,
) -> ControlFlow<()> //
{
    match cpu_state.current_cycle {
        1 => {
            let value = fetch_from_pc(cpu_state, memory);
            f(cpu_state, value);
            ControlFlow::Break(())
        }
        _ => unreachable!(),
    }
}

pub fn read_zeropage<F: FnOnce(&mut CpuState, u8)>(
    cpu_state: &mut CpuState,
    memory: &mut MemoryMapping,
    f: F,
) -> ControlFlow<()> //
{
    match cpu_state.current_cycle {
        1 => {
            cpu_state.effective_address = fetch_from_pc(cpu_state, memory) as u16;
        }
        2 => {
            let value = memory.load(cpu_state.effective_address);
            f(cpu_state, value);

            return ControlFlow::Break(());
        }
        _ => unreachable!(),
    };

    ControlFlow::Continue(())
}

pub fn read_zeropage_indexed<F, I>(
    cpu_state: &mut CpuState,
    memory: &mut MemoryMapping,
    get_index: I,
    f: F,
) -> ControlFlow<()>
where
    F: FnOnce(&mut CpuState, u8),
    I: FnOnce(&CpuState) -> u8,
{
    match cpu_state.current_cycle {
        1 => {
            cpu_state.effective_address = fetch_from_pc(cpu_state, memory) as u16;
        }
        2 => {
            // dummy read coz every cycle is a read or a write
            let _ = memory.load(cpu_state.effective_address);
            cpu_state.effective_address = cpu_state
                .effective_address
                .checked_add(get_index(cpu_state) as u16)
                .expect("everything was cast from a u8, will never overflow");

            // upper byte is always 0, page overflow is ignored
            cpu_state.effective_address &= 0xFF;
        }
        3 => {
            let value = memory.load(cpu_state.effective_address);
            f(cpu_state, value);

            return ControlFlow::Break(());
        }
        _ => unreachable!(),
    };

    ControlFlow::Continue(())
}

pub fn read_absolute<F: FnOnce(&mut CpuState, u8)>(
    cpu_state: &mut CpuState,
    memory: &mut MemoryMapping,
    f: F,
) -> ControlFlow<()> //
{
    match cpu_state.current_cycle {
        1 => {
            cpu_state.effective_address = fetch_from_pc(cpu_state, memory) as u16;
        }
        2 => {
            cpu_state.effective_address |= (fetch_from_pc(cpu_state, memory) as u16) << 8;
        }
        3 => {
            let value = memory.load(cpu_state.effective_address);
            f(cpu_state, value);
            return ControlFlow::Break(());
        }
        _ => unreachable!(),
    };

    ControlFlow::Continue(())
}

pub fn read_absolute_indexed<F, I>(
    cpu_state: &mut CpuState,
    memory: &mut MemoryMapping,
    get_index: I,
    f: F,
) -> ControlFlow<()>
where
    F: FnOnce(&mut CpuState, u8),
    I: FnOnce(&CpuState) -> u8,
{
    match cpu_state.current_cycle {
        1 => {
            cpu_state.effective_address = fetch_from_pc(cpu_state, memory) as u16;
        }
        2 => {
            let address_high_byte = fetch_from_pc(cpu_state, memory);
            let (address_low_byte, carry) =
                (cpu_state.effective_address as u8).overflowing_add(get_index(cpu_state));

            cpu_state.effective_address = (address_high_byte as u16) << 8 | address_low_byte as u16;
            // Using an internal flag, because we need to persist this state accross the clock cycle, but
            // we are not allowed to modify the CARRY flag
            cpu_state
                .internal_flags
                .set(InternalFlags::EFFECTIVE_ADDR_CARRY, carry);
        }
        3 => {
            let value = memory.load(cpu_state.effective_address);

            if cpu_state
                .internal_flags
                .contains(InternalFlags::EFFECTIVE_ADDR_CARRY)
            {
                // oops, blown through a page, fix up the effective address
                cpu_state.effective_address = cpu_state.effective_address.wrapping_add(1 << 8);
            } else {
                f(cpu_state, value);
                return ControlFlow::Break(());
            }
        }
        4 => {
            // do another read if the previous address was wrong
            let value = memory.load(cpu_state.effective_address);
            f(cpu_state, value);
            return ControlFlow::Break(());
        }
        _ => unreachable!(),
    };

    ControlFlow::Continue(())
}

pub fn read_indirect_x<F: FnOnce(&mut CpuState, u8)>(
    cpu_state: &mut CpuState,
    memory: &mut MemoryMapping,
    f: F,
) -> ControlFlow<()> //
{
    match cpu_state.current_cycle {
        1 => {
            cpu_state.pointer_address = fetch_from_pc(cpu_state, memory);
        }
        2 => {
            // dummy read :3
            let _ = memory.load(cpu_state.pointer_address as u16);
            cpu_state.pointer_address = cpu_state.pointer_address.wrapping_add(cpu_state.x_index);
        }
        3 => {
            cpu_state.effective_address = memory.load(cpu_state.pointer_address as u16) as u16;
        }
        4 => {
            cpu_state.effective_address |=
                (memory.load(cpu_state.pointer_address.wrapping_add(1) as u16) as u16) << 8;
        }
        5 => {
            let value = memory.load(cpu_state.effective_address);
            f(cpu_state, value);
            return ControlFlow::Break(());
        }

        _ => unreachable!(),
    }

    ControlFlow::Continue(())
}

pub fn read_indirect_y<F: FnOnce(&mut CpuState, u8)>(
    cpu_state: &mut CpuState,
    memory: &mut MemoryMapping,
    f: F,
) -> ControlFlow<()> //
{
    match cpu_state.current_cycle {
        1 => {
            cpu_state.pointer_address = fetch_from_pc(cpu_state, memory);
        }
        2 => {
            cpu_state.effective_address = memory.load(cpu_state.pointer_address as u16) as u16;
        }
        3 => {
            let address_high_byte = memory.load(cpu_state.pointer_address.wrapping_add(1) as u16);
            let (address_low_byte, carry) =
                (cpu_state.effective_address as u8).overflowing_add(cpu_state.y_index);

            cpu_state.effective_address = (address_high_byte as u16) << 8 | address_low_byte as u16;
            cpu_state
                .internal_flags
                .set(InternalFlags::EFFECTIVE_ADDR_CARRY, carry);
        }
        4 => {
            let value = memory.load(cpu_state.effective_address);

            if cpu_state
                .internal_flags
                .contains(InternalFlags::EFFECTIVE_ADDR_CARRY)
            {
                cpu_state.effective_address = cpu_state.effective_address.wrapping_add(1 << 8);
            } else {
                f(cpu_state, value);
                return ControlFlow::Break(());
            }
        }
        5 => {
            let value = memory.load(cpu_state.effective_address);
            f(cpu_state, value);
            return ControlFlow::Break(());
        }
        _ => unreachable!(),
    }

    ControlFlow::Continue(())
}
