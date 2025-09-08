use crate::cpu::{
    instructions::Sed,
    tests::{
        addressing_modes::{implied::*, test_addressing_modes},
        test_args::SingleBytes,
    },
    Cpu, StatusFlags,
};

impl TestImpliedInstruction for Sed {
    fn prepare(cpu: &mut Cpu, arg: u8) {
        cpu.flags = StatusFlags::from_bits_truncate(arg);
    }

    fn verify(cpu: &Cpu, arg: u8) {
        assert_eq!(
            cpu.flags,
            StatusFlags::from_bits_truncate(arg).union(StatusFlags::DECIMAL)
        )
    }
}

test_addressing_modes! {
    instruction: Sed,
    instruction_type: Implied,
}
