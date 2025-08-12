use crate::cpu::{instructions::addressing_modes::read, register_getters::a_register, Cpu};

pub struct Eor;

impl read::Instruction for Eor {
    fn instruction(cpu: &mut Cpu, value: u8) {
        let result = cpu.a ^ value;
        cpu.set_register_with_flags(a_register, result);
    }
}

impl read::Immediate for Eor {}
impl read::Zeropage for Eor {}
impl read::ZeropageX for Eor {}
impl read::Absolute for Eor {}
impl read::AbsoluteX for Eor {}
impl read::AbsoluteY for Eor {}
impl read::IndirectX for Eor {}
impl read::IndirectY for Eor {}
