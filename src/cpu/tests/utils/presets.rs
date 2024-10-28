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
        TestOpcodePreset::AbsoluteX(_) => todo!(),
        TestOpcodePreset::AbsoluteXOverflow(_) => todo!(),
        TestOpcodePreset::AbsoluteY(_) => todo!(),
        TestOpcodePreset::AbsoluteYOverflow(_) => todo!(),
        TestOpcodePreset::IndirectX(_) => todo!(),
        TestOpcodePreset::IndirectXOverflow(_) => todo!(),
        TestOpcodePreset::IndirectXPageSplit(_) => todo!(),
        TestOpcodePreset::IndirectY(_) => todo!(),
        TestOpcodePreset::IndirectYOverflow(_) => todo!(),
        TestOpcodePreset::IndirectYPageSplit(_) => todo!(),
    }
}
