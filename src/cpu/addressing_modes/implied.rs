use crate::{
    cpu::{Cpu, executor::Executor},
    memory::Memory,
};

pub trait ImpliedInstruction {
    fn instruction(cpu: &mut Cpu);
}

pub trait Implied: ImpliedInstruction {
    fn implied<M: Memory>(executor: &mut Executor<M>) {
        // dummy read at PC
        let _ = executor.read_cycle(executor.cpu.pc);
        Self::instruction(executor.cpu);
    }
}

impl<I: ImpliedInstruction> Implied for I {}
