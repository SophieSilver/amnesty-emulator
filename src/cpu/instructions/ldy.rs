use crate::cpu::{instructions::addressing_modes::read, register_getters::y_register, Cpu};

pub struct Ldy;

impl read::Instruction for Ldy {
    fn instruction(cpu: &mut Cpu, value: u8) {
        cpu.set_register_with_flags(y_register, value);
    }
}

impl read::Immediate for Ldy {}
impl read::Zeropage for Ldy {}
impl read::ZeropageX for Ldy {}
impl read::Absolute for Ldy {}
impl read::AbsoluteX for Ldy {}
