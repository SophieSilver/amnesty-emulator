use crate::cpu::{instructions::addressing_modes::implied, Cpu};

pub struct Nop;

impl implied::Instruction for Nop {
    fn instruction(_cpu: &mut Cpu) {}
}
