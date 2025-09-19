use crate::cpu::{
    Cpu,
    instructions::Dex,
    tests::{
        addressing_modes::{implied::*, test_addressing_modes},
        flags::check_nz_flags,
    },
};

impl TestImpliedInstruction for Dex {
    fn prepare(cpu: &mut Cpu, arg: u8) {
        cpu.x = arg;
    }

    fn verify(cpu: &Cpu, arg: u8) {
        assert_eq!(cpu.x, arg.wrapping_sub(1));
        check_nz_flags(cpu.x, cpu.flags);
    }
}

test_addressing_modes! {
    instruction: Dex,
    instruction_type: Implied,
}
