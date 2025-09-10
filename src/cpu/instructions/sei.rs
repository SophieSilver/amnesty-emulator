use crate::cpu::{Cpu, StatusFlags, addressing_modes::implied::*};

pub struct Sei;

impl ImpliedInstruction for Sei {
    fn instruction(cpu: &mut Cpu) {
        cpu.flags.insert(StatusFlags::INTERRUPT_DISABLE);
    }
}
