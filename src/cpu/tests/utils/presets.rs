use crate::{
    cpu::{tests::consts::*, Cpu},
    memory::MemoryMapping,
};

use super::{TestOpcodePreset, OPCODE_ADDR};

/// Applies a preset
///
/// # Returns
/// the argument length
pub fn apply_preset(preset: TestOpcodePreset, cpu: &mut Cpu, memory: &mut MemoryMapping) -> u16 {
    let arg_addr: u16 = OPCODE_ADDR + 1;
    match preset {
        TestOpcodePreset::None => 0,
        TestOpcodePreset::Immediate(value) => {
            memory.store(arg_addr, value);

            1
        }
        TestOpcodePreset::ZeroPage(value) => {
            memory.store(arg_addr, ZEROPAGE_ADDR);
            memory.store(ZEROPAGE_ADDR as u16, value);

            1
        }
        TestOpcodePreset::ZeroPageX(value) => {
            memory.store(arg_addr, ZEROPAGE_X_BASE_ADDR);
            cpu.x_index = ZEROPAGE_X_OFFSET;
            memory.store(ZEROPAGE_X_FINAL_ADDR, value);

            1
        }
        TestOpcodePreset::ZeroPageXOverflow(value) => {
            memory.store(arg_addr, ZEROPAGE_X_BASE_ADDR_OVERFLOW);
            cpu.x_index = ZEROPAGE_X_OFFSET_OVERFLOW;
            memory.store(ZEROPAGE_X_FINAL_ADDR_OVERFLOW, value);

            1
        }
        TestOpcodePreset::ZeroPageY(value) => {
            memory.store(arg_addr, ZEROPAGE_X_BASE_ADDR);
            cpu.y_index = ZEROPAGE_X_OFFSET;
            memory.store(ZEROPAGE_X_FINAL_ADDR, value);

            1
        }
        TestOpcodePreset::ZeroPageYOverflow(value) => {
            memory.store(arg_addr, ZEROPAGE_X_BASE_ADDR_OVERFLOW);
            cpu.y_index = ZEROPAGE_X_OFFSET_OVERFLOW;
            memory.store(ZEROPAGE_X_FINAL_ADDR_OVERFLOW, value);

            1
        }
        TestOpcodePreset::Absolute(value) => {
            memory.store(arg_addr, ABSOLUTE_ADDR.to_le_bytes()[0]);
            memory.store(arg_addr + 1, ABSOLUTE_ADDR.to_le_bytes()[1]);
            memory.store(ABSOLUTE_ADDR, value);

            2
        }
        TestOpcodePreset::AbsoluteX(value) => {
            memory.store(arg_addr, ABSOLUTE_X_BASE_ADDR.to_le_bytes()[0]);
            memory.store(arg_addr + 1, ABSOLUTE_X_BASE_ADDR.to_le_bytes()[1]);
            cpu.x_index = ABSOLUTE_X_OFFSET;
            memory.store(ABSOLUTE_X_FINAL_ADDR, value);

            2
        }
        TestOpcodePreset::AbsoluteXOverflow(value) => {
            memory.store(arg_addr, ABSOLUTE_X_BASE_ADDR_OVERFLOW.to_le_bytes()[0]);
            memory.store(arg_addr + 1, ABSOLUTE_X_BASE_ADDR_OVERFLOW.to_le_bytes()[1]);
            cpu.x_index = ABSOLUTE_X_OFFSET_OVERFLOW;
            memory.store(ABSOLUTE_X_FINAL_ADDR_OVERFLOW, value);

            2
        }
        TestOpcodePreset::AbsoluteY(value) => {
            memory.store(arg_addr, ABSOLUTE_X_BASE_ADDR.to_le_bytes()[0]);
            memory.store(arg_addr + 1, ABSOLUTE_X_BASE_ADDR.to_le_bytes()[1]);
            cpu.y_index = ABSOLUTE_X_OFFSET;
            memory.store(ABSOLUTE_X_FINAL_ADDR, value);

            2
        }
        TestOpcodePreset::AbsoluteYOverflow(value) => {
            memory.store(arg_addr, ABSOLUTE_X_BASE_ADDR_OVERFLOW.to_le_bytes()[0]);
            memory.store(arg_addr + 1, ABSOLUTE_X_BASE_ADDR_OVERFLOW.to_le_bytes()[1]);
            cpu.y_index = ABSOLUTE_X_OFFSET_OVERFLOW;
            memory.store(ABSOLUTE_X_FINAL_ADDR_OVERFLOW, value);

            2
        }
        TestOpcodePreset::IndirectX(value) => {
            memory.store(arg_addr, INDIRECT_X_PTR_BASE);
            cpu.x_index = INDIRECT_X_OFFSET;
            memory.store(
                INDIRECT_X_FINAL_PTR as u16,
                INDIRECT_X_ADDR.to_le_bytes()[0],
            );
            memory.store(
                INDIRECT_X_FINAL_PTR.wrapping_add(1) as u16,
                INDIRECT_X_ADDR.to_le_bytes()[1],
            );
            memory.store(INDIRECT_X_ADDR, value);

            1
        }
        TestOpcodePreset::IndirectXOverflow(value) => {
            memory.store(arg_addr, INDIRECT_X_PTR_BASE_OVERFLOW);
            cpu.x_index = INDIRECT_X_OFFSET_OVERFLOW;
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
        TestOpcodePreset::IndirectXPageSplit(value) => {
            memory.store(arg_addr, INDIRECT_X_PTR_BASE_PAGE_SPLIT);
            cpu.x_index = INDIRECT_X_OFFSET_PAGE_SPLIT;
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
        TestOpcodePreset::IndirectY(value) => {
            memory.store(arg_addr, INDIRECT_Y_PTR);
            cpu.y_index = INDIRECT_Y_OFFSET;
            memory.store(INDIRECT_Y_PTR as u16, INDIRECT_Y_BASE_ADDR.to_le_bytes()[0]);
            memory.store(
                INDIRECT_Y_PTR.wrapping_add(1) as u16,
                INDIRECT_Y_BASE_ADDR.to_le_bytes()[1],
            );
            memory.store(INDIRECT_Y_FINAL_ADDR, value);

            1
        }
        TestOpcodePreset::IndirectYOverflow(value) => {
            memory.store(arg_addr, INDIRECT_Y_PTR_OVERFLOW);
            cpu.y_index = INDIRECT_Y_OFFSET_OVERFLOW;
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
        TestOpcodePreset::IndirectYPageSplit(value) => {
            memory.store(arg_addr, INDIRECT_Y_PTR_PAGE_SPLIT);
            cpu.y_index = INDIRECT_Y_OFFSET_PAGE_SPLIT;
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
