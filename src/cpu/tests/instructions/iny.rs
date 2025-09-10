use crate::cpu::{
    Cpu,
    instructions::Iny,
    tests::{
        addressing_modes::{implied::*, test_addressing_modes},
        flags::check_negative_and_zero_flags,
    },
};

impl TestImpliedInstruction for Iny {
    fn prepare(cpu: &mut Cpu, arg: u8) {
        cpu.y = arg;
    }

    fn verify(cpu: &Cpu, arg: u8) {
        assert_eq!(cpu.y, arg.wrapping_add(1));
        check_negative_and_zero_flags(cpu.y, cpu.flags);
    }
}

test_addressing_modes! {
    instruction: Iny,
    instruction_type: Implied,
}
