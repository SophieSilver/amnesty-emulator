use super::*;

fn common(cpu: &mut Cpu, value: u8) {
    set_register(&mut cpu.accumulator, value, &mut cpu.flags);
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
