use crate::cpu::{Cpu, StatusFlags, addressing_modes::rmw::*};

pub struct Lsr;

impl RmwInstruction for Lsr {
    fn instruction(cpu: &mut Cpu, value: u8) -> u8 {
        let shifted = value >> 1;
        let new_carry = value & 1 != 0;
        cpu.flags.set(StatusFlags::CARRY, new_carry);
        cpu.set_nz_flags(shifted);

        shifted
    }
}

impl RmwAccumulator for Lsr {}
impl RmwZeropage for Lsr {}
impl RmwZeropageX for Lsr {}
impl RmwAbsolute for Lsr {}
impl RmwAbsoluteX for Lsr {}
