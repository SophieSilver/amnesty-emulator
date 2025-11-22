use crate::cpu::addressing_modes::stack::StackPushInstruction;

pub struct Pha;

impl StackPushInstruction for Pha {
    fn instruction(cpu: &mut crate::cpu::Cpu) -> u8 {
        cpu.a
    }
}
