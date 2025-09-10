use crate::cpu::{Cpu, addressing_modes::implied::*, register_getters::*};

pub struct Tay;

impl ImpliedInstruction for Tay {
    fn instruction(cpu: &mut Cpu) {
        cpu.set_register_with_flags(y_register, cpu.a);
    }
}
