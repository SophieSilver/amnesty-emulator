use crate::cpu::{Cpu, addressing_modes::implied::*};

pub struct Nop;

impl ImpliedInstruction for Nop {
    fn instruction(_cpu: &mut Cpu) {}
}
