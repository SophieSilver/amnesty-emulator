use crate::cpu::{instructions::addressing_modes::read, register_getters::a_register, Cpu};

pub struct Lda;

impl read::Instruction for Lda {
    fn instruction(cpu: &mut Cpu, value: u8) {
        cpu.set_register_with_flags(a_register, value);
    }
}

impl read::Immediate for Lda {}
impl read::Zeropage for Lda {}
impl read::ZeropageX for Lda {}
impl read::Absolute for Lda {}
impl read::AbsoluteX for Lda {}
impl read::AbsoluteY for Lda {}
impl read::IndirectX for Lda {}
impl read::IndirectY for Lda {}
