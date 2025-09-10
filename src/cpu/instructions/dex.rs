use crate::cpu::{Cpu, addressing_modes::implied::*, register_getters::x_register};

pub struct Dex;

impl ImpliedInstruction for Dex {
    fn instruction(cpu: &mut Cpu) {
        cpu.set_register_with_flags(x_register, cpu.x.wrapping_sub(1));
    }
}
