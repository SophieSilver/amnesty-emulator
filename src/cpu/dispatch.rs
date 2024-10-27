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

    // AND
    AndImmediate = 0x29,
    AndZeroPage = 0x25,
    AndZeroPageX = 0x35,
    AndAbsolute = 0x2D,
    AndAbsoluteX = 0x3D,
    AndAbsoluteY = 0x39,
    AndIndirectX = 0x21,
    AndIndirectY = 0x31,

    // BIT
    BitZeroPage = 0x24,
    BitAbsolute = 0x2C,

    // CMP
    CmpImmediate = 0xC9,
    CmpZeroPage = 0xC5,
    CmpZeroPageX = 0xD5,
    CmpAbsolute = 0xCD,
    CmpAbsoluteX = 0xDD,
    CmpAbsoluteY = 0xD9,
    CmpIndirectX = 0xC1,
    CmpIndirectY = 0xD1,

    // EOR
    EorImmediate = 0x49,
    EorZeroPage = 0x45,
    EorZeroPageX = 0x55,
    EorAbsolute = 0x4D,
    EorAbsoluteX = 0x5D,
    EorIndirectX = 0x41,
    EorIndirectY = 0x51,

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

        //AND
        OpCode::AndImmediate => and_immediate(cpu, memory),
        OpCode::AndZeroPage => and_zeropage(cpu, memory),
        OpCode::AndZeroPageX => and_zeropage_x(cpu, memory),
        OpCode::AndAbsolute => and_absolute(cpu, memory),
        OpCode::AndAbsoluteX => and_absolute_x(cpu, memory),
        OpCode::AndAbsoluteY => and_absolute_y(cpu, memory),
        OpCode::AndIndirectX => and_indirect_x(cpu, memory),
        OpCode::AndIndirectY => and_indirect_y(cpu, memory),

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
