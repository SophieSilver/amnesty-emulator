use crate::cpu::{Cpu, StatusFlags, addressing_modes::implied::*};

pub struct Cli;

impl ImpliedInstruction for Cli {
    fn instruction(cpu: &mut Cpu) {
        cpu.flags.remove(StatusFlags::INTERRUPT_DISABLE);
    }
}
