use crate::cpu::{Cpu, addressing_modes::implied::*, register_getters::y_register};

pub struct Iny;

impl ImpliedInstruction for Iny {
    fn instruction(cpu: &mut Cpu) {
        cpu.set_register_with_flags(y_register, cpu.y.wrapping_add(1));
    }
}
