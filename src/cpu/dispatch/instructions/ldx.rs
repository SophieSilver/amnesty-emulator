use super::*;

fn common(cpu: &mut Cpu, value: u8) {
    set_register(&mut cpu.x_index, value, &mut cpu.flags);
}

impl_addressing_modes! {
    common: common,
    instruction_type: read,
    modes: [
        immediate,
        zeropage,
        zeropage_y,
        absolute,
        absolute_y
    ]
}
