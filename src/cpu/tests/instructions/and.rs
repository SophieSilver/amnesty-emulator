use crate::cpu::{
    Cpu,
    instructions::And,
    tests::{
        addressing_modes::{read::*, test_addressing_modes},
        flags::check_negative_and_zero_flags,
        test_args::BytePairs,
    },
};

impl TestReadInstruction for And {
    type Args = BytePairs;

    fn prepare(cpu: &mut Cpu, _: u8, a: u8) {
        cpu.a = a;
    }

    fn verify(cpu: &Cpu, b: u8, a: u8) {
        let result = a & b;

        assert_eq!(cpu.a, result, "bitwise AND result incorrect");
        check_negative_and_zero_flags(cpu.a, cpu.flags);
    }
}

test_addressing_modes! {
    instruction: And,
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
