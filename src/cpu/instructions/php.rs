use crate::cpu::{Cpu, StatusFlags, addressing_modes::stack::StackPushInstruction};

pub struct Php;

impl StackPushInstruction for Php {
    fn instruction(cpu: &mut Cpu) -> u8 {
        (cpu.flags | StatusFlags::IGNORED | StatusFlags::BREAK).bits()
    }
}
