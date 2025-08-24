use super::executor::Executor;
use crate::{
    cpu::instructions::addressing_modes::{implied::*, read::*, write::*},
    memory::Memory,
};

mod addressing_modes;

mod adc;
mod and;
mod bit;
mod cl_;
mod cmp;
mod de_;
mod eor;
mod in_;
mod lda;
mod ldx;
mod ldy;
mod nop;
mod ora;
mod sbc;
mod se_;
mod sta;
mod stx;
mod sty;
mod t__;

pub use adc::*;
pub use and::*;
pub use bit::*;
pub use cl_::*;
pub use cmp::*;
pub use de_::*;
pub use eor::*;
pub use in_::*;
pub use lda::*;
pub use ldx::*;
pub use ldy::*;
pub use nop::*;
pub use ora::*;
pub use sbc::*;
pub use se_::*;
pub use sta::*;
pub use stx::*;
pub use sty::*;
pub use t__::*;

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

        // CL*
        OpCode::Clc => Clc::implied(executor),
        OpCode::Cld => Cld::implied(executor),
        OpCode::Cli => Cli::implied(executor),
        OpCode::Clv => Clv::implied(executor),

        // CMP
        OpCode::CmpImmediate => Cmp::immediate(executor),
        OpCode::CmpZeropage => Cmp::zeropage(executor),
        OpCode::CmpZeropageX => Cmp::zeropage_x(executor),
        OpCode::CmpAbsolute => Cmp::absolute(executor),
        OpCode::CmpAbsoluteX => Cmp::absolute_x(executor),
        OpCode::CmpAbsoluteY => Cmp::absolute_y(executor),
        OpCode::CmpIndirectX => Cmp::indirect_x(executor),
        OpCode::CmpIndirectY => Cmp::indirect_y(executor),

        // DE*
        OpCode::Dex => Dex::implied(executor),
        OpCode::Dey => Dey::implied(executor),

        // EOR
        OpCode::EorImmediate => Eor::immediate(executor),
        OpCode::EorZeropage => Eor::zeropage(executor),
        OpCode::EorZeropageX => Eor::zeropage_x(executor),
        OpCode::EorAbsolute => Eor::absolute(executor),
        OpCode::EorAbsoluteX => Eor::absolute_x(executor),
        OpCode::EorAbsoluteY => Eor::absolute_y(executor),
        OpCode::EorIndirectX => Eor::indirect_x(executor),
        OpCode::EorIndirectY => Eor::indirect_y(executor),

        // IN*
        OpCode::Inx => Inx::implied(executor),
        OpCode::Iny => Iny::implied(executor),

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

        OpCode::Nop => Nop::implied(executor),

        // ORA
        OpCode::OraImmediate => Ora::immediate(executor),
        OpCode::OraZeropage => Ora::zeropage(executor),
        OpCode::OraZeropageX => Ora::zeropage_x(executor),
        OpCode::OraAbsolute => Ora::absolute(executor),
        OpCode::OraAbsoluteX => Ora::absolute_x(executor),
        OpCode::OraAbsoluteY => Ora::absolute_y(executor),
        OpCode::OraIndirectX => Ora::indirect_x(executor),
        OpCode::OraIndirectY => Ora::indirect_y(executor),

        // SBC
        OpCode::SbcImmediate => Sbc::immediate(executor),
        OpCode::SbcZeropage => Sbc::zeropage(executor),
        OpCode::SbcZeropageX => Sbc::zeropage_x(executor),
        OpCode::SbcAbsolute => Sbc::absolute(executor),
        OpCode::SbcAbsoluteX => Sbc::absolute_x(executor),
        OpCode::SbcAbsoluteY => Sbc::absolute_y(executor),
        OpCode::SbcIndirectX => Sbc::indirect_x(executor),
        OpCode::SbcIndirectY => Sbc::indirect_y(executor),

        // SE*
        OpCode::Sec => Sec::implied(executor),
        OpCode::Sed => Sed::implied(executor),
        OpCode::Sei => Sei::implied(executor),

        // STA
        OpCode::StaZeropage => Sta::zeropage(executor),
        OpCode::StaZeropageX => Sta::zeropage_x(executor),
        OpCode::StaAbsolute => Sta::absolute(executor),
        OpCode::StaAbsoluteX => Sta::absolute_x(executor),
        OpCode::StaAbsoluteY => Sta::absolute_y(executor),
        OpCode::StaIndirectX => Sta::indirect_x(executor),
        OpCode::StaIndirectY => Sta::indirect_y(executor),

        // STX
        OpCode::StxZeropage => Stx::zeropage(executor),
        OpCode::StxZeropageY => Stx::zeropage_y(executor),
        OpCode::StxAbsolute => Stx::absolute(executor),

        // Sty
        OpCode::StyZeropage => Sty::zeropage(executor),
        OpCode::StyZeropageX => Sty::zeropage_x(executor),
        OpCode::StyAbsolute => Sty::absolute(executor),

        // T**
        OpCode::Tax => Tax::implied(executor),
        OpCode::Tay => Tay::implied(executor),
        OpCode::Tsx => Tsx::implied(executor),
        OpCode::Txa => Txa::implied(executor),
        OpCode::Txs => Txs::implied(executor),
        OpCode::Tya => Tya::implied(executor),

        _ => unimplemented!(),
    }
}
