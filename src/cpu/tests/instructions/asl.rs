use crate::cpu::{
    Cpu, StatusFlags,
    instructions::Asl,
    tests::addressing_modes::{rmw::*, test_addressing_modes},
};

impl TestRmwInstruction for Asl {
    fn verify(cpu: &Cpu, arg: u8) -> u8 {
        assert_eq!(
            cpu.flags.contains(StatusFlags::CARRY),
            arg >> 7 != 0,
            "ASL must set carry correctly"
        );

        arg << 1
    }
}

test_addressing_modes! {
    instruction: Asl,
    instruction_type: Rmw,
    addressing_modes: [
        Accumulator,
        Zeropage,
        ZeropageX,
        Absolute,
        AbsoluteX,
    ]
}
