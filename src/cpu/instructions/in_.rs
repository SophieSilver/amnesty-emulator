use crate::cpu::{
    instructions::addressing_modes::implied,
    register_getters::{x_register, y_register},
    Cpu,
};

pub struct Inx;

impl implied::Instruction for Inx {
    fn instruction(cpu: &mut Cpu) {
        cpu.set_register_with_flags(x_register, cpu.x.wrapping_add(1));
    }
}

pub struct Iny;

impl implied::Instruction for Iny {
    fn instruction(cpu: &mut Cpu) {
        cpu.set_register_with_flags(y_register, cpu.y.wrapping_add(1));
    }
}
