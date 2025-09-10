use crate::cpu::{
    Cpu,
    instructions::Txs,
    tests::addressing_modes::{implied::*, test_addressing_modes},
};

impl TestImpliedInstruction for Txs {
    fn prepare(cpu: &mut Cpu, arg: u8) {
        cpu.x = arg;
    }

    fn verify(cpu: &Cpu, arg: u8) {
        assert_eq!(cpu.sp, cpu.x);
        assert_eq!(cpu.x, arg);
    }
}

test_addressing_modes! {
    instruction: Txs,
    instruction_type: Implied,
}
