use crate::cpu::{
    Cpu, StatusFlags,
    instructions::Cmp,
    tests::{
        addressing_modes::{read::*, test_addressing_modes},
        test_args::BytePairs,
    },
};

impl TestReadInstruction for Cmp {
    type Args = BytePairs;

    fn prepare(cpu: &mut Cpu, _: u8, a: u8) {
        cpu.a = a;
    }

    fn verify(cpu: &Cpu, b: u8, a: u8) {
        let z = a == b;
        let n = (a.wrapping_sub(b) as i8) < 0;
        let c = a >= b;

        assert_eq!(
            cpu.flags.contains(StatusFlags::CARRY),
            c,
            "CARRY flag set incorrectly, a = {a}, b = {b}"
        );
        assert_eq!(
            cpu.flags.contains(StatusFlags::NEGATIVE),
            n,
            "NEGATIVE flag set incorrectly, a = {a}, b = {b}"
        );
        assert_eq!(cpu.flags.contains(StatusFlags::ZERO), z);
    }
}

test_addressing_modes! {
    instruction: Cmp,
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
