use crate::cpu::instructions::opcode::OpCode;

use super::*;

#[test]
pub(crate) fn clc() {
    TestOpcodeOptions::new(OpCode::Clc, 2, |cpu, _memory| {
        assert!(!cpu.flags.contains(StatusFlags::CARRY))
    })
    .with_prepare(|cpu| cpu.flags.insert(StatusFlags::CARRY))
    .test();
}

#[test]
pub(crate) fn cld() {
    TestOpcodeOptions::new(OpCode::Cld, 2, |cpu, _memory| {
        assert!(!cpu.flags.contains(StatusFlags::DECIMAL))
    })
    .with_prepare(|cpu| cpu.flags.insert(StatusFlags::DECIMAL))
    .test();
}

#[test]
pub(crate) fn cli() {
    TestOpcodeOptions::new(OpCode::Cli, 2, |cpu, _memory| {
        assert!(!cpu.flags.contains(StatusFlags::INTERRUPT_DISABLE))
    })
    .with_prepare(|cpu| cpu.flags.insert(StatusFlags::INTERRUPT_DISABLE))
    .test();
}

#[test]
pub(crate) fn clv() {
    TestOpcodeOptions::new(OpCode::Clv, 2, |cpu, _memory| {
        assert!(!cpu.flags.contains(StatusFlags::OVERFLOW))
    })
    .with_prepare(|cpu| cpu.flags.insert(StatusFlags::OVERFLOW))
    .test();
}

#[test]
pub(crate) fn sec() {
    TestOpcodeOptions::new(OpCode::Sec, 2, |cpu, _memory| {
        assert!(cpu.flags.contains(StatusFlags::CARRY))
    })
    .with_prepare(|cpu| cpu.flags.remove(StatusFlags::CARRY))
    .test();
}

#[test]
pub(crate) fn sed() {
    TestOpcodeOptions::new(OpCode::Sed, 2, |cpu, _memory| {
        assert!(cpu.flags.contains(StatusFlags::DECIMAL))
    })
    .with_prepare(|cpu| cpu.flags.remove(StatusFlags::DECIMAL))
    .test();
}

#[test]
pub(crate) fn sei() {
    TestOpcodeOptions::new(OpCode::Sei, 2, |cpu, _memory| {
        assert!(cpu.flags.contains(StatusFlags::INTERRUPT_DISABLE))
    })
    .with_prepare(|cpu| cpu.flags.remove(StatusFlags::INTERRUPT_DISABLE))
    .test();
}

#[test]
pub(crate) fn nop() {
    let a = 2;
    let sp = 3;
    let x = 4;
    let y = 5;
    let flags = StatusFlags::CARRY | StatusFlags::NEGATIVE | StatusFlags::OVERFLOW;

    TestOpcodeOptions::new(OpCode::Nop, 2, |cpu, _memory| {
        assert_eq!(cpu.a, a);
        assert_eq!(cpu.sp, sp);
        assert_eq!(cpu.x, x);
        assert_eq!(cpu.y, y);
        assert_eq!(cpu.flags, flags);
    })
    .with_prepare(|cpu| {
        cpu.a = 2;
        cpu.sp = 3;
        cpu.x = 4;
        cpu.y = 5;
        cpu.flags = flags;
    })
    .test();
}

#[test]
pub(crate) fn dex() {
    for x in 0..u8::MAX {
        let expected = x.wrapping_sub(1);

        TestOpcodeOptions::new(OpCode::Dex, 2, |cpu, _memory| {
            assert_eq!(cpu.x, expected, "X register decremented incorrectly");
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
        .with_prepare(|cpu| cpu.x = x)
        .test();
    }
}

#[test]
pub(crate) fn dey() {
    for y in 0..u8::MAX {
        let expected = y.wrapping_sub(1);

        TestOpcodeOptions::new(OpCode::Dey, 2, |cpu, _memory| {
            assert_eq!(cpu.y, expected, "Y register decremented incorrectly");
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
        .with_prepare(|cpu| cpu.y = y)
        .test();
    }
}

#[test]
pub(crate) fn inx() {
    for x in 0..u8::MAX {
        let expected = x.wrapping_add(1);

        TestOpcodeOptions::new(OpCode::Inx, 2, |cpu, _memory| {
            assert_eq!(cpu.x, expected, "X register incremented incorrectly");
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
        .with_prepare(|cpu| cpu.x = x)
        .test();
    }
}

#[test]
pub(crate) fn iny() {
    for y in 0..u8::MAX {
        let expected = y.wrapping_add(1);

        TestOpcodeOptions::new(OpCode::Iny, 2, |cpu, _memory| {
            assert_eq!(cpu.y, expected, "Y register incremented incorrectly");
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
        .with_prepare(|cpu| cpu.y = y)
        .test();
    }
}

#[test]
pub(crate) fn tax() {
    for value in 0..u8::MAX {
        TestOpcodeOptions::new(OpCode::Tax, 2, |cpu, _memory| {
            assert!(
                cpu.x == value && cpu.x == cpu.a,
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
        .with_prepare(|cpu| cpu.a = value)
        .test();
    }
}

#[test]
pub(crate) fn tay() {
    for value in 0..u8::MAX {
        TestOpcodeOptions::new(OpCode::Tay, 2, |cpu, _memory| {
            assert!(
                cpu.y == value && cpu.y == cpu.a,
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
        .with_prepare(|cpu| cpu.a = value)
        .test();
    }
}

#[test]
pub(crate) fn tsx() {
    for value in 0..u8::MAX {
        TestOpcodeOptions::new(OpCode::Tsx, 2, |cpu, _memory| {
            assert!(
                cpu.x == value && cpu.x == cpu.sp,
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
        .with_prepare(|cpu| cpu.sp = value)
        .test();
    }
}

#[test]
pub(crate) fn txa() {
    for value in 0..u8::MAX {
        TestOpcodeOptions::new(OpCode::Txa, 2, |cpu, _memory| {
            assert!(
                cpu.a == value && cpu.a == cpu.x,
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
        .with_prepare(|cpu| cpu.x = value)
        .test();
    }
}

#[test]
pub(crate) fn txs() {
    // TXS doesn't change flags
    for (value, negative, zero) in possible_values_with_2_bools() {
        TestOpcodeOptions::new(OpCode::Txs, 2, |cpu, _memory| {
            assert!(
                cpu.sp == value && cpu.sp == cpu.x,
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
            cpu.x = value;
            cpu.flags.set(StatusFlags::NEGATIVE, negative);
            cpu.flags.set(StatusFlags::ZERO, zero);
        })
        .test();
    }
}

#[test]
pub(crate) fn tya() {
    for value in 0..u8::MAX {
        TestOpcodeOptions::new(OpCode::Tya, 2, |cpu, _memory| {
            assert!(
                cpu.a == value && cpu.a == cpu.y,
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
        .with_prepare(|cpu| cpu.y = value)
        .test();
    }
}
