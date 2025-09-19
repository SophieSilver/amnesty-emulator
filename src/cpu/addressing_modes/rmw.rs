use crate::{
    cpu::{Cpu, executor::Executor},
    memory::Memory,
};

pub trait RmwInstruction {
    fn instruction(cpu: &mut Cpu, value: u8) -> u8;
}

pub trait RmwAccumulator: RmwInstruction {
    fn accumulator<M: Memory>(executor: &mut Executor<M>) {
        // dummy read at PC
        let _ = executor.read_cycle(executor.cpu.pc);
        let value = executor.cpu.a;
        executor.cpu.a = Self::instruction(executor.cpu, value);
    }
}

pub trait RmwZeropage: RmwInstruction {
    fn zeropage<M: Memory>(executor: &mut Executor<M>) {
        let addr = executor.fetch_from_pc_cycle() as u16;
        let value = executor.read_cycle(addr);

        // dummy write back to effective addr
        executor.write_cycle(addr, value);
        let output = Self::instruction(executor.cpu, value);

        executor.write_cycle(addr, output);
    }
}

pub trait RmwZeropageX: RmwInstruction {
    fn zeropage_x<M: Memory>(executor: &mut Executor<M>) {
        let base_addr = executor.fetch_from_pc_cycle();
        // dummy read from addr
        let _ = executor.read_cycle(base_addr as u16);
        // overflow is intentionally not carried into the high byte
        let addr = base_addr.wrapping_add(executor.cpu.x) as u16;
        let value = executor.read_cycle(addr);

        executor.write_cycle(addr, value);
        let output = Self::instruction(executor.cpu, value);

        executor.write_cycle(addr, output);
    }
}

pub trait RmwAbsolute: RmwInstruction {
    fn absolute<M: Memory>(executor: &mut Executor<M>) {
        let addr_low = executor.fetch_from_pc_cycle();
        let addr_high = executor.fetch_from_pc_cycle();

        let addr = (addr_high as u16) << 8 | addr_low as u16;
        let value = executor.read_cycle(addr);

        executor.write_cycle(addr, value);
        let output = Self::instruction(executor.cpu, value);

        executor.write_cycle(addr, output);
    }
}

pub trait RmwAbsoluteX: RmwInstruction {
    fn absolute_x<M: Memory>(executor: &mut Executor<M>) {
        let base_addr_low = executor.fetch_from_pc_cycle();
        let base_addr_high = executor.fetch_from_pc_cycle();

        let (addr_low, carry) = base_addr_low.overflowing_add(executor.cpu.x);
        let addr = (base_addr_high as u16) << 8 | addr_low as u16;

        let _ = executor.read_cycle(addr);
        let addr = addr.wrapping_add((carry as u16) << 8);

        let value = executor.read_cycle(addr);
        executor.write_cycle(addr, value);

        let output = Self::instruction(executor.cpu, value);
        executor.write_cycle(addr, output);
    }
}
