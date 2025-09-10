use crate::cpu::{Cpu, addressing_modes::implied::*};

pub struct Txs;

impl ImpliedInstruction for Txs {
    fn instruction(cpu: &mut Cpu) {
        // flags aren't set when writing to SP
        cpu.sp = cpu.x;
    }
}
