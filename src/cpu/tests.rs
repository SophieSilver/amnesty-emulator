#![allow(clippy::arithmetic_side_effects)]
use utils::{possible_values_with_2_bools, TestOpcodeOptions};

use crate::{
    cpu::dispatch::OpCode,
    memory::{ram::Ram, Memory},
};

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

#[derive(Debug, Clone)]
struct TestMemory {
    ram: Ram,
}

impl TestMemory {
    pub fn new() -> Self {
        Self { ram: Ram::new() }
    }
}

impl Memory for TestMemory {
    fn load(&mut self, address: u16) -> u8 {
        self.ram.load(address)
    }

    fn store(&mut self, address: u16, value: u8) {
        self.ram.store(address, value)
    }
}

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

#[test]
fn dex() {
    for x in 0..u8::MAX {
        let expected = x.wrapping_sub(1);

        TestOpcodeOptions::new(OpCode::Dex, 2, |cpu, _memory| {
            assert_eq!(cpu.x_index, expected, "X register decremented incorrectly");
            assert_eq!(
                cpu.flags.contains(StatusFlags::NEGATIVE),
                (expected as i8) < 0,
                "NEGATIVE flag set incorrectly"
            );
            assert_eq!(
                cpu.flags.contains(StatusFlags::ZERO),
                expected == 0,
                "ZERO flag set incorrectly"
            );
        })
        .with_prepare(|cpu| cpu.x_index = x)
        .test();
    }
}

#[test]
fn dey() {
    for y in 0..u8::MAX {
        let expected = y.wrapping_sub(1);

        TestOpcodeOptions::new(OpCode::Dey, 2, |cpu, _memory| {
            assert_eq!(cpu.y_index, expected, "Y register decremented incorrectly");
            assert_eq!(
                cpu.flags.contains(StatusFlags::NEGATIVE),
                (expected as i8) < 0,
                "NEGATIVE flag set incorrectly"
            );
            assert_eq!(
                cpu.flags.contains(StatusFlags::ZERO),
                expected == 0,
                "ZERO flag set incorrectly"
            );
        })
        .with_prepare(|cpu| cpu.y_index = y)
        .test();
    }
}

#[test]
fn inx() {
    for x in 0..u8::MAX {
        let expected = x.wrapping_add(1);

        TestOpcodeOptions::new(OpCode::Inx, 2, |cpu, _memory| {
            assert_eq!(cpu.x_index, expected, "X register incremented incorrectly");
            assert_eq!(
                cpu.flags.contains(StatusFlags::NEGATIVE),
                (expected as i8) < 0,
                "NEGATIVE flag set incorrectly"
            );
            assert_eq!(
                cpu.flags.contains(StatusFlags::ZERO),
                expected == 0,
                "ZERO flag set incorrectly"
            );
        })
        .with_prepare(|cpu| cpu.x_index = x)
        .test();
    }
}

#[test]
fn iny() {
    for y in 0..u8::MAX {
        let expected = y.wrapping_add(1);

        TestOpcodeOptions::new(OpCode::Iny, 2, |cpu, _memory| {
            assert_eq!(cpu.y_index, expected, "Y register incremented incorrectly");
            assert_eq!(
                cpu.flags.contains(StatusFlags::NEGATIVE),
                (expected as i8) < 0,
                "NEGATIVE flag set incorrectly"
            );
            assert_eq!(
                cpu.flags.contains(StatusFlags::ZERO),
                expected == 0,
                "ZERO flag set incorrectly"
            );
        })
        .with_prepare(|cpu| cpu.y_index = y)
        .test();
    }
}

#[test]
fn tax() {
    for value in 0..u8::MAX {
        TestOpcodeOptions::new(OpCode::Tax, 2, |cpu, _memory| {
            assert!(
                cpu.x_index == value && cpu.x_index == cpu.accumulator,
                "register transfered incorrectly"
            );
            assert_eq!(
                cpu.flags.contains(StatusFlags::NEGATIVE),
                (value as i8) < 0,
                "NEGATIVE flag set incorrectly"
            );
            assert_eq!(
                cpu.flags.contains(StatusFlags::ZERO),
                value == 0,
                "ZERO flag set incorrectly"
            );
        })
        .with_prepare(|cpu| cpu.accumulator = value)
        .test();
    }
}

