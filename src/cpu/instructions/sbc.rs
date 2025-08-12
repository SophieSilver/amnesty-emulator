use crate::cpu::{
    arithmetic, instructions::addressing_modes::read, register_getters::a_register, Cpu,
    StatusFlags,
};

pub struct Sbc;

impl read::Instruction for Sbc {
    fn instruction(cpu: &mut Cpu, value: u8) {
        let carry = cpu.flags.contains(StatusFlags::CARRY);
        let (result, new_carry) = arithmetic::sub_with_carry(cpu.a, value, carry);
        let overflow = arithmetic::sub_overflows(cpu.a as i8, value as i8, carry);

        cpu.set_register_with_flags(a_register, result);
        cpu.flags.set(StatusFlags::CARRY, new_carry);
        cpu.flags.set(StatusFlags::OVERFLOW, overflow);
    }
}

impl read::Immediate for Sbc {}
impl read::Zeropage for Sbc {}
impl read::ZeropageX for Sbc {}
impl read::Absolute for Sbc {}
impl read::AbsoluteX for Sbc {}
impl read::AbsoluteY for Sbc {}
impl read::IndirectX for Sbc {}
impl read::IndirectY for Sbc {}
