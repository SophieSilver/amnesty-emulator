use crate::cpu::{
    instructions::{opcode::OpCode, Adc},
    tests::{addressing_modes::read::*, test_args::BytePairsWithCarry},
    Cpu, StatusFlags,
};

impl TestReadInstruction for Adc {
    type Args = BytePairsWithCarry;

    fn prepare(cpu: &mut Cpu, _: u8, (a, carry): (u8, bool)) {
        cpu.a = a;
        cpu.flags.set(StatusFlags::CARRY, carry);
    }

    fn verify(cpu: &Cpu, b: u8, (a, carry): (u8, bool)) {
        // just dump everything into u32 and see what's out of range
        let unsigned_result = a as u32 + b as u32 + carry as u32;
        // first casting to i8 to have it sign extended
        let signed_result = a as i8 as i32 + b as i8 as i32 + carry as i32;

        let should_carry = !(u8::MIN as u32..=u8::MAX as u32).contains(&unsigned_result);
        let should_overflow = !(i8::MIN as i32..=i8::MAX as i32).contains(&signed_result);

        assert_eq!(
            cpu.a,
            a.wrapping_add(b).wrapping_add(carry as u8),
            "Addition result incorrect"
        );
        assert_eq!(
            cpu.flags.contains(StatusFlags::CARRY),
            should_carry,
            "CARRY flag set incorrectly"
        );
        assert_eq!(
            cpu.flags.contains(StatusFlags::OVERFLOW),
            should_overflow,
            "OVERFLOW flag set incorrectly"
        );
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

impl TestReadImmediate for Adc {
    const OPCODE: OpCode = OpCode::AdcImmediate;
}

impl TestReadZeropage for Adc {
    const OPCODE: OpCode = OpCode::AdcZeropage;
}

impl TestReadZeropageX for Adc {
    const OPCODE: OpCode = OpCode::AdcZeropageX;
}

impl TestReadAbsolute for Adc {
    const OPCODE: OpCode = OpCode::AdcAbsolute;
}

impl TestReadAbsoluteX for Adc {
    const OPCODE: OpCode = OpCode::AdcAbsoluteX;
}

impl TestReadAbsoluteY for Adc {
    const OPCODE: OpCode = OpCode::AdcAbsoluteY;
}

impl TestReadIndirectX for Adc {
    const OPCODE: OpCode = OpCode::AdcIndirectX;
}

impl TestReadIndirectY for Adc {
    const OPCODE: OpCode = OpCode::AdcIndirectY;
}

#[test]
fn immediate() {
    Adc::test_immediate();
}

#[test]
fn zeropage() {
    Adc::test_zeropage();
}

#[test]
fn zeropage_x() {
    Adc::test_zeropage_x();
}

#[test]
fn zeropage_x_overflow() {
    Adc::test_zeropage_x_overflow();
}

#[test]
fn absolute() {
    Adc::test_absolute();
}

#[test]
fn absolute_x() {
    Adc::test_absolute_x();
}

#[test]
fn absolute_y() {
    Adc::test_absolute_y();
}

#[test]
fn absolute_x_overflow() {
    Adc::test_absolute_x_overflow();
}

#[test]
fn absolute_y_overflow() {
    Adc::test_absolute_y_overflow();
}

#[test]
fn indirect_x() {
    Adc::test_indirect_x();
}

#[test]
fn indirect_y() {
    Adc::test_indirect_y();
}

#[test]
fn indirect_x_overflow() {
    Adc::test_indirect_x_overflow();
}

#[test]
fn indirect_y_overflow() {
    Adc::test_indirect_y_overflow();
}

#[test]
fn indirect_x_page_split() {
    Adc::test_indirect_x_page_split();
}

#[test]
fn indirect_y_page_split() {
    Adc::test_indirect_y_page_split();
}
