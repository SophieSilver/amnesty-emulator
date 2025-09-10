use crate::cpu::{Cpu, addressing_modes::read::*, register_getters::a_register};

pub struct And;

impl ReadInstruction for And {
    fn instruction(cpu: &mut Cpu, value: u8) {
        let result = cpu.a & value;
        cpu.set_register_with_flags(a_register, result);
    }
}

impl ReadImmediate for And {}
impl ReadZeropage for And {}
impl ReadZeropageX for And {}
impl ReadAbsolute for And {}
impl ReadAbsoluteX for And {}
impl ReadAbsoluteY for And {}
impl ReadIndirectX for And {}
impl ReadIndirectY for And {}
