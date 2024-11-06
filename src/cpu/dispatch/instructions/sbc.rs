use super::*;

fn common(cpu: &mut Cpu, value: u8) {
    let (result, carry) = sub_with_carry(
        cpu.accumulator,
        value,
        cpu.flags.contains(StatusFlags::CARRY),
    );

    let overflow = sub_would_overflow(
        cpu.accumulator as i8,
        value as i8,
        cpu.flags.contains(StatusFlags::CARRY),
    );

    cpu.flags.set(StatusFlags::CARRY, carry);
    cpu.flags.set(StatusFlags::OVERFLOW, overflow);

    set_register_with_flags(&mut cpu.accumulator, result, &mut cpu.flags);
}

impl_addressing_modes! {
    common: common,
    instruction_type: read,
    modes: [
        immediate,
        zeropage,
        zeropage_x,
        absolute,
        absolute_x,
        absolute_y,
        indirect_x,
        indirect_y,
    ],
}
