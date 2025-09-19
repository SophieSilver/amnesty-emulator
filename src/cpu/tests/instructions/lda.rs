use crate::cpu::{
    Cpu,
    instructions::Lda,
    tests::{
        addressing_modes::{read::*, test_addressing_modes},
        flags::check_nz_flags,
        test_args::SingleBytes,
    },
};

impl TestReadInstruction for Lda {
    type Args = SingleBytes;

    fn prepare(_: &mut Cpu, _: u8, _: ()) {}

    fn verify(cpu: &Cpu, arg: u8, _: ()) {
        assert_eq!(cpu.a, arg);
        check_nz_flags(cpu.a, cpu.flags);
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
