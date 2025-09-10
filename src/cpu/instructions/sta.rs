use crate::cpu::{Cpu, addressing_modes::write::*};

pub struct Sta;

impl WriteInstruction for Sta {
    fn instruction(cpu: &Cpu) -> u8 {
        cpu.a
    }
}

impl WriteZeropage for Sta {}
impl WriteZeropageX for Sta {}
impl WriteAbsolute for Sta {}
impl WriteAbsoluteX for Sta {}
impl WriteAbsoluteY for Sta {}
impl WriteIndirectX for Sta {}
impl WriteIndirectY for Sta {}
