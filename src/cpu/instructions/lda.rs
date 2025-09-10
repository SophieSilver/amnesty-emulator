use crate::cpu::{Cpu, addressing_modes::read::*, register_getters::a_register};

pub struct Lda;

impl ReadInstruction for Lda {
    fn instruction(cpu: &mut Cpu, value: u8) {
        cpu.set_register_with_flags(a_register, value);
    }
}

impl ReadImmediate for Lda {}
impl ReadZeropage for Lda {}
impl ReadZeropageX for Lda {}
impl ReadAbsolute for Lda {}
impl ReadAbsoluteX for Lda {}
impl ReadAbsoluteY for Lda {}
impl ReadIndirectX for Lda {}
impl ReadIndirectY for Lda {}
