use crate::cpu::{Cpu, StatusFlags, addressing_modes::implied::*};

pub struct Sed;

impl ImpliedInstruction for Sed {
    fn instruction(cpu: &mut Cpu) {
        cpu.flags.insert(StatusFlags::DECIMAL);
    }
}
