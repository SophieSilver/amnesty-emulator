use crate::cpu::{
    Cpu, StatusFlags,
    instructions::Bit,
    tests::{
        addressing_modes::{read::*, test_addressing_modes},
        test_args::BytePairs,
    },
};

impl TestReadInstruction for Bit {
    type Args = BytePairs;

    fn prepare(cpu: &mut Cpu, _: u8, a: u8) {
        cpu.a = a;
    }

    fn verify(cpu: &Cpu, b: u8, a: u8) {
        let result = a & b;
        let zero = result == 0;
        let negative_flag = (b as i8) < 0;
        let overflow_flag = (b >> 6) & 1 == 1;

        assert_eq!(
            cpu.flags.contains(StatusFlags::ZERO),
            zero,
            "ZERO flag set incorrectly"
        );
        assert_eq!(
            cpu.flags.contains(StatusFlags::NEGATIVE),
            negative_flag,
            "NEGATIVE flag set incorrectly"
        );
        assert_eq!(
            cpu.flags.contains(StatusFlags::OVERFLOW),
            overflow_flag,
            "OVERFLOW flag set incorrectly"
        );
    }
}

test_addressing_modes! {
    instruction: Bit,
    instruction_type: Read,
    addressing_modes: [
        Zeropage,
        Absolute,
    ],
}
