use super::*;

fn common(cpu: &mut Cpu, value: u8) {
    let bit7 = ((value >> 7) & 1) == 1;
    let bit6 = ((value >> 6) & 1) == 1;

    let result = cpu.accumulator & value;
    cpu.flags.set(StatusFlags::ZERO, result == 0);
    cpu.flags.set(StatusFlags::NEGATIVE, bit7);
    cpu.flags.set(StatusFlags::OVERFLOW, bit6);
}

impl_addressing_modes! {
    common: common,
    instruction_type: read,
    modes: [
        zeropage,
        absolute,
    ],
}
