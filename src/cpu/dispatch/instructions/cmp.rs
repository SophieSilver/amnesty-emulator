use super::*;

fn common(cpu: &mut Cpu, value: u8) {
    let (result, carry) = sub_with_carry(cpu.accumulator, value, true);

    let negative = (result as i8) < 0;
    let zero = result == 0;

    cpu.flags.set(StatusFlags::CARRY, carry);
    cpu.flags.set(StatusFlags::NEGATIVE, negative);
    cpu.flags.set(StatusFlags::ZERO, zero);
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
