use crate::cpu::{
    instructions::Txa,
    tests::{
        addressing_modes::{implied::*, test_addressing_modes},
        flags::check_negative_and_zero_flags,
    },
    Cpu,
};

impl TestImpliedInstruction for Txa {
    fn prepare(cpu: &mut Cpu, arg: u8) {
        cpu.x = arg;
    }

    fn verify(cpu: &Cpu, arg: u8) {
        assert_eq!(cpu.a, cpu.x);
        assert_eq!(cpu.x, arg);
        check_negative_and_zero_flags(cpu.a, cpu.flags);
    }
}

test_addressing_modes! {
    instruction: Txa,
    instruction_type: Implied,
}
