use crate::cpu::{instructions::addressing_modes::implied::*, register_getters::*, Cpu};

pub struct Tax;

impl ImpliedInstruction for Tax {
    fn instruction(cpu: &mut Cpu) {
        cpu.set_register_with_flags(x_register, cpu.a);
    }
}

pub struct Tay;

impl ImpliedInstruction for Tay {
    fn instruction(cpu: &mut Cpu) {
        cpu.set_register_with_flags(y_register, cpu.a);
    }
}

pub struct Tsx;

impl ImpliedInstruction for Tsx {
    fn instruction(cpu: &mut Cpu) {
        cpu.set_register_with_flags(x_register, cpu.sp);
    }
}

pub struct Txa;

impl ImpliedInstruction for Txa {
    fn instruction(cpu: &mut Cpu) {
        cpu.set_register_with_flags(a_register, cpu.x);
    }
}

pub struct Txs;

impl ImpliedInstruction for Txs {
    fn instruction(cpu: &mut Cpu) {
        // flags aren't set when writing to SPs
        cpu.sp = cpu.x;
    }
}

pub struct Tya;

impl ImpliedInstruction for Tya {
    fn instruction(cpu: &mut Cpu) {
        cpu.set_register_with_flags(a_register, cpu.y);
    }
}
