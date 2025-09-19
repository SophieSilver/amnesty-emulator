use crate::cpu::{
    Cpu,
    instructions::Ldy,
    tests::{
        addressing_modes::{read::TestReadInstruction, test_addressing_modes},
        flags::check_nz_flags,
        test_args::SingleBytes,
    },
};

impl TestReadInstruction for Ldy {
    type Args = SingleBytes;

    fn prepare(_: &mut Cpu, _: u8, _: ()) {}

    fn verify(cpu: &Cpu, arg: u8, _: ()) {
        assert_eq!(cpu.y, arg);
        check_nz_flags(cpu.y, cpu.flags);
    }
}

test_addressing_modes! {
    instruction: Ldy,
    instruction_type: Read,
    addressing_modes: [
        Immediate,
        Zeropage,
        ZeropageX,
        Absolute,
        AbsoluteX,
    ],
}
