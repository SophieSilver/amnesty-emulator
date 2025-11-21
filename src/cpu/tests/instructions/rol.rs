use crate::cpu::{
    Cpu, StatusFlags,
    instructions::Rol,
    tests::addressing_modes::{rmw::*, test_addressing_modes},
};

impl TestRmwInstruction for Rol {
    fn verify(cpu: &Cpu, arg: u8, previous_carry: bool) -> u8 {
        assert_eq!(
            cpu.flags.contains(StatusFlags::CARRY),
            arg >> 7 != 0,
            "ROL must set carry correctly"
        );

        previous_carry as u8 | arg << 1
    }
}

test_addressing_modes! {
    instruction: Rol,
    instruction_type: Rmw,
    addressing_modes: [
        Accumulator,
        Zeropage,
        ZeropageX,
        Absolute,
        AbsoluteX,
    ]
}
