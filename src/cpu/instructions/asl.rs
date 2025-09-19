use crate::cpu::{Cpu, StatusFlags, addressing_modes::rmw::*};

pub struct Asl;

impl RmwInstruction for Asl {
    fn instruction(cpu: &mut Cpu, value: u8) -> u8 {
        let shifted = (value as u16) << 1;
        let new_carry = shifted >> 8 != 0;
        cpu.flags.set(StatusFlags::CARRY, new_carry);

        let output = shifted as u8;
        cpu.set_nz_flags(output);

        output
    }
}

impl RmwAccumulator for Asl {}
impl RmwZeropage for Asl {}
impl RmwZeropageX for Asl {}
impl RmwAbsolute for Asl {}
impl RmwAbsoluteX for Asl {}
