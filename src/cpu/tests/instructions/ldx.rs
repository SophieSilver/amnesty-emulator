use crate::cpu::{
    Cpu,
    instructions::Ldx,
    tests::{
        addressing_modes::{read::TestReadInstruction, test_addressing_modes},
        flags::check_nz_flags,
        test_args::SingleBytes,
    },
};

impl TestReadInstruction for Ldx {
    type Args = SingleBytes;

    fn prepare(_: &mut Cpu, _: u8, _: ()) {}

    fn verify(cpu: &Cpu, arg: u8, _: ()) {
        assert_eq!(cpu.x, arg);
        check_nz_flags(cpu.x, cpu.flags);
    }
}

test_addressing_modes! {
    instruction: Ldx,
    instruction_type: Read,
    addressing_modes: [
        Immediate,
        Zeropage,
        ZeropageY,
        Absolute,
        AbsoluteY,
    ],
}