#[test]
fn tay() {
    for value in 0..u8::MAX {
        TestOpcodeOptions::new(OpCode::Tay, 2, |cpu, _memory| {
            assert!(
                cpu.y_index == value && cpu.y_index == cpu.accumulator,
                "register transfered incorrectly"
            );
            assert_eq!(
                cpu.flags.contains(StatusFlags::NEGATIVE),
                (value as i8) < 0,
                "NEGATIVE flag set incorrectly"
            );
            assert_eq!(
                cpu.flags.contains(StatusFlags::ZERO),
                value == 0,
                "ZERO flag set incorrectly"
            );
        })
        .with_prepare(|cpu| cpu.accumulator = value)
        .test();
    }
}

#[test]
fn tsx() {
    for value in 0..u8::MAX {
        TestOpcodeOptions::new(OpCode::Tsx, 2, |cpu, _memory| {
            assert!(
                cpu.x_index == value && cpu.x_index == cpu.stack_ptr,
                "register transfered incorrectly"
            );
            assert_eq!(
                cpu.flags.contains(StatusFlags::NEGATIVE),
                (value as i8) < 0,
                "NEGATIVE flag set incorrectly"
            );
            assert_eq!(
                cpu.flags.contains(StatusFlags::ZERO),
                value == 0,
                "ZERO flag set incorrectly"
            );
        })
        .with_prepare(|cpu| cpu.stack_ptr = value)
        .test();
    }
}

#[test]
fn txa() {
    for value in 0..u8::MAX {
        TestOpcodeOptions::new(OpCode::Txa, 2, |cpu, _memory| {
            assert!(
                cpu.accumulator == value && cpu.accumulator == cpu.x_index,
                "register transfered incorrectly"
            );
            assert_eq!(
                cpu.flags.contains(StatusFlags::NEGATIVE),
                (value as i8) < 0,
                "NEGATIVE flag set incorrectly"
            );
            assert_eq!(
                cpu.flags.contains(StatusFlags::ZERO),
                value == 0,
                "ZERO flag set incorrectly"
            );
        })
        .with_prepare(|cpu| cpu.x_index = value)
        .test();
    }
}

#[test]
fn txs() {
    // TXS doesn't change flags
    for (value, negative, zero) in possible_values_with_2_bools() {
        TestOpcodeOptions::new(OpCode::Txs, 2, |cpu, _memory| {
            assert!(
                cpu.stack_ptr == value && cpu.stack_ptr == cpu.x_index,
                "register transfered incorrectly"
            );
            assert_eq!(
                cpu.flags.contains(StatusFlags::NEGATIVE),
                negative,
                "NEGATIVE flag modified incorrectly"
            );
            assert_eq!(
                cpu.flags.contains(StatusFlags::ZERO),
                zero,
                "ZERO flag modified incorrectly"
            );
        })
        .with_prepare(|cpu| {
            cpu.x_index = value;
            cpu.flags.set(StatusFlags::NEGATIVE, negative);
            cpu.flags.set(StatusFlags::ZERO, zero);
        })
        .test();
    }
}

#[test]
fn tya() {
    for value in 0..u8::MAX {
        TestOpcodeOptions::new(OpCode::Tya, 2, |cpu, _memory| {
            assert!(
                cpu.accumulator == value && cpu.accumulator == cpu.y_index,
                "register transfered incorrectly"
            );
            assert_eq!(
                cpu.flags.contains(StatusFlags::NEGATIVE),
                (value as i8) < 0,
                "NEGATIVE flag set incorrectly"
            );
            assert_eq!(
                cpu.flags.contains(StatusFlags::ZERO),
                value == 0,
                "ZERO flag set incorrectly"
            );
        })
        .with_prepare(|cpu| cpu.y_index = value)
        .test();
    }
}
