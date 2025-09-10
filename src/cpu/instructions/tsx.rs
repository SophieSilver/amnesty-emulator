use crate::cpu::{Cpu, addressing_modes::implied::*, register_getters::*};

pub struct Tsx;

impl ImpliedInstruction for Tsx {
    fn instruction(cpu: &mut Cpu) {
        cpu.set_register_with_flags(x_register, cpu.sp);
    }
}
