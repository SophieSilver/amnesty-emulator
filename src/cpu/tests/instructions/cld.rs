use crate::cpu::{
    instructions::Cld,
    tests::addressing_modes::{implied::*, test_addressing_modes},
    Cpu, StatusFlags,
};

impl TestImpliedInstruction for Cld {
    fn prepare(cpu: &mut Cpu, arg: u8) {
        cpu.flags = StatusFlags::from_bits_truncate(arg);
    }

    fn verify(cpu: &Cpu, arg: u8) {
        assert_eq!(
            cpu.flags,
            StatusFlags::from_bits_truncate(arg).difference(StatusFlags::DECIMAL)
        )
    }
}

test_addressing_modes! {
    instruction: Cld,
    instruction_type: Implied,
}
