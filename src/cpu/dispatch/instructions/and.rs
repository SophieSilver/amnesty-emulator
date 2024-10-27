use super::*;

fn common(cpu: &mut Cpu, value: u8) {
    let new_accumulator = cpu.accumulator & value;
    set_register(&mut cpu.accumulator, new_accumulator, &mut cpu.flags);
}

impl_addressing_modes! {
    common: common,
    preset: read_to_accumulator,
}
