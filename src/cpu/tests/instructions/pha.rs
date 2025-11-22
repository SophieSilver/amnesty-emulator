use crate::cpu::{
    Cpu, instructions::Pha, tests::addressing_modes::{stack_push::TestStackPushInstruction, test_addressing_modes}
};

impl TestStackPushInstruction for Pha {
    fn prepare(cpu: &mut Cpu, arg: u8) {
        cpu.a = arg;
    }

    fn verify(cpu: &mut Cpu, value_pushed: u8) {
        assert_eq!(cpu.a, value_pushed);
    }
}

test_addressing_modes! {
    instruction: Pha,
    instruction_type: StackPush,
}
