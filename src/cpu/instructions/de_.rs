use crate::cpu::{
    instructions::addressing_modes::implied,
    register_getters::{x_register, y_register},
    Cpu,
};

pub struct Dex;

impl implied::ImpliedInstruction for Dex {
    fn instruction(cpu: &mut Cpu) {
        cpu.set_register_with_flags(x_register, cpu.x.wrapping_sub(1));
    }
}

pub struct Dey;

impl implied::ImpliedInstruction for Dey {
    fn instruction(cpu: &mut Cpu) {
        cpu.set_register_with_flags(y_register, cpu.y.wrapping_sub(1));
    }
}
