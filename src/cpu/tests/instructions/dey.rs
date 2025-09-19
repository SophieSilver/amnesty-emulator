use crate::cpu::{
    Cpu,
    instructions::Dey,
    tests::{
        addressing_modes::{implied::*, test_addressing_modes},
        flags::check_nz_flags,
    },
};

impl TestImpliedInstruction for Dey {
    fn prepare(cpu: &mut Cpu, arg: u8) {
        cpu.y = arg;
    }

    fn verify(cpu: &Cpu, arg: u8) {
        assert_eq!(cpu.y, arg.wrapping_sub(1));
        check_nz_flags(cpu.y, cpu.flags);
    }
}

test_addressing_modes! {
    instruction: Dey,
    instruction_type: Implied,
}
