use crate::cpu::{instructions::addressing_modes::read::*, register_getters::a_register, Cpu};

pub struct Eor;

impl ReadInstruction for Eor {
    fn instruction(cpu: &mut Cpu, value: u8) {
        let result = cpu.a ^ value;
        cpu.set_register_with_flags(a_register, result);
    }
}

impl ReadImmediate for Eor {}
impl ReadZeropage for Eor {}
impl ReadZeropageX for Eor {}
impl ReadAbsolute for Eor {}
impl ReadAbsoluteX for Eor {}
impl ReadAbsoluteY for Eor {}
impl ReadIndirectX for Eor {}
impl ReadIndirectY for Eor {}
