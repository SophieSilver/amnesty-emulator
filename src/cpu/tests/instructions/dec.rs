use crate::cpu::{
    Cpu, StatusFlags,
    instructions::Dec,
    tests::addressing_modes::{rmw::*, test_addressing_modes},
};

impl TestRmwInstruction for Dec {
    fn verify(cpu: &Cpu, arg: u8, previous_carry: bool) -> u8 {
        assert_eq!(
            cpu.flags.contains(StatusFlags::CARRY),
            previous_carry,
            "DEC must not change CARRY"
        );

        arg.wrapping_sub(1)
    }
}

test_addressing_modes! {
    instruction: Dec,
    instruction_type: Rmw,
    addressing_modes: [
        Zeropage,
        ZeropageX,
        Absolute,
        AbsoluteX,
    ],
}
