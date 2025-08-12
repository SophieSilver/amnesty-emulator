use crate::{
    cpu::{executor::Executor, Cpu},
    memory::Memory,
};

pub trait Instruction {
    fn instruction(cpu: &mut Cpu, value: u8);
}

pub trait Immediate: Instruction {
    fn immediate<M: Memory>(executor: &mut Executor<M>) {
        let value = executor.fetch_from_pc_cycle();
        Self::instruction(executor.cpu, value);
    }
}

pub trait Zeropage: Instruction {
    fn zeropage<M: Memory>(executor: &mut Executor<M>) {
        let addr = executor.fetch_from_pc_cycle() as u16;
        let value = executor.read_cycle(addr);
        Self::instruction(executor.cpu, value);
    }
}

pub trait ZeropageX: Instruction {
    fn zeropage_x<M: Memory>(executor: &mut Executor<M>) {
        zeropage_indexed::<Self>(executor, |cpu| cpu.x);
    }
}

pub trait ZeropageY: Instruction {
    fn zeropage_y<M: Memory>(executor: &mut Executor<M>) {
        zeropage_indexed::<Self>(executor, |cpu| cpu.y);
    }
}

// we take memory and get_index with impls insted of generics
// so that we can call it as zeropage_indexed::<Self>
// instead of zeropage_indexed::<Self, _, _>

fn zeropage_indexed<I: Instruction + ?Sized>(
    executor: &mut Executor<impl Memory>,
    get_index: impl FnOnce(&Cpu) -> u8,
) {
    let base_addr = executor.fetch_from_pc_cycle();
    // dummy read from addr
    let _ = executor.read_cycle(base_addr as u16);
    let addr = base_addr.wrapping_add(get_index(executor.cpu)) as u16;
    let value = executor.read_cycle(addr);
    I::instruction(executor.cpu, value);
}

pub trait Absolute: Instruction {
    fn absolute<M: Memory>(executor: &mut Executor<M>) {
        let addr_low = executor.fetch_from_pc_cycle();
        let addr_high = executor.fetch_from_pc_cycle();

        let addr = (addr_high as u16) << 8 | addr_low as u16;
        let value = executor.read_cycle(addr);
        Self::instruction(executor.cpu, value);
    }
}

pub trait AbsoluteX: Instruction {
    fn absolute_x<M: Memory>(executor: &mut Executor<M>) {
        absolute_indexed::<Self>(executor, |cpu| cpu.x);
    }
}

pub trait AbsoluteY: Instruction {
    fn absolute_y<M: Memory>(executor: &mut Executor<M>) {
        absolute_indexed::<Self>(executor, |cpu| cpu.y);
    }
}

fn absolute_indexed<I: Instruction + ?Sized>(
    executor: &mut Executor<impl Memory>,
    get_index: impl FnOnce(&Cpu) -> u8,
) {
    let base_addr_low = executor.fetch_from_pc_cycle();
    let base_addr_high = executor.fetch_from_pc_cycle();

    let (addr_low, carry) = base_addr_low.overflowing_add(get_index(executor.cpu));

    let addr = (base_addr_high as u16) << 8 | addr_low as u16;
    // if carry is true, addr is wrong, this will be a dummy read
    let value = executor.read_cycle(addr);

    if carry {
        // redo the read
        let corrected_addr = addr.wrapping_add(1 << 8);
        let value = executor.read_cycle(corrected_addr);
        I::instruction(executor.cpu, value);
    } else {
        I::instruction(executor.cpu, value);
    }
}

pub trait IndirectX: Instruction {
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

        let value = executor.read_cycle(addr);
        Self::instruction(executor.cpu, value);
    }
}

pub trait IndirectY: Instruction {
    fn indirect_y<M: Memory>(executor: &mut Executor<M>) {
        let ptr = executor.fetch_from_pc_cycle();

        // note: zero page crossing is intentionally not handled
        let base_addr_low = executor.read_cycle(ptr as u16);
        let base_addr_high = executor.read_cycle(ptr.wrapping_add(1) as u16);

        let (addr_low, carry) = base_addr_low.overflowing_add(executor.cpu.y);
        let addr = (base_addr_high as u16) << 8 | addr_low as u16;
        // might be a dummy read if carry is true and we need to fix up the high byte of the address
        let value = executor.read_cycle(addr);

        if carry {
            let corrected_addr = addr.wrapping_add(1 << 8);
            let value = executor.read_cycle(corrected_addr);
            Self::instruction(executor.cpu, value);
        } else {
            Self::instruction(executor.cpu, value);
        }
    }
}
