use crate::cpu::{
    instructions::{opcode::OpCode, And},
    tests::{addressing_modes::read::*, test_args::BytePairs},
    Cpu, StatusFlags,
};

impl TestReadInstruction for And {
    type Args = BytePairs;

    fn prepare(cpu: &mut Cpu, _: u8, a: u8) {
        cpu.a = a;
    }

    fn verify(cpu: &Cpu, b: u8, a: u8) {
        let result = a & b;

        assert_eq!(cpu.a, result, "bitwise AND result incorrect");
        assert_eq!(
            cpu.flags.contains(StatusFlags::NEGATIVE),
            (cpu.a as i8).is_negative(),
            "NEGATIVE flag set incorrectly"
        );
        assert_eq!(
            cpu.flags.contains(StatusFlags::ZERO),
            cpu.a == 0,
            "ZERO flag set incorrectly"
        );
    }
}

impl TestReadImmediate for And {
    const OPCODE: OpCode = OpCode::AndImmediate;
}

impl TestReadZeropage for And {
    const OPCODE: OpCode = OpCode::AndZeropage;
}

impl TestReadZeropageX for And {
    const OPCODE: OpCode = OpCode::AndZeropageX;
}

impl TestReadAbsolute for And {
    const OPCODE: OpCode = OpCode::AndAbsolute;
}

impl TestReadAbsoluteX for And {
    const OPCODE: OpCode = OpCode::AndAbsoluteX;
}

impl TestReadAbsoluteY for And {
    const OPCODE: OpCode = OpCode::AndAbsoluteY;
}

impl TestReadIndirectX for And {
    const OPCODE: OpCode = OpCode::AndIndirectX;
}

impl TestReadIndirectY for And {
    const OPCODE: OpCode = OpCode::AndIndirectY;
}

#[test]
fn immediate() {
    And::test_immediate();
}

#[test]
fn zeropage() {
    And::test_zeropage();
}

#[test]
fn zeropage_x() {
    And::test_zeropage_x();
}

#[test]
fn zeropage_x_overflow() {
    And::test_zeropage_x_overflow();
}

#[test]
fn absolute() {
    And::test_absolute();
}

#[test]
fn absolute_x() {
    And::test_absolute_x();
}

#[test]
fn absolute_y() {
    And::test_absolute_y();
}

#[test]
fn absolute_x_overflow() {
    And::test_absolute_x_overflow();
}

#[test]
fn absolute_y_overflow() {
    And::test_absolute_y_overflow();
}

#[test]
fn indirect_x() {
    And::test_indirect_x();
}

#[test]
fn indirect_y() {
    And::test_indirect_y();
}

#[test]
fn indirect_x_overflow() {
    And::test_indirect_x_overflow();
}

#[test]
fn indirect_y_overflow() {
    And::test_indirect_y_overflow();
}

#[test]
fn indirect_x_page_split() {
    And::test_indirect_x_page_split();
}

#[test]
fn indirect_y_page_split() {
    And::test_indirect_y_page_split();
}
