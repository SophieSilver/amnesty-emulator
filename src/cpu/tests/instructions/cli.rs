use crate::cpu::{
    Cpu, StatusFlags,
    instructions::Cli,
    tests::addressing_modes::{implied::*, test_addressing_modes},
};

impl TestImpliedInstruction for Cli {
    fn prepare(cpu: &mut Cpu, arg: u8) {
        cpu.flags = StatusFlags::from_bits_truncate(arg);
    }

    fn verify(cpu: &Cpu, arg: u8) {
        assert_eq!(
            cpu.flags,
            StatusFlags::from_bits_truncate(arg).difference(StatusFlags::INTERRUPT_DISABLE)
        )
    }
}

test_addressing_modes! {
    instruction: Cli,
    instruction_type: Implied,
}
