mod addrs;
use addrs::*;

pub use addrs::OPCODE_ADDR;

use crate::{
    cpu::{executor::Executor, tests::TestMemory},
    memory::Memory,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressingMode {
    Immediate,
    Zeropage,
    ZeropageX,
    ZeropageXOverflow,
    ZeropageY,
    ZeropageYOverflow,
    Absolute,
    AbsoluteX,
    AbsoluteXOverflow,
    AbsoluteY,
    AbsoluteYOverflow,
    IndirectX,
    IndirectXOverflow,
    IndirectXPageSplit,
    IndirectY,
    IndirectYOverflow,
    IndirectYPageSplit,
}

impl AddressingMode {
    pub fn instruction_length(self) -> u16 {
        match self {
            AddressingMode::Immediate
            | AddressingMode::Zeropage
            | AddressingMode::ZeropageX
            | AddressingMode::ZeropageXOverflow
            | AddressingMode::ZeropageY
            | AddressingMode::ZeropageYOverflow
            | AddressingMode::IndirectX
            | AddressingMode::IndirectXOverflow
            | AddressingMode::IndirectXPageSplit
            | AddressingMode::IndirectY
            | AddressingMode::IndirectYOverflow
            | AddressingMode::IndirectYPageSplit => 2,

            AddressingMode::Absolute
            | AddressingMode::AbsoluteX
            | AddressingMode::AbsoluteXOverflow
            | AddressingMode::AbsoluteY
            | AddressingMode::AbsoluteYOverflow => 3,
        }
    }

    pub fn prepare(self, Executor { cpu, memory }: &mut Executor<TestMemory>) {
        match self {
            AddressingMode::Immediate => {}
            AddressingMode::Zeropage => {
                memory.store(ARG_ADDR, ZEROPAGE_ADDR);
            }
            AddressingMode::ZeropageX => {
                memory.store(ARG_ADDR, ZEROPAGE_X_BASE_ADDR);
                cpu.x = ZEROPAGE_X_OFFSET;
            }
            AddressingMode::ZeropageXOverflow => {
                memory.store(ARG_ADDR, ZEROPAGE_X_BASE_ADDR_OVERFLOW);
                cpu.x = ZEROPAGE_X_OFFSET_OVERFLOW;
            }
            AddressingMode::ZeropageY => {
                memory.store(ARG_ADDR, ZEROPAGE_X_BASE_ADDR);
                cpu.y = ZEROPAGE_X_OFFSET;
            }
            AddressingMode::ZeropageYOverflow => {
                memory.store(ARG_ADDR, ZEROPAGE_X_BASE_ADDR_OVERFLOW);
                cpu.y = ZEROPAGE_X_OFFSET_OVERFLOW;
            }
            AddressingMode::Absolute => {
                memory.store(ARG_ADDR, ABSOLUTE_ADDR.to_le_bytes()[0]);
                memory.store(ARG_ADDR + 1, ABSOLUTE_ADDR.to_le_bytes()[1]);
            }
            AddressingMode::AbsoluteX => {
                memory.store(ARG_ADDR, ABSOLUTE_X_BASE_ADDR.to_le_bytes()[0]);
                memory.store(ARG_ADDR + 1, ABSOLUTE_X_BASE_ADDR.to_le_bytes()[1]);
                cpu.x = ABSOLUTE_X_OFFSET;
            }
            AddressingMode::AbsoluteXOverflow => {
                memory.store(ARG_ADDR, ABSOLUTE_X_BASE_ADDR_OVERFLOW.to_le_bytes()[0]);
                memory.store(ARG_ADDR + 1, ABSOLUTE_X_BASE_ADDR_OVERFLOW.to_le_bytes()[1]);
                cpu.x = ABSOLUTE_X_OFFSET_OVERFLOW;
            }
            AddressingMode::AbsoluteY => {
                memory.store(ARG_ADDR, ABSOLUTE_X_BASE_ADDR.to_le_bytes()[0]);
                memory.store(ARG_ADDR + 1, ABSOLUTE_X_BASE_ADDR.to_le_bytes()[1]);
                cpu.y = ABSOLUTE_X_OFFSET;
            }
            AddressingMode::AbsoluteYOverflow => {
                memory.store(ARG_ADDR, ABSOLUTE_X_BASE_ADDR_OVERFLOW.to_le_bytes()[0]);
                memory.store(ARG_ADDR + 1, ABSOLUTE_X_BASE_ADDR_OVERFLOW.to_le_bytes()[1]);
                cpu.y = ABSOLUTE_X_OFFSET_OVERFLOW;
            }
            AddressingMode::IndirectX => {
                memory.store(ARG_ADDR, INDIRECT_X_PTR_BASE);
                cpu.x = INDIRECT_X_OFFSET;
                memory.store(
                    INDIRECT_X_FINAL_PTR as u16,
                    INDIRECT_X_ADDR.to_le_bytes()[0],
                );
                memory.store(
                    INDIRECT_X_FINAL_PTR.wrapping_add(1) as u16,
                    INDIRECT_X_ADDR.to_le_bytes()[1],
                );
            }
            AddressingMode::IndirectXOverflow => {
                memory.store(ARG_ADDR, INDIRECT_X_PTR_BASE_OVERFLOW);
                cpu.x = INDIRECT_X_OFFSET_OVERFLOW;
                memory.store(
                    INDIRECT_X_FINAL_PTR_OVERFLOW as u16,
                    INDIRECT_X_ADDR_OVERFLOW.to_le_bytes()[0],
                );
                memory.store(
                    INDIRECT_X_FINAL_PTR_OVERFLOW.wrapping_add(1) as u16,
                    INDIRECT_X_ADDR_OVERFLOW.to_le_bytes()[1],
                );
            }
            AddressingMode::IndirectXPageSplit => {
                memory.store(ARG_ADDR, INDIRECT_X_PTR_BASE_PAGE_SPLIT);
                cpu.x = INDIRECT_X_OFFSET_PAGE_SPLIT;
                memory.store(
                    INDIRECT_X_FINAL_PTR_PAGE_SPLIT as u16,
                    INDIRECT_X_ADDR_PAGE_SPLIT.to_le_bytes()[0],
                );
                memory.store(
                    INDIRECT_X_FINAL_PTR_PAGE_SPLIT.wrapping_add(1) as u16,
                    INDIRECT_X_ADDR_PAGE_SPLIT.to_le_bytes()[1],
                );
            }
            AddressingMode::IndirectY => {
                memory.store(ARG_ADDR, INDIRECT_Y_PTR);
                cpu.y = INDIRECT_Y_OFFSET;
                memory.store(INDIRECT_Y_PTR as u16, INDIRECT_Y_BASE_ADDR.to_le_bytes()[0]);
                memory.store(
                    INDIRECT_Y_PTR.wrapping_add(1) as u16,
                    INDIRECT_Y_BASE_ADDR.to_le_bytes()[1],
                );
            }
            AddressingMode::IndirectYOverflow => {
                memory.store(ARG_ADDR, INDIRECT_Y_PTR_OVERFLOW);
                cpu.y = INDIRECT_Y_OFFSET_OVERFLOW;
                memory.store(
                    INDIRECT_Y_PTR_OVERFLOW as u16,
                    INDIRECT_Y_BASE_ADDR_OVERFLOW.to_le_bytes()[0],
                );
                memory.store(
                    INDIRECT_Y_PTR_OVERFLOW.wrapping_add(1) as u16,
                    INDIRECT_Y_BASE_ADDR_OVERFLOW.to_le_bytes()[1],
                );
            }
            AddressingMode::IndirectYPageSplit => {
                memory.store(ARG_ADDR, INDIRECT_Y_PTR_PAGE_SPLIT);
                cpu.y = INDIRECT_Y_OFFSET_PAGE_SPLIT;
                memory.store(
                    INDIRECT_Y_PTR_PAGE_SPLIT as u16,
                    INDIRECT_Y_BASE_ADDR_PAGE_SPLIT.to_le_bytes()[0],
                );
                memory.store(
                    INDIRECT_Y_PTR_PAGE_SPLIT.wrapping_add(1) as u16,
                    INDIRECT_Y_BASE_ADDR_PAGE_SPLIT.to_le_bytes()[1],
                );
            }
        }
    }

    pub fn value_addr(self) -> u16 {
        match self {
            AddressingMode::Immediate => ARG_ADDR,
            AddressingMode::Zeropage => ZEROPAGE_ADDR as u16,
            AddressingMode::ZeropageX => ZEROPAGE_X_FINAL_ADDR,
            AddressingMode::ZeropageXOverflow => ZEROPAGE_X_FINAL_ADDR_OVERFLOW,
            AddressingMode::ZeropageY => ZEROPAGE_X_FINAL_ADDR,
            AddressingMode::ZeropageYOverflow => ZEROPAGE_X_FINAL_ADDR_OVERFLOW,
            AddressingMode::Absolute => ABSOLUTE_ADDR,
            AddressingMode::AbsoluteX => ABSOLUTE_X_FINAL_ADDR,
            AddressingMode::AbsoluteXOverflow => ABSOLUTE_X_FINAL_ADDR_OVERFLOW,
            AddressingMode::AbsoluteY => ABSOLUTE_X_FINAL_ADDR,
            AddressingMode::AbsoluteYOverflow => ABSOLUTE_X_FINAL_ADDR_OVERFLOW,
            AddressingMode::IndirectX => INDIRECT_X_ADDR,
            AddressingMode::IndirectXOverflow => INDIRECT_X_ADDR_OVERFLOW,
            AddressingMode::IndirectXPageSplit => INDIRECT_X_ADDR_PAGE_SPLIT,
            AddressingMode::IndirectY => INDIRECT_Y_FINAL_ADDR,
            AddressingMode::IndirectYOverflow => INDIRECT_Y_FINAL_ADDR_OVERFLOW,
            AddressingMode::IndirectYPageSplit => INDIRECT_Y_FINAL_ADDR_PAGE_SPLIT,
        }
    }
}
