use super::*;

pub fn clc<M: Memory>(cpu: &mut Cpu, memory: &mut M) -> ControlFlow<()> {
    templates::implied(cpu, memory, |cpu| {
        cpu.flags.remove(StatusFlags::CARRY);
    })
}

pub fn cld<M: Memory>(cpu: &mut Cpu, memory: &mut M) -> ControlFlow<()> {
    templates::implied(cpu, memory, |cpu| {
        cpu.flags.remove(StatusFlags::DECIMAL);
    })
}

pub fn cli<M: Memory>(cpu: &mut Cpu, memory: &mut M) -> ControlFlow<()> {
    templates::implied(cpu, memory, |cpu| {
        cpu.flags.remove(StatusFlags::INTERRUPT_DISABLE);
    })
}

pub fn clv<M: Memory>(cpu: &mut Cpu, memory: &mut M) -> ControlFlow<()> {
    templates::implied(cpu, memory, |cpu| {
        cpu.flags.remove(StatusFlags::OVERFLOW);
    })
}

pub fn sec<M: Memory>(cpu: &mut Cpu, memory: &mut M) -> ControlFlow<()> {
    templates::implied(cpu, memory, |cpu| {
        cpu.flags.insert(StatusFlags::CARRY);
    })
}

pub fn sed<M: Memory>(cpu: &mut Cpu, memory: &mut M) -> ControlFlow<()> {
    templates::implied(cpu, memory, |cpu| {
        cpu.flags.insert(StatusFlags::DECIMAL);
    })
}

pub fn sei<M: Memory>(cpu: &mut Cpu, memory: &mut M) -> ControlFlow<()> {
    templates::implied(cpu, memory, |cpu| {
        cpu.flags.insert(StatusFlags::INTERRUPT_DISABLE);
    })
}

pub fn nop<M: Memory>(cpu: &mut Cpu, memory: &mut M) -> ControlFlow<()> {
    templates::implied(cpu, memory, |_| {})
}
