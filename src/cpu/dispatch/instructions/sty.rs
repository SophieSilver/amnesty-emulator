use super::*;

fn common(cpu: &Cpu) -> u8 {
    cpu.y_index
}

impl_addressing_modes! {
    common: common,
    instruction_type: write,
    modes: [
        zeropage,
        zeropage_x,
        absolute
    ],
}
