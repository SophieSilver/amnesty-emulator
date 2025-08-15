use crate::cpu::{arithmetic, instructions::addressing_modes::read::*, Cpu, StatusFlags};

pub struct Cmp;

impl ReadInstruction for Cmp {
    fn instruction(cpu: &mut Cpu, value: u8) {
        let (result, carry) = arithmetic::sub_with_carry(cpu.a, value, true);

        cpu.flags.set(StatusFlags::NEGATIVE, (result as i8) < 0);
        cpu.flags.set(StatusFlags::ZERO, result == 0);
        cpu.flags.set(StatusFlags::CARRY, carry);
    }
}

impl ReadImmediate for Cmp {}
impl ReadZeropage for Cmp {}
impl ReadZeropageX for Cmp {}
impl ReadAbsolute for Cmp {}
impl ReadAbsoluteX for Cmp {}
impl ReadAbsoluteY for Cmp {}
impl ReadIndirectX for Cmp {}
impl ReadIndirectY for Cmp {}
