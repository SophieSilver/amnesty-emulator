use crate::cpu::instructions::opcode::OpCode;

use super::*;

#[test]
fn immediate() {
    const TARGET: u8 = 0x42;

    TestOpcodeOptions::new(OpCode::LdxImmediate, 2, |cpu, _memory| {
        assert_eq!(cpu.x, TARGET);
    })
    .with_arguments(&[TARGET])
    .test();
}

#[test]
fn zeropage() {
    const TARGET: u8 = 0x21;
    const ADDR: u16 = 0x42;

    TestOpcodeOptions::new(OpCode::LdxZeropage, 3, |cpu, _memory| {
        assert_eq!(cpu.x, TARGET);
    })
    .with_arguments(&[ADDR as u8])
    .with_additional_values(&[(ADDR, TARGET)])
    .test();
}

#[test]
fn zeropage_y() {
    const TARGET: u8 = 0x69;
    const BASE_ADDR: u16 = 0x42;
    const OFFSET: u8 = 0x3;
    const FINAL_ADDR: u16 = (BASE_ADDR as u8).wrapping_add(OFFSET) as u16;

    TestOpcodeOptions::new(OpCode::LdxZeropageY, 4, |cpu, _memory| {
        assert_eq!(cpu.x, TARGET);
    })
    .with_arguments(&[BASE_ADDR as u8])
    .with_prepare(|cpu| cpu.y = OFFSET)
    .with_additional_values(&[(FINAL_ADDR, TARGET)])
    .test();
}

#[test]
fn zeropage_y_overflow() {
    const TARGET: u8 = 0x69;
    const BASE_ADDR: u16 = 0x42;
    const OFFSET: u8 = 0xFA;
    const FINAL_ADDR: u16 = (BASE_ADDR as u8).wrapping_add(OFFSET) as u16;

    TestOpcodeOptions::new(OpCode::LdxZeropageY, 4, |cpu, _memory| {
        assert_eq!(cpu.x, TARGET);
    })
    .with_arguments(&[BASE_ADDR as u8])
    .with_prepare(|cpu| cpu.y = OFFSET)
    .with_additional_values(&[(FINAL_ADDR, TARGET)])
    .test();
}

#[test]
fn absolute() {
    const TARGET: u8 = 0x75;
    const ADDR: u16 = 0x0457;

    TestOpcodeOptions::new(OpCode::LdxAbsolute, 4, |cpu, _memory| {
        assert_eq!(cpu.x, TARGET);
    })
    .with_arguments(&ADDR.to_le_bytes())
    .with_additional_values(&[(ADDR, TARGET)])
    .test();
}

#[test]
fn absolute_y() {
    const TARGET: u8 = 0x33;
    const BASE_ADDR: u16 = 0x0365;
    const OFFSET: u8 = 0x12;
    const FINAL_ADDR: u16 = BASE_ADDR.wrapping_add(OFFSET as u16);

    TestOpcodeOptions::new(OpCode::LdxAbsoluteY, 4, |cpu, _memory| {
        assert_eq!(cpu.x, TARGET);
    })
    .with_arguments(&BASE_ADDR.to_le_bytes())
    .with_prepare(|cpu| cpu.y = OFFSET)
    .with_additional_values(&[(FINAL_ADDR, TARGET)])
    .test();
}

#[test]
fn absolute_y_overflow() {
    const TARGET: u8 = 0x33;
    const BASE_ADDR: u16 = 0x0365;
    const OFFSET: u8 = 0xFE;
    const FINAL_ADDR: u16 = BASE_ADDR.wrapping_add(OFFSET as u16);

    TestOpcodeOptions::new(OpCode::LdxAbsoluteY, 5, |cpu, _memory| {
        assert_eq!(cpu.x, TARGET);
    })
    .with_arguments(&BASE_ADDR.to_le_bytes())
    .with_prepare(|cpu| cpu.y = OFFSET)
    .with_additional_values(&[(FINAL_ADDR, TARGET)])
    .test();
}
