use crate::cpu::{instructions::addressing_modes::implied::*, Cpu};

pub struct Nop;

impl ImpliedInstruction for Nop {
    fn instruction(_cpu: &mut Cpu) {}
}
