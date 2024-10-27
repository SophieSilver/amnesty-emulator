use super::*;

fn common(cpu: &mut Cpu, value: u8) {
    set_register(&mut cpu.x_index, value, &mut cpu.flags);
}

pub fn immediate(cpu: &mut Cpu, memory: &mut MemoryMapping) -> ControlFlow<()> {
    read_immediate(cpu, memory, common)
}

pub fn zeropage(cpu: &mut Cpu, memory: &mut MemoryMapping) -> ControlFlow<()> {
    read_zeropage(cpu, memory, common)
}

pub fn zeropage_y(cpu: &mut Cpu, memory: &mut MemoryMapping) -> ControlFlow<()> {
    read_zeropage_indexed(cpu, memory, get_y_index, common)
}

pub fn absolute(cpu: &mut Cpu, memory: &mut MemoryMapping) -> ControlFlow<()> {
    read_absolute(cpu, memory, common)
}

pub fn absolute_y(cpu: &mut Cpu, memory: &mut MemoryMapping) -> ControlFlow<()> {
    read_absolute_indexed(cpu, memory, get_y_index, common)
}
