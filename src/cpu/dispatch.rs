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
    // ADC
    AdcImmediate = 0x69,
    AdcZeroPage = 0x65,
    AdcZeroPageX = 0x75,
    AdcAbsolute = 0x6D,
    AdcAbsoluteX = 0x7D,
    AdcAbsoluteY = 0x79,
    AdcIndirectX = 0x61,
    AdcIndirectY = 0x71,

    // LDA
    LdaImmediate = 0xA9,
    LdaZeroPage = 0xA5,
    LdaZeroPageX = 0xB5,
    LdaAbsolute = 0xAD,
    LdaAbsoluteX = 0xBD,
    LdaAbsoluteY = 0xB9,
    LdaIndirectX = 0xA1,
    LdaIndirectY = 0xB1,

    // LDX
    LdxImmediate = 0xA2,
    LdxZeroPage = 0xA6,
    LdxZeroPageY = 0xB6,
    LdxAbsolute = 0xAE,
    LdxAbsoluteY = 0xBE,

    // LDY
    LdyImmediate = 0xA0,
    LdyZeroPage = 0xA4,
    LdyZeroPageX = 0xB4,
    LdyAbsolute = 0xAC,
    LdyAbsoluteX = 0xBC,

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
        // ADC
        OpCode::AdcImmediate => adc_immediate(cpu, memory),
        OpCode::AdcZeroPage => adc_zeropage(cpu, memory),
        OpCode::AdcZeroPageX => adc_zeropage_x(cpu, memory),
        OpCode::AdcAbsolute => adc_absolute(cpu, memory),
        OpCode::AdcAbsoluteX => adc_absolute_x(cpu, memory),
        OpCode::AdcAbsoluteY => adc_absolute_y(cpu, memory),
        OpCode::AdcIndirectX => adc_indirect_x(cpu, memory),
        OpCode::AdcIndirectY => adc_indirect_y(cpu, memory),

        // LDA
        OpCode::LdaImmediate => lda_immediate(cpu, memory),
        OpCode::LdaZeroPage => lda_zeropage(cpu, memory),
        OpCode::LdaZeroPageX => lda_zeropage_x(cpu, memory),
        OpCode::LdaAbsolute => lda_absolute(cpu, memory),
        OpCode::LdaAbsoluteX => lda_absolute_x(cpu, memory),
        OpCode::LdaAbsoluteY => lda_absolute_y(cpu, memory),
        OpCode::LdaIndirectX => lda_indirect_x(cpu, memory),
        OpCode::LdaIndirectY => lda_indirect_y(cpu, memory),

        // LDX
        OpCode::LdxImmediate => ldx_immediate(cpu, memory),
        OpCode::LdxZeroPage => ldx_zeropage(cpu, memory),
        OpCode::LdxZeroPageY => ldx_zeropage_y(cpu, memory),
        OpCode::LdxAbsolute => ldx_absolute(cpu, memory),
        OpCode::LdxAbsoluteY => ldx_absolute_y(cpu, memory),

        // LDY
        OpCode::LdyImmediate => ldy_immediate(cpu, memory),
        OpCode::LdyZeroPage => ldy_zeropage(cpu, memory),
        OpCode::LdyZeroPageX => ldy_zeropage_x(cpu, memory),
        OpCode::LdyAbsolute => ldy_absolute(cpu, memory),
        OpCode::LdyAbsoluteX => ldy_absolute_x(cpu, memory),

        _ => unimplemented!(),
    }
}
