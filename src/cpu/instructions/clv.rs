use crate::cpu::{Cpu, StatusFlags, addressing_modes::implied::*};

pub struct Clv;

impl ImpliedInstruction for Clv {
    fn instruction(cpu: &mut Cpu) {
        cpu.flags.remove(StatusFlags::OVERFLOW);
    }
}
