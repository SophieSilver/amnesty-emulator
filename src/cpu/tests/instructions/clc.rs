use crate::cpu::{
    Cpu, StatusFlags,
    instructions::Clc,
    tests::addressing_modes::{implied::*, test_addressing_modes},
};

impl TestImpliedInstruction for Clc {
    fn prepare(cpu: &mut Cpu, arg: u8) {
        cpu.flags = StatusFlags::from_bits_truncate(arg);
    }

    fn verify(cpu: &Cpu, arg: u8) {
        assert_eq!(
            cpu.flags,
            StatusFlags::from_bits_truncate(arg).difference(StatusFlags::CARRY)
        )
    }
}

test_addressing_modes! {
    instruction: Clc,
    instruction_type: Implied,
}
