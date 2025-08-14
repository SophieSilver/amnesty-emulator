use crate::cpu::{instructions::addressing_modes::implied, Cpu, StatusFlags};

pub struct Clc;

impl implied::Instruction for Clc {
    fn instruction(cpu: &mut Cpu) {
        cpu.flags.remove(StatusFlags::CARRY);
    }
}

pub struct Cld;

impl implied::Instruction for Cld {
    fn instruction(cpu: &mut Cpu) {
        cpu.flags.remove(StatusFlags::DECIMAL);
    }
}

pub struct Cli;

impl implied::Instruction for Cli {
    fn instruction(cpu: &mut Cpu) {
        cpu.flags.remove(StatusFlags::INTERRUPT_DISABLE);
    }
}

pub struct Clv;

impl implied::Instruction for Clv {
    fn instruction(cpu: &mut Cpu) {
        cpu.flags.remove(StatusFlags::OVERFLOW);
    }
}
