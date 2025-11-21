use crate::cpu::{
    Cpu, StatusFlags,
    instructions::Ror,
    tests::addressing_modes::{rmw::*, test_addressing_modes},
};

impl TestRmwInstruction for Ror {
    fn verify(cpu: &Cpu, arg: u8, previous_carry: bool) -> u8 {
        assert_eq!(
            cpu.flags.contains(StatusFlags::CARRY),
            arg & 1 != 0,
            "ROR must set carry correctly"
        );

        (previous_carry as u8) << 7 | arg >> 1
    }
}

test_addressing_modes! {
    instruction: Ror,
    instruction_type: Rmw,
    addressing_modes: [
        Accumulator,
        Zeropage,
        ZeropageX,
        Absolute,
        AbsoluteX,
    ]
}
