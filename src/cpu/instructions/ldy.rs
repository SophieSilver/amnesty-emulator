use crate::cpu::{instructions::addressing_modes::read::*, register_getters::y_register, Cpu};

pub struct Ldy;

impl ReadInstruction for Ldy {
    fn instruction(cpu: &mut Cpu, value: u8) {
        cpu.set_register_with_flags(y_register, value);
    }
}

impl ReadImmediate for Ldy {}
impl ReadZeropage for Ldy {}
impl ReadZeropageX for Ldy {}
impl ReadAbsolute for Ldy {}
impl ReadAbsoluteX for Ldy {}
