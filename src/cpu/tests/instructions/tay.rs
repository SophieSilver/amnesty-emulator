use crate::cpu::{
    instructions::Tay,
    tests::{
        addressing_modes::{implied::*, test_addressing_modes},
        flags::check_negative_and_zero_flags,
    },
    Cpu,
};

impl TestImpliedInstruction for Tay {
    fn prepare(cpu: &mut Cpu, arg: u8) {
        cpu.a = arg;
    }

    fn verify(cpu: &Cpu, arg: u8) {
        assert_eq!(cpu.y, cpu.a);
        assert_eq!(cpu.a, arg);
        check_negative_and_zero_flags(cpu.y, cpu.flags);
    }
}

test_addressing_modes! {
    instruction: Tay,
    instruction_type: Implied,
}
