use crate::cpu::{instructions::addressing_modes::read, register_getters::a_register, Cpu};

pub struct Ora;

impl read::Instruction for Ora {
    fn instruction(cpu: &mut Cpu, value: u8) {
        let result = cpu.a | value;
        cpu.set_register_with_flags(a_register, result);
    }
}

impl read::Immediate for Ora {}
impl read::Zeropage for Ora {}
impl read::ZeropageX for Ora {}
impl read::Absolute for Ora {}
impl read::AbsoluteX for Ora {}
impl read::AbsoluteY for Ora {}
impl read::IndirectX for Ora {}
impl read::IndirectY for Ora {}
