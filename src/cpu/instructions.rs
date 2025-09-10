use super::{
    addressing_modes::{implied::*, read::*, write::*},
    executor::Executor,
    opcode::Opcode,
};
use crate::memory::Memory;

mod adc;
mod and;
mod bit;
mod clc;
mod cld;
mod cli;
mod clv;
mod cmp;
mod cpx;
mod cpy;
mod dex;
mod dey;
mod eor;
mod inx;
mod iny;
mod lda;
mod ldx;
mod ldy;
mod nop;
mod ora;
mod sbc;
mod sec;
mod sed;
mod sei;
mod sta;
mod stx;
mod sty;
mod tax;
mod tay;
mod tsx;
mod txa;
mod txs;
mod tya;

pub use adc::*;
pub use and::*;
pub use bit::*;
pub use clc::*;
pub use cld::*;
pub use cli::*;
pub use clv::*;
pub use cmp::*;
pub use cpx::*;
pub use cpy::*;
pub use dex::*;
pub use dey::*;
pub use eor::*;
pub use inx::*;
pub use iny::*;
pub use lda::*;
pub use ldx::*;
pub use ldy::*;
pub use nop::*;
pub use ora::*;
pub use sbc::*;
pub use sec::*;
pub use sed::*;
pub use sei::*;
pub use sta::*;
pub use stx::*;
pub use sty::*;
pub use tax::*;
pub use tay::*;
pub use tsx::*;
pub use txa::*;
pub use txs::*;
pub use tya::*;

