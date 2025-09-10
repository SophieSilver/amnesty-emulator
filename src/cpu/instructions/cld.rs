use crate::cpu::{Cpu, StatusFlags, addressing_modes::implied::*};

pub struct Cld;

impl ImpliedInstruction for Cld {
    fn instruction(cpu: &mut Cpu) {
        cpu.flags.remove(StatusFlags::DECIMAL);
    }
}
