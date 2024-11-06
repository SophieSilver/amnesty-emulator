use super::*;

fn common(cpu: &mut Cpu, value: u8) {
    let new_accumulator = cpu.accumulator & value;
    set_register_with_flags(&mut cpu.accumulator, new_accumulator, &mut cpu.flags);
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
