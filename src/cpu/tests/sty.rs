use utils::Preset;

use crate::cpu::Cpu;

use super::*;

fn verify(value: u8, preset: Preset) -> impl Fn(&mut Cpu, &mut TestMemory) {
    move |cpu, memory| {
        assert_eq!(cpu.y, value, "Y register has the wrong value");
        assert_eq!(
            memory.load(preset.final_value_addr()),
            value,
            "wrong value at address"
        );
    }
}

fn prepare(value: u8) -> impl Fn(&mut Cpu) {
    move |cpu| cpu.y = value
}

#[test]
fn zeropage() {
    TestOpcodeOptions::new(OpCode::StyZeropage, 3, verify(0x69, Preset::ZeroPage(0)))
        .with_prepare(prepare(0x69))
        .with_preset(Preset::ZeroPage(0))
        .test();
}

#[test]
fn zeropage_x() {
    TestOpcodeOptions::new(OpCode::StyZeropageX, 4, verify(0x69, Preset::ZeroPageX(0)))
        .with_prepare(prepare(0x69))
        .with_preset(Preset::ZeroPageX(0))
        .test();
}

#[test]
fn absolute() {
    TestOpcodeOptions::new(OpCode::StyAbsolute, 4, verify(0x69, Preset::Absolute(0)))
        .with_prepare(prepare(0x69))
        .with_preset(Preset::Absolute(0))
        .test();
}
