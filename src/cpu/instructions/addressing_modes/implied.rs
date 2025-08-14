use crate::{
    cpu::{executor::Executor, Cpu},
    memory::Memory,
};

pub trait Instruction {
    fn instruction(cpu: &mut Cpu);
}

pub trait Implied: Instruction {
    fn implied<M: Memory>(executor: &mut Executor<M>) {
        // dummy read at PC
        let _ = executor.read_cycle(executor.cpu.pc);
        Self::instruction(executor.cpu);
    }
}

impl<I: Instruction> Implied for I {}
