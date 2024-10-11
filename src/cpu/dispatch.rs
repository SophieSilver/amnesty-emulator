use super::CpuState;
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

    #[default]
    Unimplemented,
}

pub fn dispatch_current_opcode(
    cpu_state: &mut CpuState,
    memory: &mut MemoryMapping,
) -> ControlFlow<()> //
{
    // First cycle is always fetching the opcode
    if cpu_state.current_cycle == 0 {
        cpu_state.current_opcode = OpCode::from(fetch_from_pc(cpu_state, memory));
        return ControlFlow::Continue(());
    }
    match cpu_state.current_opcode {
        OpCode::LdaImmediate => lda_immediate(cpu_state, memory),
        OpCode::LdaZeroPage => lda_zeropage(cpu_state, memory),
        OpCode::LdaZeroPageX => lda_zeropage_x(cpu_state, memory),
        OpCode::LdaAbs => lda_absolute(cpu_state, memory),
        OpCode::LdaAbsX => lda_absolute_x(cpu_state, memory),
        OpCode::LdaAbsY => lda_absolute_y(cpu_state, memory),
        OpCode::LdaIndirectX => lda_indirect_x(cpu_state, memory),
        OpCode::LdaIndirectY => lda_indirect_y(cpu_state, memory),

        OpCode::LdxImmediate => ldx_immediate(cpu_state, memory),
        OpCode::LdxZeroPage => ldx_zeropage(cpu_state, memory),
        OpCode::LdxZeroPageY => ldx_zeropage_y(cpu_state, memory),
        OpCode::LdxAbs => ldx_absolute(cpu_state, memory),
        OpCode::LdxAbsY => ldx_absolute_y(cpu_state, memory),

        _ => unimplemented!(),
    }
}
