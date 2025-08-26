use crate::cpu::{
    instructions::{opcode::OpCode, Eor},
    tests::{
        addressing_modes::read::*, flags::check_negative_and_zero_flags, test_args::BytePairs,
    },
    Cpu,
};

impl TestReadInstruction for Eor {
    type Args = BytePairs;

    fn prepare(cpu: &mut Cpu, _: u8, a: u8) {
        cpu.a = a;
    }

    fn verify(cpu: &Cpu, b: u8, a: u8) {
        let result = a ^ b;

        assert_eq!(cpu.a, result, "bitwise EOR result incorrect");
        check_negative_and_zero_flags(cpu.a, cpu.flags);
    }
}
impl TestReadImmediate for Eor {
    const OPCODE: OpCode = OpCode::EorImmediate;
}

impl TestReadZeropage for Eor {
    const OPCODE: OpCode = OpCode::EorZeropage;
}

impl TestReadZeropageX for Eor {
    const OPCODE: OpCode = OpCode::EorZeropageX;
}

impl TestReadAbsolute for Eor {
    const OPCODE: OpCode = OpCode::EorAbsolute;
}

impl TestReadAbsoluteX for Eor {
    const OPCODE: OpCode = OpCode::EorAbsoluteX;
}

impl TestReadAbsoluteY for Eor {
    const OPCODE: OpCode = OpCode::EorAbsoluteY;
}

impl TestReadIndirectX for Eor {
    const OPCODE: OpCode = OpCode::EorIndirectX;
}

impl TestReadIndirectY for Eor {
    const OPCODE: OpCode = OpCode::EorIndirectY;
}

#[test]
fn immediate() {
    Eor::test_immediate();
}

#[test]
fn zeropage() {
    Eor::test_zeropage();
}

#[test]
fn zeropage_x() {
    Eor::test_zeropage_x();
}

#[test]
fn zeropage_x_overflow() {
    Eor::test_zeropage_x_overflow();
}

#[test]
fn absolute() {
    Eor::test_absolute();
}

#[test]
fn absolute_x() {
    Eor::test_absolute_x();
}

#[test]
fn absolute_y() {
    Eor::test_absolute_y();
}

#[test]
fn absolute_x_overflow() {
    Eor::test_absolute_x_overflow();
}

#[test]
fn absolute_y_overflow() {
    Eor::test_absolute_y_overflow();
}

#[test]
fn indirect_x() {
    Eor::test_indirect_x();
}

#[test]
fn indirect_y() {
    Eor::test_indirect_y();
}

#[test]
fn indirect_x_overflow() {
    Eor::test_indirect_x_overflow();
}

#[test]
fn indirect_y_overflow() {
    Eor::test_indirect_y_overflow();
}

#[test]
fn indirect_x_page_split() {
    Eor::test_indirect_x_page_split();
}

#[test]
fn indirect_y_page_split() {
    Eor::test_indirect_y_page_split();
}
