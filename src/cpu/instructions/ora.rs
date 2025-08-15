use crate::cpu::{instructions::addressing_modes::read::*, register_getters::a_register, Cpu};

pub struct Ora;

impl ReadInstruction for Ora {
    fn instruction(cpu: &mut Cpu, value: u8) {
        let result = cpu.a | value;
        cpu.set_register_with_flags(a_register, result);
    }
}

impl ReadImmediate for Ora {}
impl ReadZeropage for Ora {}
impl ReadZeropageX for Ora {}
impl ReadAbsolute for Ora {}
impl ReadAbsoluteX for Ora {}
impl ReadAbsoluteY for Ora {}
impl ReadIndirectX for Ora {}
impl ReadIndirectY for Ora {}
