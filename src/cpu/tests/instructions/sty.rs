use crate::cpu::{
    Cpu,
    instructions::Sty,
    tests::addressing_modes::{test_addressing_modes, write::TestWriteInstruction},
};

impl TestWriteInstruction for Sty {
    fn prepare(cpu: &mut Cpu, arg: u8) {
        cpu.y = arg;
    }

    fn expected_value(cpu: &Cpu) -> u8 {
        cpu.y
    }
}

test_addressing_modes! {
    instruction: Sty,
    instruction_type: Write,
    addressing_modes: [
        Zeropage,
        ZeropageX,
        Absolute,
    ],
}
