use crate::cpu::{Cpu, addressing_modes::stack::StackPushInstruction};

pub struct Pha;

impl StackPushInstruction for Pha {
    fn instruction(cpu: &mut Cpu) -> u8 {
        cpu.a
    }
}
