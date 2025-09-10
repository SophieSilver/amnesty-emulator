use crate::cpu::{
    Cpu,
    instructions::Inx,
    tests::{
        addressing_modes::{implied::*, test_addressing_modes},
        flags::check_negative_and_zero_flags,
    },
};

impl TestImpliedInstruction for Inx {
    fn prepare(cpu: &mut Cpu, arg: u8) {
        cpu.x = arg;
    }

    fn verify(cpu: &Cpu, arg: u8) {
        assert_eq!(cpu.x, arg.wrapping_add(1));
        check_negative_and_zero_flags(cpu.x, cpu.flags);
    }
}

test_addressing_modes! {
    instruction: Inx,
    instruction_type: Implied,
}
