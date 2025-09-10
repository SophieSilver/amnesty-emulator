use crate::cpu::{Cpu, StatusFlags, addressing_modes::read::*, arithmetic};

pub struct Cpy;

impl ReadInstruction for Cpy {
    fn instruction(cpu: &mut Cpu, value: u8) {
        let (result, carry) = arithmetic::sub_with_carry(cpu.y, value, true);

        cpu.flags.set(StatusFlags::NEGATIVE, (result as i8) < 0);
        cpu.flags.set(StatusFlags::ZERO, result == 0);
        cpu.flags.set(StatusFlags::CARRY, carry);
    }
}

impl ReadImmediate for Cpy {}
impl ReadZeropage for Cpy {}
impl ReadAbsolute for Cpy {}
