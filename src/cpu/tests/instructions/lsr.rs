use crate::cpu::{
    Cpu, StatusFlags,
    instructions::Lsr,
    tests::addressing_modes::{rmw::*, test_addressing_modes},
};

impl TestRmwInstruction for Lsr {
    fn verify(cpu: &Cpu, arg: u8, _: bool) -> u8 {
        assert_eq!(
            cpu.flags.contains(StatusFlags::CARRY),
            arg & 1 != 0,
            "LSR must set carry correctly"
        );

        arg >> 1
    }
}

test_addressing_modes! {
    instruction: Lsr,
    instruction_type: Rmw,
    addressing_modes: [
        Accumulator,
        Zeropage,
        ZeropageX,
        Absolute,
        AbsoluteX,
    ]
}
