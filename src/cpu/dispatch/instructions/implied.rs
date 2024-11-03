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

pub fn dex<M: Memory>(cpu: &mut Cpu, memory: &mut M) -> ControlFlow<()> {
    templates::implied(cpu, memory, |cpu| {
        let new_value = cpu.x_index.wrapping_sub(1);
        set_register(&mut cpu.x_index, new_value, &mut cpu.flags)
    })
}

pub fn dey<M: Memory>(cpu: &mut Cpu, memory: &mut M) -> ControlFlow<()> {
    templates::implied(cpu, memory, |cpu| {
        let new_value = cpu.y_index.wrapping_sub(1);
        set_register(&mut cpu.y_index, new_value, &mut cpu.flags)
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
