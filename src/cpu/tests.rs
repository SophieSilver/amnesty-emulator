#![allow(clippy::arithmetic_side_effects)]

use utils::TestOpcodeOptions;

use crate::cpu::dispatch::OpCode;

use super::StatusFlags;

#[allow(dead_code)]
mod consts;
#[allow(dead_code)]
mod utils;

mod adc;
mod and;
mod bit;
mod cmp;
mod eor;
mod lda;
mod ldx;
mod ldy;
mod ora;
mod sbc;

#[test]
fn nop() {
    let a = 2;
    let sp = 3;
    let x = 4;
    let y = 5;
    let flags = StatusFlags::CARRY | StatusFlags::NEGATIVE | StatusFlags::OVERFLOW;

    TestOpcodeOptions::new(OpCode::Nop, 2, |cpu, _memory| {
        assert_eq!(cpu.accumulator, a);
        assert_eq!(cpu.stack_ptr, sp);
        assert_eq!(cpu.x_index, x);
        assert_eq!(cpu.y_index, y);
        assert_eq!(cpu.flags, flags);
    })
    .with_prepare(|cpu| {
        cpu.accumulator = 2;
        cpu.stack_ptr = 3;
        cpu.x_index = 4;
        cpu.y_index = 5;
        cpu.flags = flags;
    })
    .test();
}
