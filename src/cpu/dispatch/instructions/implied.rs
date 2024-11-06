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

pub fn inx<M: Memory>(cpu: &mut Cpu, memory: &mut M) -> ControlFlow<()> {
    templates::implied(cpu, memory, |cpu| {
        let new_value = cpu.x_index.wrapping_add(1);
        set_register(&mut cpu.x_index, new_value, &mut cpu.flags)
    })
}

pub fn iny<M: Memory>(cpu: &mut Cpu, memory: &mut M) -> ControlFlow<()> {
    templates::implied(cpu, memory, |cpu| {
        let new_value = cpu.y_index.wrapping_add(1);
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

pub fn tax<M: Memory>(cpu: &mut Cpu, memory: &mut M) -> ControlFlow<()> {
    templates::implied(cpu, memory, |cpu| {
        set_register(&mut cpu.x_index, cpu.accumulator, &mut cpu.flags);
    })
}

pub fn tay<M: Memory>(cpu: &mut Cpu, memory: &mut M) -> ControlFlow<()> {
    templates::implied(cpu, memory, |cpu| {
        set_register(&mut cpu.y_index, cpu.accumulator, &mut cpu.flags);
    })
}

pub fn tsx<M: Memory>(cpu: &mut Cpu, memory: &mut M) -> ControlFlow<()> {
    templates::implied(cpu, memory, |cpu| {
        set_register(&mut cpu.x_index, cpu.stack_ptr, &mut cpu.flags);
    })
}

pub fn txa<M: Memory>(cpu: &mut Cpu, memory: &mut M) -> ControlFlow<()> {
    templates::implied(cpu, memory, |cpu| {
        set_register(&mut cpu.accumulator, cpu.x_index, &mut cpu.flags);
    })
}

pub fn txs<M: Memory>(cpu: &mut Cpu, memory: &mut M) -> ControlFlow<()> {
    templates::implied(cpu, memory, |cpu| {
        // TXS doesn't set flags
        cpu.stack_ptr = cpu.x_index;
    })
}

pub fn tya<M: Memory>(cpu: &mut Cpu, memory: &mut M) -> ControlFlow<()> {
    templates::implied(cpu, memory, |cpu| {
        set_register(&mut cpu.accumulator, cpu.y_index, &mut cpu.flags);
    })
}
