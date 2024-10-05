use crate::{cpu::{self, CpuState}, memory::MemoryMapping};

pub fn load_operand1(cpu_state: &mut CpuState, memory: &mut MemoryMapping) {
    cpu_state.operand1 = memory.load(cpu_state.pc + 1);
}

pub fn load_operand2(cpu_state: &mut CpuState, memory: &mut MemoryMapping) {
    cpu_state.operand1 = memory.load(cpu_state.pc + 2);
}

pub fn increment_program_counter(cpu_state: &mut CpuState, amount: u16) {
    cpu_state.pc += amount;
}
