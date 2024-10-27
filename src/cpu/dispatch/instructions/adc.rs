use super::*;

fn common(cpu: &mut Cpu, value: u8) {
    let (sum, carry) = add_with_carry(
        cpu.accumulator,
        value,
        cpu.flags.contains(StatusFlags::CARRY),
    );

    let overflow = add_would_overflow(
        cpu.accumulator as i8,
        value as i8,
        cpu.flags.contains(StatusFlags::CARRY),
    );

    cpu.flags.set(StatusFlags::CARRY, carry);
    cpu.flags.set(StatusFlags::OVERFLOW, overflow);

    set_register(&mut cpu.accumulator, sum, &mut cpu.flags);
}

pub fn immediate(cpu: &mut Cpu, memory: &mut MemoryMapping) -> ControlFlow<()> {
    read_immediate(cpu, memory, common)
}

pub fn zeropage(cpu: &mut Cpu, memory: &mut MemoryMapping) -> ControlFlow<()> {
    read_zeropage(cpu, memory, common)
}

pub fn zeropage_x(cpu: &mut Cpu, memory: &mut MemoryMapping) -> ControlFlow<()> {
    read_zeropage_indexed(cpu, memory, get_x_index, common)
}

pub fn absolute(cpu: &mut Cpu, memory: &mut MemoryMapping) -> ControlFlow<()> {
    read_absolute(cpu, memory, common)
}

pub fn absolute_x(cpu: &mut Cpu, memory: &mut MemoryMapping) -> ControlFlow<()> {
    read_absolute_indexed(cpu, memory, get_x_index, common)
}

pub fn absolute_y(cpu: &mut Cpu, memory: &mut MemoryMapping) -> ControlFlow<()> {
    read_absolute_indexed(cpu, memory, get_y_index, common)
}

pub fn indirect_x(cpu: &mut Cpu, memory: &mut MemoryMapping) -> ControlFlow<()> {
    read_indirect_x(cpu, memory, common)
}

pub fn indirect_y(cpu: &mut Cpu, memory: &mut MemoryMapping) -> ControlFlow<()> {
    read_indirect_y(cpu, memory, common)
}
