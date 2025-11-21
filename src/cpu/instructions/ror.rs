use crate::cpu::{Cpu, StatusFlags, addressing_modes::rmw::*};

pub struct Ror;

impl RmwInstruction for Ror {
    fn instruction(cpu: &mut Cpu, value: u8) -> u8 {
        let carry = cpu.flags.contains(StatusFlags::CARRY);
        let rotated = value >> 1 | ((carry as u8) << 7);
        let new_carry = value & 1 != 0;
        cpu.flags.set(StatusFlags::CARRY, new_carry);
        cpu.set_nz_flags(rotated);

        rotated
    }
}

impl RmwAccumulator for Ror {}
impl RmwZeropage for Ror {}
impl RmwZeropageX for Ror {}
impl RmwAbsolute for Ror {}
impl RmwAbsoluteX for Ror {}
