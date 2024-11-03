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
fn clc() {
    TestOpcodeOptions::new(OpCode::Clc, 2, |cpu, _memory| {
        assert!(!cpu.flags.contains(StatusFlags::CARRY))
    })
    .with_prepare(|cpu| cpu.flags.insert(StatusFlags::CARRY))
    .test();
}

#[test]
fn cld() {
    TestOpcodeOptions::new(OpCode::Cld, 2, |cpu, _memory| {
        assert!(!cpu.flags.contains(StatusFlags::DECIMAL))
    })
    .with_prepare(|cpu| cpu.flags.insert(StatusFlags::DECIMAL))
    .test();
}

#[test]
fn cli() {
    TestOpcodeOptions::new(OpCode::Cli, 2, |cpu, _memory| {
        assert!(!cpu.flags.contains(StatusFlags::INTERRUPT_DISABLE))
    })
    .with_prepare(|cpu| cpu.flags.insert(StatusFlags::INTERRUPT_DISABLE))
    .test();
}

#[test]
fn clv() {
    TestOpcodeOptions::new(OpCode::Clv, 2, |cpu, _memory| {
        assert!(!cpu.flags.contains(StatusFlags::OVERFLOW))
    })
    .with_prepare(|cpu| cpu.flags.insert(StatusFlags::OVERFLOW))
    .test();
}

#[test]
fn sec() {
    TestOpcodeOptions::new(OpCode::Sec, 2, |cpu, _memory| {
        assert!(cpu.flags.contains(StatusFlags::CARRY))
    })
    .with_prepare(|cpu| cpu.flags.remove(StatusFlags::CARRY))
    .test();
}

#[test]
fn sed() {
    TestOpcodeOptions::new(OpCode::Sed, 2, |cpu, _memory| {
        assert!(cpu.flags.contains(StatusFlags::DECIMAL))
    })
    .with_prepare(|cpu| cpu.flags.remove(StatusFlags::DECIMAL))
    .test();
}

#[test]
fn sei() {
    TestOpcodeOptions::new(OpCode::Sei, 2, |cpu, _memory| {
        assert!(cpu.flags.contains(StatusFlags::INTERRUPT_DISABLE))
    })
    .with_prepare(|cpu| cpu.flags.remove(StatusFlags::INTERRUPT_DISABLE))
    .test();
}

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
