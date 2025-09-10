use crate::cpu::{Cpu, addressing_modes::read::*, register_getters::x_register};

pub struct Ldx;

impl ReadInstruction for Ldx {
    fn instruction(cpu: &mut Cpu, value: u8) {
        cpu.set_register_with_flags(x_register, value);
    }
}

impl ReadImmediate for Ldx {}
impl ReadZeropage for Ldx {}
impl ReadZeropageY for Ldx {}
impl ReadAbsolute for Ldx {}
impl ReadAbsoluteY for Ldx {}
