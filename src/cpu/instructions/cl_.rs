use crate::cpu::{instructions::addressing_modes::implied::*, Cpu, StatusFlags};

pub struct Clc;

impl ImpliedInstruction for Clc {
    fn instruction(cpu: &mut Cpu) {
        cpu.flags.remove(StatusFlags::CARRY);
    }
}

pub struct Cld;

impl ImpliedInstruction for Cld {
    fn instruction(cpu: &mut Cpu) {
        cpu.flags.remove(StatusFlags::DECIMAL);
    }
}

pub struct Cli;

impl ImpliedInstruction for Cli {
    fn instruction(cpu: &mut Cpu) {
        cpu.flags.remove(StatusFlags::INTERRUPT_DISABLE);
    }
}

pub struct Clv;

impl ImpliedInstruction for Clv {
    fn instruction(cpu: &mut Cpu) {
        cpu.flags.remove(StatusFlags::OVERFLOW);
    }
}
