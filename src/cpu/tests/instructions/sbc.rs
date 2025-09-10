use crate::cpu::{
    Cpu, StatusFlags,
    instructions::Sbc,
    tests::{
        addressing_modes::{read::TestReadInstruction, test_addressing_modes},
        flags::check_negative_and_zero_flags,
        test_args::BytePairsWithCarry,
    },
};

impl TestReadInstruction for Sbc {
    type Args = BytePairsWithCarry;

    fn prepare(cpu: &mut Cpu, _: u8, (a, carry): (u8, bool)) {
        cpu.a = a;
        cpu.flags.set(StatusFlags::CARRY, carry);
    }

    fn verify(cpu: &Cpu, b: u8, (a, carry): (u8, bool)) {
        let unsigned_result = (a as u32)
            .wrapping_sub(b as u32)
            .wrapping_sub(!carry as u32);

        let signed_result = a as i8 as i32 - b as i8 as i32 - !carry as i32;

        let should_carry = (u8::MIN as u32..=u8::MAX as u32).contains(&unsigned_result);
        let should_overflow = !(i8::MIN as i32..=i8::MAX as i32).contains(&signed_result);

        let truncated_result = unsigned_result as u8;

        assert_eq!(
            cpu.a, truncated_result,
            "Subtraction result incorrect a = {a}, b = {b}, carry = {carry}"
        );
        assert_eq!(
            cpu.flags.contains(StatusFlags::CARRY),
            should_carry,
            "CARRY flag set incorrectly a = {a}, b = {b}, carry = {carry}"
        );
        assert_eq!(
            cpu.flags.contains(StatusFlags::OVERFLOW),
            should_overflow,
            "OVERFLOW flag set incorrectly a = {a}, b = {b}, carry = {carry}"
        );

        check_negative_and_zero_flags(cpu.a, cpu.flags);
    }
}

test_addressing_modes! {
    instruction: Sbc,
    instruction_type: Read,
    addressing_modes: [
        Immediate,
        Zeropage,
        ZeropageX,
        Absolute,
        AbsoluteX,
        AbsoluteY,
        IndirectX,
        IndirectY,
    ],
}
