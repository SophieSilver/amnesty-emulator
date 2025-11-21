use crate::cpu::{Cpu, StatusFlags, addressing_modes::rmw::*};

pub struct Rol;

impl RmwInstruction for Rol {
    fn instruction(cpu: &mut Cpu, value: u8) -> u8 {
        let carry = cpu.flags.contains(StatusFlags::CARRY);
        let rotated = value << 1 | carry as u8;
        let new_carry = value >> 7 != 0;
        cpu.flags.set(StatusFlags::CARRY, new_carry);
        cpu.set_nz_flags(rotated);

        rotated
    }
}

impl RmwAccumulator for Rol {}
impl RmwZeropage for Rol {}
impl RmwZeropageX for Rol {}
impl RmwAbsolute for Rol {}
impl RmwAbsoluteX for Rol {}
