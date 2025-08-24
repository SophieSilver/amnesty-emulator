use crate::cpu::{instructions::addressing_modes::write::*, Cpu};

pub struct Stx;

impl WriteInstruction for Stx {
    fn instruction(cpu: &Cpu) -> u8 {
        cpu.x
    }
}

impl WriteZeropage for Stx {}
impl WriteZeropageY for Stx {}
impl WriteAbsolute for Stx {}
