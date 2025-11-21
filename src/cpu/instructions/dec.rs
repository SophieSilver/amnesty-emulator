use crate::cpu::{Cpu, addressing_modes::rmw::*};

pub struct Dec;

impl RmwInstruction for Dec {
    fn instruction(cpu: &mut Cpu, value: u8) -> u8 {
        let new_value = value.wrapping_sub(1);
        cpu.set_nz_flags(new_value);

        new_value
    }
}

impl RmwZeropage for Dec {}
impl RmwZeropageX for Dec {}
impl RmwAbsolute for Dec {}
impl RmwAbsoluteX for Dec {}
