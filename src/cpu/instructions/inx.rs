use crate::cpu::{Cpu, addressing_modes::implied::*, register_getters::x_register};

pub struct Inx;

impl ImpliedInstruction for Inx {
    fn instruction(cpu: &mut Cpu) {
        cpu.set_register_with_flags(x_register, cpu.x.wrapping_add(1));
    }
}
