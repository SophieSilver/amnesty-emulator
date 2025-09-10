use crate::cpu::{
    Cpu, StatusFlags, addressing_modes::read::*, arithmetic, register_getters::a_register,
};

pub struct Sbc;

impl ReadInstruction for Sbc {
    fn instruction(cpu: &mut Cpu, value: u8) {
        let carry = cpu.flags.contains(StatusFlags::CARRY);
        let (result, new_carry) = arithmetic::sub_with_carry(cpu.a, value, carry);
        let overflow = arithmetic::sub_overflows(cpu.a as i8, value as i8, carry);

        cpu.set_register_with_flags(a_register, result);
        cpu.flags.set(StatusFlags::CARRY, new_carry);
        cpu.flags.set(StatusFlags::OVERFLOW, overflow);
    }
}

impl ReadImmediate for Sbc {}
impl ReadZeropage for Sbc {}
impl ReadZeropageX for Sbc {}
impl ReadAbsolute for Sbc {}
impl ReadAbsoluteX for Sbc {}
impl ReadAbsoluteY for Sbc {}
impl ReadIndirectX for Sbc {}
impl ReadIndirectY for Sbc {}