pub fn execute_opcode<M: Memory>(executor: &mut Executor<M>, opcode: Opcode) {
    match opcode {
        // ADC
        Opcode::AdcImmediate => Adc::immediate(executor),
        Opcode::AdcZeropage => Adc::zeropage(executor),
        Opcode::AdcZeropageX => Adc::zeropage_x(executor),
        Opcode::AdcAbsolute => Adc::absolute(executor),
        Opcode::AdcAbsoluteX => Adc::absolute_x(executor),
        Opcode::AdcAbsoluteY => Adc::absolute_y(executor),
        Opcode::AdcIndirectX => Adc::indirect_x(executor),
        Opcode::AdcIndirectY => Adc::indirect_y(executor),

        // AND
        Opcode::AndImmediate => And::immediate(executor),
        Opcode::AndZeropage => And::zeropage(executor),
        Opcode::AndZeropageX => And::zeropage_x(executor),
        Opcode::AndAbsolute => And::absolute(executor),
        Opcode::AndAbsoluteX => And::absolute_x(executor),
        Opcode::AndAbsoluteY => And::absolute_y(executor),
        Opcode::AndIndirectX => And::indirect_x(executor),
        Opcode::AndIndirectY => And::indirect_y(executor),

        // BIT
        Opcode::BitZeropage => Bit::zeropage(executor),
        Opcode::BitAbsolute => Bit::absolute(executor),

        // CL*
        Opcode::Clc => Clc::implied(executor),
        Opcode::Cld => Cld::implied(executor),
        Opcode::Cli => Cli::implied(executor),
        Opcode::Clv => Clv::implied(executor),

        // CMP
        Opcode::CmpImmediate => Cmp::immediate(executor),
        Opcode::CmpZeropage => Cmp::zeropage(executor),
        Opcode::CmpZeropageX => Cmp::zeropage_x(executor),
        Opcode::CmpAbsolute => Cmp::absolute(executor),
        Opcode::CmpAbsoluteX => Cmp::absolute_x(executor),
        Opcode::CmpAbsoluteY => Cmp::absolute_y(executor),
        Opcode::CmpIndirectX => Cmp::indirect_x(executor),
        Opcode::CmpIndirectY => Cmp::indirect_y(executor),

        // CPX
        Opcode::CpxImmediate => Cpx::immediate(executor),
        Opcode::CpxZeropage => Cpx::zeropage(executor),
        Opcode::CpxAbsolute => Cpx::absolute(executor),

        // CPY
        Opcode::CpyImmediate => Cpy::immediate(executor),
        Opcode::CpyZeropage => Cpy::zeropage(executor),
        Opcode::CpyAbsolute => Cpy::absolute(executor),

        // DE*
        Opcode::Dex => Dex::implied(executor),
        Opcode::Dey => Dey::implied(executor),

        // EOR
        Opcode::EorImmediate => Eor::immediate(executor),
        Opcode::EorZeropage => Eor::zeropage(executor),
        Opcode::EorZeropageX => Eor::zeropage_x(executor),
        Opcode::EorAbsolute => Eor::absolute(executor),
        Opcode::EorAbsoluteX => Eor::absolute_x(executor),
        Opcode::EorAbsoluteY => Eor::absolute_y(executor),
        Opcode::EorIndirectX => Eor::indirect_x(executor),
        Opcode::EorIndirectY => Eor::indirect_y(executor),

        // IN*
        Opcode::Inx => Inx::implied(executor),
        Opcode::Iny => Iny::implied(executor),

        // LDA
        Opcode::LdaImmediate => Lda::immediate(executor),
        Opcode::LdaZeropage => Lda::zeropage(executor),
        Opcode::LdaZeropageX => Lda::zeropage_x(executor),
        Opcode::LdaAbsolute => Lda::absolute(executor),
        Opcode::LdaAbsoluteX => Lda::absolute_x(executor),
        Opcode::LdaAbsoluteY => Lda::absolute_y(executor),
        Opcode::LdaIndirectX => Lda::indirect_x(executor),
        Opcode::LdaIndirectY => Lda::indirect_y(executor),

        // LDX
        Opcode::LdxImmediate => Ldx::immediate(executor),
        Opcode::LdxZeropage => Ldx::zeropage(executor),
        Opcode::LdxZeropageY => Ldx::zeropage_y(executor),
        Opcode::LdxAbsolute => Ldx::absolute(executor),
        Opcode::LdxAbsoluteY => Ldx::absolute_y(executor),

        // LDY
        Opcode::LdyImmediate => Ldy::immediate(executor),
        Opcode::LdyZeropage => Ldy::zeropage(executor),
        Opcode::LdyZeropageX => Ldy::zeropage_x(executor),
        Opcode::LdyAbsolute => Ldy::absolute(executor),
        Opcode::LdyAbsoluteX => Ldy::absolute_x(executor),

        Opcode::Nop => Nop::implied(executor),

        // ORA
        Opcode::OraImmediate => Ora::immediate(executor),
        Opcode::OraZeropage => Ora::zeropage(executor),
        Opcode::OraZeropageX => Ora::zeropage_x(executor),
        Opcode::OraAbsolute => Ora::absolute(executor),
        Opcode::OraAbsoluteX => Ora::absolute_x(executor),
        Opcode::OraAbsoluteY => Ora::absolute_y(executor),
        Opcode::OraIndirectX => Ora::indirect_x(executor),
        Opcode::OraIndirectY => Ora::indirect_y(executor),

        // SBC
        Opcode::SbcImmediate => Sbc::immediate(executor),
        Opcode::SbcZeropage => Sbc::zeropage(executor),
        Opcode::SbcZeropageX => Sbc::zeropage_x(executor),
        Opcode::SbcAbsolute => Sbc::absolute(executor),
        Opcode::SbcAbsoluteX => Sbc::absolute_x(executor),
        Opcode::SbcAbsoluteY => Sbc::absolute_y(executor),
        Opcode::SbcIndirectX => Sbc::indirect_x(executor),
        Opcode::SbcIndirectY => Sbc::indirect_y(executor),

        // SE*
        Opcode::Sec => Sec::implied(executor),
        Opcode::Sed => Sed::implied(executor),
        Opcode::Sei => Sei::implied(executor),

        // STA
        Opcode::StaZeropage => Sta::zeropage(executor),
        Opcode::StaZeropageX => Sta::zeropage_x(executor),
        Opcode::StaAbsolute => Sta::absolute(executor),
        Opcode::StaAbsoluteX => Sta::absolute_x(executor),
        Opcode::StaAbsoluteY => Sta::absolute_y(executor),
        Opcode::StaIndirectX => Sta::indirect_x(executor),
        Opcode::StaIndirectY => Sta::indirect_y(executor),

        // STX
        Opcode::StxZeropage => Stx::zeropage(executor),
        Opcode::StxZeropageY => Stx::zeropage_y(executor),
        Opcode::StxAbsolute => Stx::absolute(executor),

        // Sty
        Opcode::StyZeropage => Sty::zeropage(executor),
        Opcode::StyZeropageX => Sty::zeropage_x(executor),
        Opcode::StyAbsolute => Sty::absolute(executor),

        // T**
        Opcode::Tax => Tax::implied(executor),
        Opcode::Tay => Tay::implied(executor),
        Opcode::Tsx => Tsx::implied(executor),
        Opcode::Txa => Txa::implied(executor),
        Opcode::Txs => Txs::implied(executor),
        Opcode::Tya => Tya::implied(executor),

        _ => unimplemented!(),
    }
}
