use crate::{
    cpu::{Cpu, executor::Executor},
    memory::Memory,
};

pub trait StackPushInstruction {
    fn instruction(cpu: &mut Cpu) -> u8;
}

pub trait StackPush: StackPushInstruction {
    fn stack_push<M: Memory>(executor: &mut Executor<M>) {
        let _ = executor.read_cycle(executor.cpu.pc);
        let value = Self::instruction(executor.cpu);
        executor.stack_write(value);
        executor.cpu.sp = executor.cpu.sp.wrapping_sub(1);
    }
}

impl<I: StackPushInstruction> StackPush for I {}
