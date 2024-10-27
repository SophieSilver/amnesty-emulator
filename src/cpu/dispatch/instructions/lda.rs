use super::*;

fn common(cpu: &mut Cpu, value: u8) {
    set_register(&mut cpu.accumulator, value, &mut cpu.flags);
}

impl_addressing_modes! {
    common: common,
    preset: read_to_accumulator,
}
