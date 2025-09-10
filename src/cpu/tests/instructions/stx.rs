use crate::cpu::{
    Cpu,
    instructions::Stx,
    tests::addressing_modes::{test_addressing_modes, write::TestWriteInstruction},
};

impl TestWriteInstruction for Stx {
    fn prepare(cpu: &mut Cpu, arg: u8) {
        cpu.x = arg;
    }

    fn expected_value(cpu: &Cpu) -> u8 {
        cpu.x
    }
}

test_addressing_modes! {
    instruction: Stx,
    instruction_type: Write,
    addressing_modes: [
        Zeropage,
        ZeropageY,
        Absolute,
    ],
}
