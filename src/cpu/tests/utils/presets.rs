use crate::{
    cpu::{
        tests::{addrs::*, TestMemory},
        Cpu,
    },
    memory::Memory,
};

use super::{Preset, OPCODE_ADDR};

/// Applies a preset
///
/// # Returns
/// the argument length
pub fn apply_preset(preset: Preset, cpu: &mut Cpu, memory: &mut TestMemory) -> u16 {
    let arg_addr: u16 = OPCODE_ADDR + 1;
    match preset {
        Preset::Immediate(value) => {
            memory.store(arg_addr, value);

            1
        }
        Preset::ZeroPage(value) => {
            memory.store(arg_addr, ZEROPAGE_ADDR);
            memory.store(ZEROPAGE_ADDR as u16, value);

            1
        }
        Preset::ZeroPageX(value) => {
            memory.store(arg_addr, ZEROPAGE_X_BASE_ADDR);
            cpu.x = ZEROPAGE_X_OFFSET;
            memory.store(ZEROPAGE_X_FINAL_ADDR, value);

            1
        }
        Preset::ZeroPageXOverflow(value) => {
            memory.store(arg_addr, ZEROPAGE_X_BASE_ADDR_OVERFLOW);
            cpu.x = ZEROPAGE_X_OFFSET_OVERFLOW;
            memory.store(ZEROPAGE_X_FINAL_ADDR_OVERFLOW, value);

            1
        }
        Preset::ZeroPageY(value) => {
            memory.store(arg_addr, ZEROPAGE_X_BASE_ADDR);
            cpu.y = ZEROPAGE_X_OFFSET;
            memory.store(ZEROPAGE_X_FINAL_ADDR, value);

            1
        }
        Preset::ZeroPageYOverflow(value) => {
            memory.store(arg_addr, ZEROPAGE_X_BASE_ADDR_OVERFLOW);
            cpu.y = ZEROPAGE_X_OFFSET_OVERFLOW;
            memory.store(ZEROPAGE_X_FINAL_ADDR_OVERFLOW, value);

            1
        }
        Preset::Absolute(value) => {
            memory.store(arg_addr, ABSOLUTE_ADDR.to_le_bytes()[0]);
            memory.store(arg_addr + 1, ABSOLUTE_ADDR.to_le_bytes()[1]);
            memory.store(ABSOLUTE_ADDR, value);

            2
        }
        Preset::AbsoluteX(value) => {
            memory.store(arg_addr, ABSOLUTE_X_BASE_ADDR.to_le_bytes()[0]);
            memory.store(arg_addr + 1, ABSOLUTE_X_BASE_ADDR.to_le_bytes()[1]);
            cpu.x = ABSOLUTE_X_OFFSET;
            memory.store(ABSOLUTE_X_FINAL_ADDR, value);

            2
        }
        Preset::AbsoluteXOverflow(value) => {
            memory.store(arg_addr, ABSOLUTE_X_BASE_ADDR_OVERFLOW.to_le_bytes()[0]);
            memory.store(arg_addr + 1, ABSOLUTE_X_BASE_ADDR_OVERFLOW.to_le_bytes()[1]);
            cpu.x = ABSOLUTE_X_OFFSET_OVERFLOW;
            memory.store(ABSOLUTE_X_FINAL_ADDR_OVERFLOW, value);

            2
        }
        Preset::AbsoluteY(value) => {
            memory.store(arg_addr, ABSOLUTE_X_BASE_ADDR.to_le_bytes()[0]);
            memory.store(arg_addr + 1, ABSOLUTE_X_BASE_ADDR.to_le_bytes()[1]);
            cpu.y = ABSOLUTE_X_OFFSET;
            memory.store(ABSOLUTE_X_FINAL_ADDR, value);

            2
        }
        Preset::AbsoluteYOverflow(value) => {
            memory.store(arg_addr, ABSOLUTE_X_BASE_ADDR_OVERFLOW.to_le_bytes()[0]);
            memory.store(arg_addr + 1, ABSOLUTE_X_BASE_ADDR_OVERFLOW.to_le_bytes()[1]);
            cpu.y = ABSOLUTE_X_OFFSET_OVERFLOW;
            memory.store(ABSOLUTE_X_FINAL_ADDR_OVERFLOW, value);

            2
        }
        Preset::IndirectX(value) => {
            memory.store(arg_addr, INDIRECT_X_PTR_BASE);
            cpu.x = INDIRECT_X_OFFSET;
            memory.store(
                INDIRECT_X_FINAL_PTR as u16,
                INDIRECT_X_FINAL_ADDR.to_le_bytes()[0],
            );
            memory.store(
                INDIRECT_X_FINAL_PTR.wrapping_add(1) as u16,
                INDIRECT_X_FINAL_ADDR.to_le_bytes()[1],
            );
            memory.store(INDIRECT_X_FINAL_ADDR, value);

            1
        }
        Preset::IndirectXOverflow(value) => {
            memory.store(arg_addr, INDIRECT_X_PTR_BASE_OVERFLOW);
            cpu.x = INDIRECT_X_OFFSET_OVERFLOW;
            memory.store(
                INDIRECT_X_FINAL_PTR_OVERFLOW as u16,
                INDIRECT_X_ADDR_OVERFLOW.to_le_bytes()[0],
            );
            memory.store(
                INDIRECT_X_FINAL_PTR_OVERFLOW.wrapping_add(1) as u16,
                INDIRECT_X_ADDR_OVERFLOW.to_le_bytes()[1],
            );
            memory.store(INDIRECT_X_ADDR_OVERFLOW, value);

            1
        }
        Preset::IndirectXPageSplit(value) => {
            memory.store(arg_addr, INDIRECT_X_PTR_BASE_PAGE_SPLIT);
            cpu.x = INDIRECT_X_OFFSET_PAGE_SPLIT;
            memory.store(
                INDIRECT_X_FINAL_PTR_PAGE_SPLIT as u16,
                INDIRECT_X_ADDR_PAGE_SPLIT.to_le_bytes()[0],
            );
            memory.store(
                INDIRECT_X_FINAL_PTR_PAGE_SPLIT.wrapping_add(1) as u16,
                INDIRECT_X_ADDR_PAGE_SPLIT.to_le_bytes()[1],
            );
            memory.store(INDIRECT_X_ADDR_PAGE_SPLIT, value);

            1
        }
        Preset::IndirectY(value) => {
            memory.store(arg_addr, INDIRECT_Y_PTR);
            cpu.y = INDIRECT_Y_OFFSET;
            memory.store(INDIRECT_Y_PTR as u16, INDIRECT_Y_BASE_ADDR.to_le_bytes()[0]);
            memory.store(
                INDIRECT_Y_PTR.wrapping_add(1) as u16,
                INDIRECT_Y_BASE_ADDR.to_le_bytes()[1],
            );
            memory.store(INDIRECT_Y_FINAL_ADDR, value);

            1
        }
        Preset::IndirectYOverflow(value) => {
            memory.store(arg_addr, INDIRECT_Y_PTR_OVERFLOW);
            cpu.y = INDIRECT_Y_OFFSET_OVERFLOW;
            memory.store(
                INDIRECT_Y_PTR_OVERFLOW as u16,
                INDIRECT_Y_BASE_ADDR_OVERFLOW.to_le_bytes()[0],
            );
            memory.store(
                INDIRECT_Y_PTR_OVERFLOW.wrapping_add(1) as u16,
                INDIRECT_Y_BASE_ADDR_OVERFLOW.to_le_bytes()[1],
            );
            memory.store(INDIRECT_Y_FINAL_ADDR_OVERFLOW, value);

            1
        }
        Preset::IndirectYPageSplit(value) => {
            memory.store(arg_addr, INDIRECT_Y_PTR_PAGE_SPLIT);
            cpu.y = INDIRECT_Y_OFFSET_PAGE_SPLIT;
            memory.store(
                INDIRECT_Y_PTR_PAGE_SPLIT as u16,
                INDIRECT_Y_BASE_ADDR_PAGE_SPLIT.to_le_bytes()[0],
            );
            memory.store(
                INDIRECT_Y_PTR_PAGE_SPLIT.wrapping_add(1) as u16,
                INDIRECT_Y_BASE_ADDR_PAGE_SPLIT.to_le_bytes()[1],
            );
            memory.store(INDIRECT_Y_FINAL_ADDR_PAGE_SPLIT, value);

            1
        }
    }
}
