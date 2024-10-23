use super::Cpu;
use crate::memory::MemoryMapping;
use helpers::fetch_from_pc;
use num_enum::{FromPrimitive, IntoPrimitive};
use std::ops::ControlFlow;

pub(in crate::cpu) mod instructions;
use instructions::*;

#[derive(Debug, Clone, Copy, IntoPrimitive, FromPrimitive, Default)]
#[non_exhaustive]
#[repr(u8)]
pub enum OpCode {
    // LDA
    LdaImmediate = 0xA9,
    LdaZeroPage = 0xA5,
    LdaZeroPageX = 0xB5,
    LdaAbs = 0xAD,
    LdaAbsX = 0xBD,
    LdaAbsY = 0xB9,
    LdaIndirectX = 0xA1,
    LdaIndirectY = 0xB1,

    // LDX
    LdxImmediate = 0xA2,
    LdxZeroPage = 0xA6,
    LdxZeroPageY = 0xB6,
    LdxAbs = 0xAE,
    LdxAbsY = 0xBE,

    // LDY
    LdyImmediate = 0xA0,
    LdyZeroPage = 0xA4,
    LdyZeroPageX = 0xB4,
    LdyAbs = 0xAC,
    LdyAbsX = 0xBC,

    #[default]
    Unimplemented = 0x0,
}

pub fn dispatch_current_opcode(cpu: &mut Cpu, memory: &mut MemoryMapping) -> ControlFlow<()> {
    // First cycle is always fetching the opcode
    if cpu.current_cycle == 0 {
        cpu.current_opcode = OpCode::from(fetch_from_pc(cpu, memory));
        return ControlFlow::Continue(());
    }
    match cpu.current_opcode {
        // LDA
        OpCode::LdaImmediate => lda_immediate(cpu, memory),
        OpCode::LdaZeroPage => lda_zeropage(cpu, memory),
        OpCode::LdaZeroPageX => lda_zeropage_x(cpu, memory),
        OpCode::LdaAbs => lda_absolute(cpu, memory),
        OpCode::LdaAbsX => lda_absolute_x(cpu, memory),
        OpCode::LdaAbsY => lda_absolute_y(cpu, memory),
        OpCode::LdaIndirectX => lda_indirect_x(cpu, memory),
        OpCode::LdaIndirectY => lda_indirect_y(cpu, memory),

        // LDX
        OpCode::LdxImmediate => ldx_immediate(cpu, memory),
        OpCode::LdxZeroPage => ldx_zeropage(cpu, memory),
        OpCode::LdxZeroPageY => ldx_zeropage_y(cpu, memory),
        OpCode::LdxAbs => ldx_absolute(cpu, memory),
        OpCode::LdxAbsY => ldx_absolute_y(cpu, memory),

        // LDY
        OpCode::LdyImmediate => ldy_immediate(cpu, memory),
        OpCode::LdyZeroPage => ldy_zeropage(cpu, memory),
        OpCode::LdyZeroPageX => ldy_zeropage_x(cpu, memory),
        OpCode::LdyAbs => ldy_absolute(cpu, memory),
        OpCode::LdyAbsX => ldy_absolute_x(cpu, memory),

        _ => unimplemented!(),
    }
}
