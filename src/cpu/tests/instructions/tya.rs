use crate::cpu::{
    Cpu,
    instructions::Tya,
    tests::{
        addressing_modes::{implied::*, test_addressing_modes},
        flags::check_negative_and_zero_flags,
    },
};

impl TestImpliedInstruction for Tya {
    fn prepare(cpu: &mut Cpu, arg: u8) {
        cpu.y = arg;
    }

    fn verify(cpu: &Cpu, arg: u8) {
        assert_eq!(cpu.a, cpu.y);
        assert_eq!(cpu.y, arg);
        check_negative_and_zero_flags(cpu.a, cpu.flags);
    }
}

test_addressing_modes! {
    instruction: Tya,
    instruction_type: Implied,
}
