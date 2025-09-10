use crate::cpu::{
    Cpu,
    instructions::Tax,
    tests::{
        addressing_modes::{implied::*, test_addressing_modes},
        flags::check_negative_and_zero_flags,
    },
};

impl TestImpliedInstruction for Tax {
    fn prepare(cpu: &mut Cpu, arg: u8) {
        cpu.a = arg;
    }

    fn verify(cpu: &Cpu, arg: u8) {
        assert_eq!(cpu.x, cpu.a);
        assert_eq!(cpu.a, arg);
        check_negative_and_zero_flags(cpu.x, cpu.flags);
    }
}

test_addressing_modes! {
    instruction: Tax,
    instruction_type: Implied,
}
