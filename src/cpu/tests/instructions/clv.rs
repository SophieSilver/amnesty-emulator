use crate::cpu::{
    instructions::Clv,
    tests::addressing_modes::{implied::*, test_addressing_modes},
    Cpu, StatusFlags,
};

impl TestImpliedInstruction for Clv {
    fn prepare(cpu: &mut Cpu, arg: u8) {
        cpu.flags = StatusFlags::from_bits_truncate(arg);
    }

    fn verify(cpu: &Cpu, arg: u8) {
        assert_eq!(
            cpu.flags,
            StatusFlags::from_bits_truncate(arg).difference(StatusFlags::OVERFLOW)
        )
    }
}

test_addressing_modes! {
    instruction: Clv,
    instruction_type: Implied,
}
