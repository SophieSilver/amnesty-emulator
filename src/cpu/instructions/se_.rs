use crate::cpu::{instructions::addressing_modes::implied::*, Cpu, StatusFlags};

pub struct Sec;

impl ImpliedInstruction for Sec {
    fn instruction(cpu: &mut Cpu) {
        cpu.flags.insert(StatusFlags::CARRY);
    }
}

pub struct Sed;

impl ImpliedInstruction for Sed {
    fn instruction(cpu: &mut Cpu) {
        cpu.flags.insert(StatusFlags::DECIMAL);
    }
}

pub struct Sei;

impl ImpliedInstruction for Sei {
    fn instruction(cpu: &mut Cpu) {
        cpu.flags.insert(StatusFlags::INTERRUPT_DISABLE);
    }
}
