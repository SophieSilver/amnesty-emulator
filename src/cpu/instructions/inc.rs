use crate::cpu::{Cpu, addressing_modes::rmw::*};

pub struct Inc;

impl RmwInstruction for Inc {
    fn instruction(cpu: &mut Cpu, value: u8) -> u8 {
        let new_value = value.wrapping_add(1);
        cpu.set_nz_flags(new_value);

        new_value
    }
}

impl RmwZeropage for Inc {}
impl RmwZeropageX for Inc {}
impl RmwAbsolute for Inc {}
impl RmwAbsoluteX for Inc {}
