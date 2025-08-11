use super::*;

pub fn zeropage<M: Memory, F: FnOnce(&Cpu) -> u8>(
    cpu: &mut Cpu,
    memory: &mut M,
    f: F,
) -> ControlFlow<()> //
{
    match cpu.current_instruction_cycle {
        1 => {
            cpu.effective_address = fetch_from_pc(cpu, memory) as u16;
        }
        2 => {
            memory.store(cpu.effective_address, f(cpu));

            return ControlFlow::Break(());
        }

        _ => unreachable!(),
    }

    ControlFlow::Continue(())
}

pub fn zeropage_indexed<M, F, I>(
    cpu: &mut Cpu,
    memory: &mut M,
    get_index: I,
    f: F,
) -> ControlFlow<()>
where
    M: Memory,
    F: FnOnce(&Cpu) -> u8,
    I: FnOnce(&Cpu) -> u8,
{
    match cpu.current_instruction_cycle {
        1 => {
            cpu.effective_address = fetch_from_pc(cpu, memory) as u16;
        }
        2 => {
            let _ = memory.load(cpu.effective_address);
            cpu.effective_address =
                (cpu.effective_address as u8).wrapping_add(get_index(cpu)) as u16;
        }
        3 => {
            memory.store(cpu.effective_address, f(cpu));
            return ControlFlow::Break(());
        }

        _ => unreachable!(),
    }

    ControlFlow::Continue(())
}

pub fn zeropage_x<M: Memory, F: FnOnce(&Cpu) -> u8>(
    cpu: &mut Cpu,
    memory: &mut M,
    f: F,
) -> ControlFlow<()> //
{
    zeropage_indexed(cpu, memory, get_x_index, f)
}

pub fn zeropage_y<M: Memory, F: FnOnce(&Cpu) -> u8>(
    cpu: &mut Cpu,
    memory: &mut M,
    f: F,
) -> ControlFlow<()> //
{
    zeropage_indexed(cpu, memory, get_y_index, f)
}

pub fn absolute<M: Memory, F: FnOnce(&Cpu) -> u8>(
    cpu: &mut Cpu,
    memory: &mut M,
    f: F,
) -> ControlFlow<()> //
{
    match cpu.current_instruction_cycle {
        1 => {
            cpu.effective_address = fetch_from_pc(cpu, memory) as u16;
        }
        2 => {
            cpu.effective_address |= (fetch_from_pc(cpu, memory) as u16) << 8;
        }
        3 => {
            memory.store(cpu.effective_address, f(cpu));
            return ControlFlow::Break(());
        }
        _ => unreachable!(),
    };

    ControlFlow::Continue(())
}

pub fn absolute_indexed<M, F, I>(
    cpu: &mut Cpu,
    memory: &mut M,
    get_index: I,
    f: F,
) -> ControlFlow<()>
where
    M: Memory,
    F: FnOnce(&Cpu) -> u8,
    I: FnOnce(&Cpu) -> u8,
{
    match cpu.current_instruction_cycle {
        1 => {
            cpu.effective_address = fetch_from_pc(cpu, memory) as u16;
        }
        2 => {
            let address_high_byte = fetch_from_pc(cpu, memory);
            let (addresss_low_byte, carry) =
                (cpu.effective_address as u8).overflowing_add(get_index(cpu));

            cpu.effective_address = (address_high_byte as u16) << 8 | addresss_low_byte as u16;
            cpu.internal_flags
                .set(InternalFlags::EFFECTIVE_ADDR_CARRY, carry);
        }
        3 => {
            // dummy read
            let _ = memory.load(cpu.effective_address);

            let carry = cpu
                .internal_flags
                .contains(InternalFlags::EFFECTIVE_ADDR_CARRY);
            cpu.effective_address = cpu.effective_address.wrapping_add((carry as u16) << 8);
        }
        4 => {
            memory.store(cpu.effective_address, f(cpu));
            return ControlFlow::Break(());
        }

        _ => unreachable!(),
    }

    ControlFlow::Continue(())
}

pub fn absolute_x<M: Memory, F: FnOnce(&Cpu) -> u8>(
    cpu: &mut Cpu,
    memory: &mut M,
    f: F,
) -> ControlFlow<()> //
{
    absolute_indexed(cpu, memory, get_x_index, f)
}

pub fn absolute_y<M: Memory, F: FnOnce(&Cpu) -> u8>(
    cpu: &mut Cpu,
    memory: &mut M,
    f: F,
) -> ControlFlow<()> //
{
    absolute_indexed(cpu, memory, get_y_index, f)
}

pub fn indirect_x<M: Memory, F: FnOnce(&Cpu) -> u8>(
    cpu: &mut Cpu,
    memory: &mut M,
    f: F,
) -> ControlFlow<()> //
{
    match cpu.current_instruction_cycle {
        1 => {
            cpu.pointer_address = fetch_from_pc(cpu, memory);
        }
        2 => {
            // dummy read :3
            let _ = memory.load(cpu.pointer_address as u16);
            cpu.pointer_address = cpu.pointer_address.wrapping_add(cpu.x_index);
        }
        3 => {
            cpu.effective_address = memory.load(cpu.pointer_address as u16) as u16;
        }
        4 => {
            cpu.effective_address |=
                (memory.load(cpu.pointer_address.wrapping_add(1) as u16) as u16) << 8;
        }
        5 => {
            memory.store(cpu.effective_address, f(cpu));
            return ControlFlow::Break(());
        }

        _ => unreachable!(),
    }

    ControlFlow::Continue(())
}

pub fn indirect_y<M: Memory, F: FnOnce(&Cpu) -> u8>(
    cpu: &mut Cpu,
    memory: &mut M,
    f: F,
) -> ControlFlow<()> //
{
    match cpu.current_instruction_cycle {
        1 => {
            cpu.pointer_address = fetch_from_pc(cpu, memory);
        }
        2 => {
            cpu.effective_address = memory.load(cpu.pointer_address as u16) as u16;
        }
        3 => {
            let address_high_byte = memory.load(cpu.pointer_address.wrapping_add(1) as u16);
            let (address_low_byte, carry) =
                (cpu.effective_address as u8).overflowing_add(cpu.y_index);

            cpu.effective_address = (address_high_byte as u16) << 8 | address_low_byte as u16;
            cpu.internal_flags
                .set(InternalFlags::EFFECTIVE_ADDR_CARRY, carry);
        }
        4 => {
            let _ = memory.load(cpu.effective_address);

            let carry = cpu
                .internal_flags
                .contains(InternalFlags::EFFECTIVE_ADDR_CARRY);
            cpu.effective_address = cpu.effective_address.wrapping_add((carry as u16) << 8);
        }
        5 => {
            memory.store(cpu.effective_address, f(cpu));
            return ControlFlow::Break(());
        }
        _ => unreachable!(),
    }

    ControlFlow::Continue(())
}
