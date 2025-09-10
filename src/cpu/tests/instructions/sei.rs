use crate::cpu::{
    Cpu, StatusFlags,
    instructions::Sei,
    tests::addressing_modes::{implied::*, test_addressing_modes},
};

impl TestImpliedInstruction for Sei {
    fn prepare(cpu: &mut Cpu, arg: u8) {
        cpu.flags = StatusFlags::from_bits_truncate(arg);
    }

    fn verify(cpu: &Cpu, arg: u8) {
        assert_eq!(
            cpu.flags,
            StatusFlags::from_bits_truncate(arg).union(StatusFlags::INTERRUPT_DISABLE)
        )
    }
}

test_addressing_modes! {
    instruction: Sei,
    instruction_type: Implied,
}
