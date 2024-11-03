use super::*;

pub fn clc(cpu: &mut Cpu, memory: &mut MemoryMapping) -> ControlFlow<()> {
    templates::implied(cpu, memory, |cpu| {
        cpu.flags.remove(StatusFlags::CARRY);
    })
}

pub fn cld(cpu: &mut Cpu, memory: &mut MemoryMapping) -> ControlFlow<()> {
    templates::implied(cpu, memory, |cpu| {
        cpu.flags.remove(StatusFlags::DECIMAL);
    })
}

pub fn cli(cpu: &mut Cpu, memory: &mut MemoryMapping) -> ControlFlow<()> {
    templates::implied(cpu, memory, |cpu| {
        cpu.flags.remove(StatusFlags::INTERRUPT_DISABLE);
    })
}

pub fn clv(cpu: &mut Cpu, memory: &mut MemoryMapping) -> ControlFlow<()> {
    templates::implied(cpu, memory, |cpu| {
        cpu.flags.remove(StatusFlags::OVERFLOW);
    })
}

pub fn sec(cpu: &mut Cpu, memory: &mut MemoryMapping) -> ControlFlow<()> {
    templates::implied(cpu, memory, |cpu| {
        cpu.flags.insert(StatusFlags::CARRY);
    })
}

pub fn sed(cpu: &mut Cpu, memory: &mut MemoryMapping) -> ControlFlow<()> {
    templates::implied(cpu, memory, |cpu| {
        cpu.flags.insert(StatusFlags::DECIMAL);
    })
}

pub fn sei(cpu: &mut Cpu, memory: &mut MemoryMapping) -> ControlFlow<()> {
    templates::implied(cpu, memory, |cpu| {
        cpu.flags.insert(StatusFlags::INTERRUPT_DISABLE);
    })
}

pub fn nop(cpu: &mut Cpu, memory: &mut MemoryMapping) -> ControlFlow<()> {
    templates::implied(cpu, memory, |_| {})
}
