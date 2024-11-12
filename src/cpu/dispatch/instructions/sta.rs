use super::*;

fn common(cpu: &Cpu) -> u8 {
    cpu.accumulator
}

impl_addressing_modes! {
    common: common,
    instruction_type: write,
    modes: [
        zeropage,
        zeropage_x,
        absolute,
        absolute_x,
        absolute_y,
        indirect_x,
        indirect_y,
    ],
}
