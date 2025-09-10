use crate::cpu::{Cpu, addressing_modes::implied::*, register_getters::y_register};

pub struct Dey;

impl ImpliedInstruction for Dey {
    fn instruction(cpu: &mut Cpu) {
        cpu.set_register_with_flags(y_register, cpu.y.wrapping_sub(1));
    }
}
