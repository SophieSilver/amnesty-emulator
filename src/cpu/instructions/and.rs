use crate::cpu::{instructions::addressing_modes::read, register_getters::a_register, Cpu};

pub struct And;

impl read::Instruction for And {
    fn instruction(cpu: &mut Cpu, value: u8) {
        let result = cpu.a & value;
        cpu.set_register_with_flags(a_register, result);
    }
}

impl read::Immediate for And {}
impl read::Zeropage for And {}
impl read::ZeropageX for And {}
impl read::Absolute for And {}
impl read::AbsoluteX for And {}
impl read::AbsoluteY for And {}
impl read::IndirectX for And {}
impl read::IndirectY for And {}
