use crate::cpu::{
    instructions::Sec,
    tests::{
        addressing_modes::{implied::*, test_addressing_modes},
        test_args::SingleBytes,
    },
    Cpu, StatusFlags,
};

impl TestImpliedInstruction for Sec {
    fn prepare(cpu: &mut Cpu, arg: u8) {
        cpu.flags = StatusFlags::from_bits_truncate(arg);
    }

    fn verify(cpu: &Cpu, arg: u8) {
        assert_eq!(
            cpu.flags,
            StatusFlags::from_bits_truncate(arg).union(StatusFlags::CARRY)
        )
    }
}

test_addressing_modes! {
    instruction: Sec,
    instruction_type: Implied,
}
