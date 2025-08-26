use crate::cpu::{
    instructions::Lda,
    tests::{
        addressing_modes::{read::*, test_addressing_modes},
        flags::check_negative_and_zero_flags,
        test_args::SingleBytes,
    },
    Cpu,
};

impl TestReadInstruction for Lda {
    type Args = SingleBytes;

    fn prepare(_: &mut Cpu, _: u8, _: ()) {}

    fn verify(cpu: &Cpu, arg: u8, _: ()) {
        assert_eq!(cpu.a, arg);
        check_negative_and_zero_flags(cpu.a, cpu.flags);
    }
}

test_addressing_modes! {
    instruction: Lda,
    instruction_type: Read,
    addressing_modes: [
        Immediate,
        Zeropage,
        ZeropageX,
        Absolute,
        AbsoluteX,
        AbsoluteY,
        IndirectX,
        IndirectY,
    ],
}
