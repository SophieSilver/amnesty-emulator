use crate::cpu::{arithmetic, instructions::addressing_modes::read, Cpu, StatusFlags};

pub struct Cmp;

impl read::Instruction for Cmp {
    fn instruction(cpu: &mut Cpu, value: u8) {
        let (result, carry) = arithmetic::sub_with_carry(cpu.a, value, true);

        cpu.flags.set(StatusFlags::NEGATIVE, (result as i8) < 0);
        cpu.flags.set(StatusFlags::ZERO, result == 0);
        cpu.flags.set(StatusFlags::CARRY, carry);
    }
}

impl read::Immediate for Cmp {}
impl read::Zeropage for Cmp {}
impl read::ZeropageX for Cmp {}
impl read::Absolute for Cmp {}
impl read::AbsoluteX for Cmp {}
impl read::AbsoluteY for Cmp {}
impl read::IndirectX for Cmp {}
impl read::IndirectY for Cmp {}
