use crate::cpu::{
    instructions::{opcode::OpCode, Cmp},
    tests::{addressing_modes::read::*, test_args::BytePairs},
    Cpu, StatusFlags,
};

impl TestReadInstruction for Cmp {
    type Args = BytePairs;

    fn prepare(cpu: &mut Cpu, _: u8, a: u8) {
        cpu.a = a;
    }

    fn verify(cpu: &Cpu, b: u8, a: u8) {
        let z = a == b;
        let n = (a.wrapping_sub(b) as i8) < 0;
        let c = a >= b;

        assert_eq!(
            cpu.flags.contains(StatusFlags::CARRY),
            c,
            "CARRY flag set incorrectly, a = {a}, b = {b}"
        );
        assert_eq!(
            cpu.flags.contains(StatusFlags::NEGATIVE),
            n,
            "NEGATIVE flag set incorrectly, a = {a}, b = {b}"
        );
        assert_eq!(cpu.flags.contains(StatusFlags::ZERO), z);
    }
}

impl TestReadImmediate for Cmp {
    const OPCODE: OpCode = OpCode::CmpImmediate;
}

impl TestReadZeropage for Cmp {
    const OPCODE: OpCode = OpCode::CmpZeropage;
}

impl TestReadZeropageX for Cmp {
    const OPCODE: OpCode = OpCode::CmpZeropageX;
}

impl TestReadAbsolute for Cmp {
    const OPCODE: OpCode = OpCode::CmpAbsolute;
}

impl TestReadAbsoluteX for Cmp {
    const OPCODE: OpCode = OpCode::CmpAbsoluteX;
}

impl TestReadAbsoluteY for Cmp {
    const OPCODE: OpCode = OpCode::CmpAbsoluteY;
}

impl TestReadIndirectX for Cmp {
    const OPCODE: OpCode = OpCode::CmpIndirectX;
}

impl TestReadIndirectY for Cmp {
    const OPCODE: OpCode = OpCode::CmpIndirectY;
}

#[test]
fn immediate() {
    Cmp::test_immediate();
}

#[test]
fn zeropage() {
    Cmp::test_zeropage();
}

#[test]
fn zeropage_x() {
    Cmp::test_zeropage_x();
}

#[test]
fn zeropage_x_overflow() {
    Cmp::test_zeropage_x_overflow();
}

#[test]
fn absolute() {
    Cmp::test_absolute();
}

#[test]
fn absolute_x() {
    Cmp::test_absolute_x();
}

#[test]
fn absolute_y() {
    Cmp::test_absolute_y();
}

#[test]
fn absolute_x_overflow() {
    Cmp::test_absolute_x_overflow();
}

#[test]
fn absolute_y_overflow() {
    Cmp::test_absolute_y_overflow();
}

#[test]
fn indirect_x() {
    Cmp::test_indirect_x();
}

#[test]
fn indirect_y() {
    Cmp::test_indirect_y();
}

#[test]
fn indirect_x_overflow() {
    Cmp::test_indirect_x_overflow();
}

#[test]
fn indirect_y_overflow() {
    Cmp::test_indirect_y_overflow();
}

#[test]
fn indirect_x_page_split() {
    Cmp::test_indirect_x_page_split();
}

#[test]
fn indirect_y_page_split() {
    Cmp::test_indirect_y_page_split();
}
