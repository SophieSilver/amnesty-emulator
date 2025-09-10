use crate::cpu::{Cpu, addressing_modes::implied::*, register_getters::*};

pub struct Tya;

impl ImpliedInstruction for Tya {
    fn instruction(cpu: &mut Cpu) {
        cpu.set_register_with_flags(a_register, cpu.y);
    }
}
