use super::*;

fn common(cpu: &mut Cpu, value: u8) {
    let new_accomulator = cpu.accumulator | value;
    set_register(&mut cpu.accumulator, new_accomulator, &mut cpu.flags);
}

impl_addressing_modes! {
    common: common,
    preset: read_to_accumulator,
}
