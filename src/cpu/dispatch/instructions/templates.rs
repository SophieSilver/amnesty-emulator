//! Template functions for implementing instructions
//!
//! Most instructions have commonalities, such as memory access patterns
//!
//! This module defines template instructions that implement the common functionality between instructions
//! by taking a closure as an argument that performs the actual work after the addressing is done

use std::ops::ControlFlow;

use crate::{
    cpu::{Cpu, InternalFlags},
    memory::MemoryMapping,
};

use super::{fetch_from_pc, get_x_index, get_y_index};

pub mod read {
    use super::*;

    pub fn immediate<F: FnOnce(&mut Cpu, u8)>(
        cpu: &mut Cpu,
        memory: &mut MemoryMapping,
        f: F,
    ) -> ControlFlow<()> //
    {
        match cpu.current_cycle {
            1 => {
                let value = fetch_from_pc(cpu, memory);
                f(cpu, value);
                ControlFlow::Break(())
            }
            _ => unreachable!(),
        }
    }

    pub fn zeropage<F: FnOnce(&mut Cpu, u8)>(
        cpu: &mut Cpu,
        memory: &mut MemoryMapping,
        f: F,
    ) -> ControlFlow<()> //
    {
        match cpu.current_cycle {
            1 => {
                cpu.effective_address = fetch_from_pc(cpu, memory) as u16;
            }
            2 => {
                let value = memory.load(cpu.effective_address);
                f(cpu, value);

                return ControlFlow::Break(());
            }
            _ => unreachable!(),
        };

        ControlFlow::Continue(())
    }

    pub fn zeropage_indexed<F, I>(
        cpu: &mut Cpu,
        memory: &mut MemoryMapping,
        get_index: I,
        f: F,
    ) -> ControlFlow<()>
    where
        F: FnOnce(&mut Cpu, u8),
        I: FnOnce(&Cpu) -> u8,
    {
        match cpu.current_cycle {
            1 => {
                cpu.effective_address = fetch_from_pc(cpu, memory) as u16;
            }
            2 => {
                // dummy read coz every cycle is a read or a write
                let _ = memory.load(cpu.effective_address);
                cpu.effective_address = cpu
                    .effective_address
                    .checked_add(get_index(cpu) as u16)
                    .expect("everything was cast from a u8, will never overflow");

                // upper byte is always 0, page overflow is ignored
                cpu.effective_address &= 0xFF;
            }
            3 => {
                let value = memory.load(cpu.effective_address);
                f(cpu, value);

                return ControlFlow::Break(());
            }
            _ => unreachable!(),
        };

        ControlFlow::Continue(())
    }

    pub fn zeropage_x<F: FnOnce(&mut Cpu, u8)>(
        cpu: &mut Cpu,
        memory: &mut MemoryMapping,
        f: F,
    ) -> ControlFlow<()> //
    {
        zeropage_indexed(cpu, memory, get_x_index, f)
    }

    pub fn zeropage_y<F: FnOnce(&mut Cpu, u8)>(
        cpu: &mut Cpu,
        memory: &mut MemoryMapping,
        f: F,
    ) -> ControlFlow<()> //
    {
        zeropage_indexed(cpu, memory, get_y_index, f)
    }

    pub fn absolute<F: FnOnce(&mut Cpu, u8)>(
        cpu: &mut Cpu,
        memory: &mut MemoryMapping,
        f: F,
    ) -> ControlFlow<()> //
    {
        match cpu.current_cycle {
            1 => {
                cpu.effective_address = fetch_from_pc(cpu, memory) as u16;
            }
            2 => {
                cpu.effective_address |= (fetch_from_pc(cpu, memory) as u16) << 8;
            }
            3 => {
                let value = memory.load(cpu.effective_address);
                f(cpu, value);
                return ControlFlow::Break(());
            }
            _ => unreachable!(),
        };

        ControlFlow::Continue(())
    }

