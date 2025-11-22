use crate::cpu::{
    Cpu, StatusFlags,
    instructions::Php,
    tests::addressing_modes::{stack_push::TestStackPushInstruction, test_addressing_modes},
};

impl TestStackPushInstruction for Php {
    fn prepare(cpu: &mut Cpu, arg: u8) {
        cpu.flags = StatusFlags::from_bits_truncate(arg);
    }

    fn verify(cpu: &mut Cpu, value_pushed: u8) {
        assert_eq!(
            (cpu.flags | StatusFlags::BREAK | StatusFlags::IGNORED).bits(),
            value_pushed
        );
    }
}

test_addressing_modes! {
    instruction: Php,
    instruction_type: StackPush,
}
