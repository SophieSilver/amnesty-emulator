use crate::{
    cpu::{Cpu, executor::Executor},
    memory::Memory,
};

pub trait WriteInstruction {
    fn instruction(cpu: &Cpu) -> u8;
}

pub trait WriteZeropage: WriteInstruction {
    fn zeropage<M: Memory>(executor: &mut Executor<M>) {
        let addr = executor.fetch_from_pc_cycle() as u16;
        let value = Self::instruction(executor.cpu);
        executor.write_cycle(addr, value);
    }
}

pub trait WriteZeropageX: WriteInstruction {
    fn zeropage_x<M: Memory>(executor: &mut Executor<M>) {
        zeropage_indexed::<Self>(executor, |cpu| cpu.x);
    }
}

pub trait WriteZeropageY: WriteInstruction {
    fn zeropage_y<M: Memory>(executor: &mut Executor<M>) {
        zeropage_indexed::<Self>(executor, |cpu| cpu.y);
    }
}

fn zeropage_indexed<I: WriteInstruction + ?Sized>(
    executor: &mut Executor<impl Memory>,
    get_index: impl FnOnce(&Cpu) -> u8,
) {
    let base_addr = executor.fetch_from_pc_cycle();
    // dummy read from addr
    let _ = executor.read_cycle(base_addr as u16);
    let addr = base_addr.wrapping_add(get_index(executor.cpu)) as u16;
    let value = I::instruction(executor.cpu);
    executor.write_cycle(addr, value);
}

pub trait WriteAbsolute: WriteInstruction {
    fn absolute<M: Memory>(executor: &mut Executor<M>) {
        let addr_low = executor.fetch_from_pc_cycle();
        let addr_high = executor.fetch_from_pc_cycle();

        let addr = (addr_high as u16) << 8 | addr_low as u16;
        let value = Self::instruction(executor.cpu);
        executor.write_cycle(addr, value);
    }
}

pub trait WriteAbsoluteX: WriteInstruction {
    fn absolute_x<M: Memory>(executor: &mut Executor<M>) {
        absolute_indexed::<Self>(executor, |cpu| cpu.x);
    }
}

pub trait WriteAbsoluteY: WriteInstruction {
    fn absolute_y<M: Memory>(executor: &mut Executor<M>) {
        absolute_indexed::<Self>(executor, |cpu| cpu.y);
    }
}

fn absolute_indexed<I: WriteInstruction + ?Sized>(
    executor: &mut Executor<impl Memory>,
    get_index: impl FnOnce(&Cpu) -> u8,
) {
    let base_addr_low = executor.fetch_from_pc_cycle();
    let base_addr_high = executor.fetch_from_pc_cycle();

    let (addr_low, carry) = base_addr_low.overflowing_add(get_index(executor.cpu));

    let addr = (base_addr_high as u16) << 8 | addr_low as u16;
    // dummy read from an address that might be wrong
    let _ = executor.read_cycle(addr);

    let addr = if carry {
        addr.wrapping_add(1 << 8)
    } else {
        addr
    };

    let value = I::instruction(executor.cpu);
    executor.write_cycle(addr, value);
}

pub trait WriteIndirectX: WriteInstruction {
    fn indirect_x<M: Memory>(executor: &mut Executor<M>) {
        let base_ptr = executor.fetch_from_pc_cycle();
        // dummy read
        let _ = executor.read_cycle(base_ptr as u16);
        let ptr = base_ptr.wrapping_add(executor.cpu.x);

        // note: intentionally first adding and then extending to 16 bit
        // overflowing page zero wraps to the beginning of page zero, not to page 1
        let addr_low = executor.read_cycle(ptr as u16);
        let addr_high = executor.read_cycle(ptr.wrapping_add(1) as u16);

        let addr = (addr_high as u16) << 8 | addr_low as u16;
        let value = Self::instruction(executor.cpu);
        executor.write_cycle(addr, value);
    }
}

pub trait WriteIndirectY: WriteInstruction {
    fn indirect_y<M: Memory>(executor: &mut Executor<M>) {
        let ptr = executor.fetch_from_pc_cycle();

        // note: zero page crossing is intentionally not handled
        let base_addr_low = executor.read_cycle(ptr as u16);
        let base_addr_high = executor.read_cycle(ptr.wrapping_add(1) as u16);

        let (addr_low, carry) = base_addr_low.overflowing_add(executor.cpu.y);
        let addr = (base_addr_high as u16) << 8 | addr_low as u16;
        // might be a dummy read if carry is true and we need to fix up the high byte of the address
        let _ = executor.read_cycle(addr);

        let addr = if carry {
            addr.wrapping_add(1 << 8)
        } else {
            addr
        };

        let value = Self::instruction(executor.cpu);
        executor.write_cycle(addr, value);
    }
}
