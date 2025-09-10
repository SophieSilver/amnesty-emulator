use crate::cpu::{
    Cpu,
    instructions::Sta,
    tests::addressing_modes::{test_addressing_modes, write::TestWriteInstruction},
};

impl TestWriteInstruction for Sta {
    fn prepare(cpu: &mut Cpu, arg: u8) {
        cpu.a = arg;
    }

    fn expected_value(cpu: &Cpu) -> u8 {
        cpu.a
    }
}

test_addressing_modes! {
    instruction: Sta,
    instruction_type: Write,
    addressing_modes: [
        Zeropage,
        ZeropageX,
        Absolute,
        AbsoluteX,
        AbsoluteY,
        IndirectX,
        IndirectY,
    ],
}
