use crate::cpu::{
    Cpu, StatusFlags,
    instructions::Inc,
    tests::addressing_modes::{rmw::*, test_addressing_modes},
};

impl TestRmwInstruction for Inc {
    fn verify(cpu: &Cpu, arg: u8, previous_carry: bool) -> u8 {
        assert_eq!(
            cpu.flags.contains(StatusFlags::CARRY),
            previous_carry,
            "INC must not change CARRY"
        );

        arg.wrapping_add(1)
    }
}

test_addressing_modes! {
    instruction: Inc,
    instruction_type: Rmw,
    addressing_modes: [
        Zeropage,
        ZeropageX,
        Absolute,
        AbsoluteX,
    ],
}
