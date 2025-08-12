use crate::cpu::{
    arithmetic, instructions::addressing_modes::read, register_getters::a_register, Cpu,
    StatusFlags,
};

pub struct Adc;

impl read::Instruction for Adc {
    fn instruction(cpu: &mut Cpu, value: u8) {
        let carry = cpu.flags.contains(StatusFlags::CARRY);
        let (result, new_carry) = arithmetic::add_with_carry(cpu.a, value, carry);
        let overflow = arithmetic::add_overflows(cpu.a as i8, value as i8, carry);

        cpu.set_register_with_flags(a_register, result);
        cpu.flags.set(StatusFlags::CARRY, new_carry);
        cpu.flags.set(StatusFlags::OVERFLOW, overflow);
    }
}

impl read::Immediate for Adc {}
impl read::Zeropage for Adc {}
impl read::ZeropageX for Adc {}
impl read::Absolute for Adc {}
impl read::AbsoluteX for Adc {}
impl read::AbsoluteY for Adc {}
impl read::IndirectX for Adc {}
impl read::IndirectY for Adc {}