    pub fn absolute_indexed<F, I>(
        cpu: &mut Cpu,
        memory: &mut MemoryMapping,
        get_index: I,
        f: F,
    ) -> ControlFlow<()>
    where
        F: FnOnce(&mut Cpu, u8),
        I: FnOnce(&Cpu) -> u8,
    {
        match cpu.current_cycle {
            1 => {
                cpu.effective_address = fetch_from_pc(cpu, memory) as u16;
            }
            2 => {
                let address_high_byte = fetch_from_pc(cpu, memory);
                let (address_low_byte, carry) =
                    (cpu.effective_address as u8).overflowing_add(get_index(cpu));

                cpu.effective_address = (address_high_byte as u16) << 8 | address_low_byte as u16;
                // Using an internal flag, because we need to persist this state accross the clock cycle, but
                // we are not allowed to modify the CARRY flag
                cpu.internal_flags
                    .set(InternalFlags::EFFECTIVE_ADDR_CARRY, carry);
            }
            3 => {
                let value = memory.load(cpu.effective_address);

                if cpu
                    .internal_flags
                    .contains(InternalFlags::EFFECTIVE_ADDR_CARRY)
                {
                    // oops, blown through a page, fix up the effective address
                    cpu.effective_address = cpu.effective_address.wrapping_add(1 << 8);
                } else {
                    f(cpu, value);
                    return ControlFlow::Break(());
                }
            }
            4 => {
                // do another read if the previous address was wrong
                let value = memory.load(cpu.effective_address);
                f(cpu, value);
                return ControlFlow::Break(());
            }
            _ => unreachable!(),
        };

        ControlFlow::Continue(())
    }

    pub fn absolute_x<F: FnOnce(&mut Cpu, u8)>(
        cpu: &mut Cpu,
        memory: &mut MemoryMapping,
        f: F,
    ) -> ControlFlow<()> //
    {
        absolute_indexed(cpu, memory, get_x_index, f)
    }

    pub fn absolute_y<F: FnOnce(&mut Cpu, u8)>(
        cpu: &mut Cpu,
        memory: &mut MemoryMapping,
        f: F,
    ) -> ControlFlow<()> //
    {
        absolute_indexed(cpu, memory, get_y_index, f)
    }

    pub fn indirect_x<F: FnOnce(&mut Cpu, u8)>(
        cpu: &mut Cpu,
        memory: &mut MemoryMapping,
        f: F,
    ) -> ControlFlow<()> //
    {
        match cpu.current_cycle {
            1 => {
                cpu.pointer_address = fetch_from_pc(cpu, memory);
            }
            2 => {
                // dummy read :3
                let _ = memory.load(cpu.pointer_address as u16);
                cpu.pointer_address = cpu.pointer_address.wrapping_add(cpu.x_index);
            }
            3 => {
                cpu.effective_address = memory.load(cpu.pointer_address as u16) as u16;
            }
            4 => {
                cpu.effective_address |=
                    (memory.load(cpu.pointer_address.wrapping_add(1) as u16) as u16) << 8;
            }
            5 => {
                let value = memory.load(cpu.effective_address);
                f(cpu, value);
                return ControlFlow::Break(());
            }

            _ => unreachable!(),
        }

        ControlFlow::Continue(())
    }

    pub fn indirect_y<F: FnOnce(&mut Cpu, u8)>(
        cpu: &mut Cpu,
        memory: &mut MemoryMapping,
        f: F,
    ) -> ControlFlow<()> //
    {
        match cpu.current_cycle {
            1 => {
                cpu.pointer_address = fetch_from_pc(cpu, memory);
            }
            2 => {
                cpu.effective_address = memory.load(cpu.pointer_address as u16) as u16;
            }
            3 => {
                let address_high_byte = memory.load(cpu.pointer_address.wrapping_add(1) as u16);
                let (address_low_byte, carry) =
                    (cpu.effective_address as u8).overflowing_add(cpu.y_index);

                cpu.effective_address = (address_high_byte as u16) << 8 | address_low_byte as u16;
                cpu.internal_flags
                    .set(InternalFlags::EFFECTIVE_ADDR_CARRY, carry);
            }
            4 => {
                let value = memory.load(cpu.effective_address);

                if cpu
                    .internal_flags
                    .contains(InternalFlags::EFFECTIVE_ADDR_CARRY)
                {
                    cpu.effective_address = cpu.effective_address.wrapping_add(1 << 8);
                } else {
                    f(cpu, value);
                    return ControlFlow::Break(());
                }
            }
            5 => {
                let value = memory.load(cpu.effective_address);
                f(cpu, value);
                return ControlFlow::Break(());
            }
            _ => unreachable!(),
        }

        ControlFlow::Continue(())
    }
}
