use crate::cpu::{
    Cpu, StatusFlags,
    instructions::Cpx,
    tests::{
        addressing_modes::{read::*, test_addressing_modes},
        test_args::BytePairs,
    },
};

impl TestReadInstruction for Cpx {
    type Args = BytePairs;

    fn prepare(cpu: &mut Cpu, _: u8, x: u8) {
        cpu.x = x;
    }

    fn verify(cpu: &Cpu, b: u8, x: u8) {
        let z = x == b;
        let n = (x.wrapping_sub(b) as i8) < 0;
        let c = x >= b;

        assert_eq!(
            cpu.flags.contains(StatusFlags::CARRY),
            c,
            "CARRY flag set incorrectly, x = {x}, b = {b}"
        );
        assert_eq!(
            cpu.flags.contains(StatusFlags::NEGATIVE),
            n,
            "NEGATIVE flag set incorrectly, x = {x}, b = {b}"
        );
        assert_eq!(cpu.flags.contains(StatusFlags::ZERO), z);
    }
}

test_addressing_modes! {
    instruction: Cpx,
    instruction_type: Read,
    addressing_modes: [
        Immediate,
        Zeropage,
        Absolute,
    ]
}
