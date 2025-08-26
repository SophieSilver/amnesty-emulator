use crate::cpu::{
    instructions::{opcode::OpCode, Lda},
    tests::{
        addressing_modes::read::*, flags::check_negative_and_zero_flags, test_args::SingleBytes,
    },
    Cpu,
};

impl TestReadInstruction for Lda {
    type Args = SingleBytes;

    fn prepare(_: &mut Cpu, _: u8, _: ()) {}

    fn verify(cpu: &Cpu, arg: u8, _: ()) {
        assert_eq!(cpu.a, arg);
        check_negative_and_zero_flags(cpu.a, cpu.flags);
    }
}
impl TestReadImmediate for Lda {
    const OPCODE: OpCode = OpCode::LdaImmediate;
}

impl TestReadZeropage for Lda {
    const OPCODE: OpCode = OpCode::LdaZeropage;
}

impl TestReadZeropageX for Lda {
    const OPCODE: OpCode = OpCode::LdaZeropageX;
}

impl TestReadAbsolute for Lda {
    const OPCODE: OpCode = OpCode::LdaAbsolute;
}

impl TestReadAbsoluteX for Lda {
    const OPCODE: OpCode = OpCode::LdaAbsoluteX;
}

impl TestReadAbsoluteY for Lda {
    const OPCODE: OpCode = OpCode::LdaAbsoluteY;
}

impl TestReadIndirectX for Lda {
    const OPCODE: OpCode = OpCode::LdaIndirectX;
}

impl TestReadIndirectY for Lda {
    const OPCODE: OpCode = OpCode::LdaIndirectY;
}

#[test]
fn immediate() {
    Lda::test_immediate();
}

#[test]
fn zeropage() {
    Lda::test_zeropage();
}

#[test]
fn zeropage_x() {
    Lda::test_zeropage_x();
}

#[test]
fn zeropage_x_overflow() {
    Lda::test_zeropage_x_overflow();
}

#[test]
fn absolute() {
    Lda::test_absolute();
}

#[test]
fn absolute_x() {
    Lda::test_absolute_x();
}

#[test]
fn absolute_y() {
    Lda::test_absolute_y();
}

#[test]
fn absolute_x_overflow() {
    Lda::test_absolute_x_overflow();
}

#[test]
fn absolute_y_overflow() {
    Lda::test_absolute_y_overflow();
}

#[test]
fn indirect_x() {
    Lda::test_indirect_x();
}

#[test]
fn indirect_y() {
    Lda::test_indirect_y();
}

#[test]
fn indirect_x_overflow() {
    Lda::test_indirect_x_overflow();
}

#[test]
fn indirect_y_overflow() {
    Lda::test_indirect_y_overflow();
}

#[test]
fn indirect_x_page_split() {
    Lda::test_indirect_x_page_split();
}

#[test]
fn indirect_y_page_split() {
    Lda::test_indirect_y_page_split();
}
