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

    set_register(&mut cpu.accumulator, result, &mut cpu.flags);
}

impl_addressing_modes! {
    common: common,
    preset: read_to_accumulator,
}
