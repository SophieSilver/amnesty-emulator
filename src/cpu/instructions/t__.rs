use crate::cpu::{instructions::addressing_modes::implied, register_getters::*, Cpu};

pub struct Tax;

impl implied::Instruction for Tax {
    fn instruction(cpu: &mut Cpu) {
        cpu.set_register_with_flags(x_register, cpu.a);
    }
}

pub struct Tay;

impl implied::Instruction for Tay {
    fn instruction(cpu: &mut Cpu) {
        cpu.set_register_with_flags(y_register, cpu.a);
    }
}

pub struct Tsx;

impl implied::Instruction for Tsx {
    fn instruction(cpu: &mut Cpu) {
        cpu.set_register_with_flags(x_register, cpu.sp);
    }
}

pub struct Txa;

impl implied::Instruction for Txa {
    fn instruction(cpu: &mut Cpu) {
        cpu.set_register_with_flags(a_register, cpu.x);
    }
}

pub struct Txs;

impl implied::Instruction for Txs {
    fn instruction(cpu: &mut Cpu) {
        // flags aren't set when writing to SPs
        cpu.sp = cpu.x;
    }
}

pub struct Tya;

impl implied::Instruction for Tya {
    fn instruction(cpu: &mut Cpu) {
        cpu.set_register_with_flags(a_register, cpu.y);
    }
}
