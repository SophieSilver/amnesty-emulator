use crate::cpu::{
    instructions::Tsx,
    tests::{
        addressing_modes::{implied::*, test_addressing_modes},
        flags::check_negative_and_zero_flags,
    },
    Cpu,
};

impl TestImpliedInstruction for Tsx {
    fn prepare(cpu: &mut Cpu, arg: u8) {
        cpu.sp = arg;
    }

    fn verify(cpu: &Cpu, arg: u8) {
        assert_eq!(cpu.x, cpu.sp);
        assert_eq!(cpu.sp, arg);
        check_negative_and_zero_flags(cpu.x, cpu.flags);
    }
}

test_addressing_modes! {
    instruction: Tsx,
    instruction_type: Implied,
}
