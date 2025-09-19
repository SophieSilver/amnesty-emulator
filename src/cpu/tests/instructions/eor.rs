use crate::cpu::{
    Cpu,
    instructions::Eor,
    tests::{
        addressing_modes::{read::*, test_addressing_modes},
        flags::check_nz_flags,
        test_args::BytePairs,
    },
};

impl TestReadInstruction for Eor {
    type Args = BytePairs;

    fn prepare(cpu: &mut Cpu, _: u8, a: u8) {
        cpu.a = a;
    }

    fn verify(cpu: &Cpu, b: u8, a: u8) {
        let result = a ^ b;

        assert_eq!(cpu.a, result, "bitwise EOR result incorrect");
        check_nz_flags(cpu.a, cpu.flags);
    }
}

test_addressing_modes! {
    instruction: Eor,
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
