use crate::cpu::{instructions::addressing_modes::read, Cpu, StatusFlags};

pub struct Bit;

impl read::Instruction for Bit {
    fn instruction(cpu: &mut Cpu, value: u8) {
        let bit7 = value >> 7 & 1 != 0;
        let bit6 = value >> 6 & 1 != 0;

        let result = cpu.a & value;
        cpu.flags.set(StatusFlags::ZERO, result == 0);
        cpu.flags.set(StatusFlags::NEGATIVE, bit7);
        cpu.flags.set(StatusFlags::OVERFLOW, bit6);
    }
}

impl read::Zeropage for Bit {}
impl read::Absolute for Bit {}
