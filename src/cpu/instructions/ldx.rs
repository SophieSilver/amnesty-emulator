use crate::cpu::{instructions::addressing_modes::read, register_getters::x_register, Cpu};

pub struct Ldx;

impl read::Instruction for Ldx {
    fn instruction(cpu: &mut Cpu, value: u8) {
        cpu.set_register_with_flags(x_register, value);
    }
}

impl read::Immediate for Ldx {}
impl read::Zeropage for Ldx {}
impl read::ZeropageY for Ldx {}
impl read::Absolute for Ldx {}
impl read::AbsoluteY for Ldx {}
