use crate::cpu::{
    Cpu, StatusFlags,
    instructions::Cpy,
    tests::{
        addressing_modes::{read::*, test_addressing_modes},
        test_args::BytePairs,
    },
};

impl TestReadInstruction for Cpy {
    type Args = BytePairs;

    fn prepare(cpu: &mut Cpu, _: u8, y: u8) {
        cpu.y = y;
    }

    fn verify(cpu: &Cpu, b: u8, y: u8) {
        let z = y == b;
        let n = (y.wrapping_sub(b) as i8) < 0;
        let c = y >= b;

        assert_eq!(
            cpu.flags.contains(StatusFlags::CARRY),
            c,
            "CARRY flag set incorrectly, y = {y}, b = {b}"
        );
        assert_eq!(
            cpu.flags.contains(StatusFlags::NEGATIVE),
            n,
            "NEGATIVE flag set incorrectly, y = {y}, b = {b}"
        );
        assert_eq!(cpu.flags.contains(StatusFlags::ZERO), z);
    }
}

test_addressing_modes! {
    instruction: Cpy,
    instruction_type: Read,
    addressing_modes: [
        Immediate,
        Zeropage,
        Absolute,
    ]
}
