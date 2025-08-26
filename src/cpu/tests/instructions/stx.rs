use crate::{
    cpu::{
        instructions::opcode::OpCode,
        tests::{
            utils::{possible_byte_pairs, possible_pairs_with_carry, Preset, TestOpcodeOptions},
            TestMemory,
        },
        Cpu, StatusFlags,
    },
    memory::Memory,
};

fn verify(value: u8, preset: Preset) -> impl Fn(&mut Cpu, &mut TestMemory) {
    move |cpu, memory| {
        assert_eq!(cpu.x, value, "X register has the wrong value");
        assert_eq!(
            memory.load(preset.final_value_addr()),
            value,
            "wrong value at address"
        );
    }
}

fn prepare(value: u8) -> impl Fn(&mut Cpu) {
    move |cpu| cpu.x = value
}

#[test]
fn zeropage() {
    TestOpcodeOptions::new(OpCode::StxZeropage, 3, verify(0x69, Preset::ZeroPage(0)))
        .with_prepare(prepare(0x69))
        .with_preset(Preset::ZeroPage(0))
        .test();
}

#[test]
fn zeropage_y() {
    TestOpcodeOptions::new(OpCode::StxZeropageY, 4, verify(0x69, Preset::ZeroPageY(0)))
        .with_prepare(prepare(0x69))
        .with_preset(Preset::ZeroPageY(0))
        .test();
}

#[test]
fn absolute() {
    TestOpcodeOptions::new(OpCode::StxAbsolute, 4, verify(0x69, Preset::Absolute(0)))
        .with_prepare(prepare(0x69))
        .with_preset(Preset::Absolute(0))
        .test();
}
