use super::executor::Executor;
use crate::{cpu::instructions::addressing_modes::read::*, memory::Memory};

mod addressing_modes;

mod adc;
mod and;
mod bit;
mod cmp;
mod lda;
mod ldx;
mod ldy;
mod sbc;

pub use adc::*;
pub use and::*;
pub use bit::*;
pub use cmp::*;
pub use lda::*;
pub use ldx::*;
pub use ldy::*;
pub use sbc::*;

pub mod opcode;
use opcode::OpCode;

pub fn execute_opcode<M: Memory>(executor: &mut Executor<M>, opcode: OpCode) {
    match opcode {
        // ADC
        OpCode::AdcImmediate => Adc::immediate(executor),
        OpCode::AdcZeropage => Adc::zeropage(executor),
        OpCode::AdcZeropageX => Adc::zeropage_x(executor),
        OpCode::AdcAbsolute => Adc::absolute(executor),
        OpCode::AdcAbsoluteX => Adc::absolute_x(executor),
        OpCode::AdcAbsoluteY => Adc::absolute_y(executor),
        OpCode::AdcIndirectX => Adc::indirect_x(executor),
        OpCode::AdcIndirectY => Adc::indirect_y(executor),

        // AND
        OpCode::AndImmediate => And::immediate(executor),
        OpCode::AndZeropage => And::zeropage(executor),
        OpCode::AndZeropageX => And::zeropage_x(executor),
        OpCode::AndAbsolute => And::absolute(executor),
        OpCode::AndAbsoluteX => And::absolute_x(executor),
        OpCode::AndAbsoluteY => And::absolute_y(executor),
        OpCode::AndIndirectX => And::indirect_x(executor),
        OpCode::AndIndirectY => And::indirect_y(executor),

        // BIT
        OpCode::BitZeropage => Bit::zeropage(executor),
        OpCode::BitAbsolute => Bit::absolute(executor),

        // CMP
        OpCode::CmpImmediate => Cmp::immediate(executor),
        OpCode::CmpZeropage => Cmp::zeropage(executor),
        OpCode::CmpZeropageX => Cmp::zeropage_x(executor),
        OpCode::CmpAbsolute => Cmp::absolute(executor),
        OpCode::CmpAbsoluteX => Cmp::absolute_x(executor),
        OpCode::CmpAbsoluteY => Cmp::absolute_y(executor),
        OpCode::CmpIndirectX => Cmp::indirect_x(executor),
        OpCode::CmpIndirectY => Cmp::indirect_y(executor),

        // LDA
        OpCode::LdaImmediate => Lda::immediate(executor),
        OpCode::LdaZeropage => Lda::zeropage(executor),
        OpCode::LdaZeropageX => Lda::zeropage_x(executor),
        OpCode::LdaAbsolute => Lda::absolute(executor),
        OpCode::LdaAbsoluteX => Lda::absolute_x(executor),
        OpCode::LdaAbsoluteY => Lda::absolute_y(executor),
        OpCode::LdaIndirectX => Lda::indirect_x(executor),
        OpCode::LdaIndirectY => Lda::indirect_y(executor),

        // LDX
        OpCode::LdxImmediate => Ldx::immediate(executor),
        OpCode::LdxZeropage => Ldx::zeropage(executor),
        OpCode::LdxZeropageY => Ldx::zeropage_y(executor),
        OpCode::LdxAbsolute => Ldx::absolute(executor),
        OpCode::LdxAbsoluteY => Ldx::absolute_y(executor),

        // LDY
        OpCode::LdyImmediate => Ldy::immediate(executor),
        OpCode::LdyZeropage => Ldy::zeropage(executor),
        OpCode::LdyZeropageX => Ldy::zeropage_x(executor),
        OpCode::LdyAbsolute => Ldy::absolute(executor),
        OpCode::LdyAbsoluteX => Ldy::absolute_x(executor),

        // SBC
        OpCode::SbcImmediate => Sbc::immediate(executor),
        OpCode::SbcZeropage => Sbc::zeropage(executor),
        OpCode::SbcZeropageX => Sbc::zeropage_x(executor),
        OpCode::SbcAbsolute => Sbc::absolute(executor),
        OpCode::SbcAbsoluteX => Sbc::absolute_x(executor),
        OpCode::SbcAbsoluteY => Sbc::absolute_y(executor),
        OpCode::SbcIndirectX => Sbc::indirect_x(executor),
        OpCode::SbcIndirectY => Sbc::indirect_y(executor),

        _ => unimplemented!(),
    }
}
