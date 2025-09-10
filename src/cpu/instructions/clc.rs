use crate::cpu::{Cpu, StatusFlags, addressing_modes::implied::*};

pub struct Clc;

impl ImpliedInstruction for Clc {
    fn instruction(cpu: &mut Cpu) {
        cpu.flags.remove(StatusFlags::CARRY);
    }
}
