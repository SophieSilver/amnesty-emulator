use crate::cpu::{
    instructions::Nop,
    tests::{
        addressing_modes::{implied::*, test_addressing_modes},
        test_args::SingleBytes,
    },
    Cpu, StatusFlags,
};

const A_VALUE: u8 = 42;
const X_VALUE: u8 = 69;
const Y_VALUE: u8 = 0x22;
const SP_VALUE: u8 = 87;

impl TestImpliedInstruction for Nop {
    fn prepare(cpu: &mut Cpu, arg: u8) {
        cpu.flags = StatusFlags::from_bits_truncate(arg);
        cpu.a = A_VALUE;
        cpu.x = X_VALUE;
        cpu.y = Y_VALUE;
        cpu.sp = SP_VALUE;
    }

    fn verify(cpu: &Cpu, arg: u8) {
        assert_eq!(cpu.flags, StatusFlags::from_bits_truncate(arg));
        assert_eq!(cpu.a, A_VALUE);
        assert_eq!(cpu.x, X_VALUE);
        assert_eq!(cpu.y, Y_VALUE);
        assert_eq!(cpu.sp, SP_VALUE);
    }
}

test_addressing_modes! {
    instruction: Nop,
    instruction_type: Implied,
}
