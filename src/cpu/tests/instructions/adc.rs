use crate::cpu::{
    Cpu, StatusFlags,
    instructions::Adc,
    tests::{
        addressing_modes::{read::*, test_addressing_modes},
        flags::check_nz_flags,
        test_args::BytePairsWithCarry,
    },
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

        check_nz_flags(cpu.a, cpu.flags);
    }
}

test_addressing_modes! {
    instruction: Adc,
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
