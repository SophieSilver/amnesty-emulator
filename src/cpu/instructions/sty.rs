use crate::cpu::{instructions::addressing_modes::write::*, Cpu};

pub struct Sty;

impl WriteInstruction for Sty {
    fn instruction(cpu: &Cpu) -> u8 {
        cpu.y
    }
}

impl WriteZeropage for Sty {}
impl WriteZeropageX for Sty {}
impl WriteAbsolute for Sty {}
