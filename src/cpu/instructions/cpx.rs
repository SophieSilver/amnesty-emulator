use crate::cpu::{Cpu, StatusFlags, addressing_modes::read::*, arithmetic};

pub struct Cpx;

impl ReadInstruction for Cpx {
    fn instruction(cpu: &mut Cpu, value: u8) {
        let (result, carry) = arithmetic::sub_with_carry(cpu.x, value, true);

        cpu.flags.set(StatusFlags::NEGATIVE, (result as i8) < 0);
        cpu.flags.set(StatusFlags::ZERO, result == 0);
        cpu.flags.set(StatusFlags::CARRY, carry);
    }
}

impl ReadImmediate for Cpx {}
impl ReadZeropage for Cpx {}
impl ReadAbsolute for Cpx {}
