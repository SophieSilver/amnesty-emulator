use crate::cpu::{Cpu, addressing_modes::implied::*, register_getters::*};

pub struct Txa;

impl ImpliedInstruction for Txa {
    fn instruction(cpu: &mut Cpu) {
        cpu.set_register_with_flags(a_register, cpu.x);
    }
}
