use crate::cpu::{Cpu, StatusFlags, addressing_modes::implied::*};

pub struct Sec;

impl ImpliedInstruction for Sec {
    fn instruction(cpu: &mut Cpu) {
        cpu.flags.insert(StatusFlags::CARRY);
    }
}
