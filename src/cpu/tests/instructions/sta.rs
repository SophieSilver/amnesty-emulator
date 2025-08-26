use crate::{
    cpu::{
        instructions::opcode::OpCode,
        tests::{
            utils::{Preset, TestOpcodeOptions},
            TestMemory,
        },
        Cpu,
    },
    memory::Memory,
};

fn verify(value: u8, preset: Preset) -> impl Fn(&mut Cpu, &mut TestMemory) {
    move |cpu, memory| {
        assert_eq!(cpu.a, value, "A register has the wrong value");
        assert_eq!(
            memory.load(preset.final_value_addr()),
            value,
            "wrong value at address"
        );
    }
}

fn prepare(value: u8) -> impl Fn(&mut Cpu) {
    move |cpu| cpu.a = value
}

#[test]
fn zeropage() {
    TestOpcodeOptions::new(OpCode::StaZeropage, 3, verify(0x69, Preset::ZeroPage(0)))
        .with_prepare(prepare(0x69))
        .with_preset(Preset::ZeroPage(0))
        .test();
}

#[test]
fn zeropage_x() {
    TestOpcodeOptions::new(OpCode::StaZeropageX, 4, verify(0x69, Preset::ZeroPageX(0)))
        .with_prepare(prepare(0x69))
        .with_preset(Preset::ZeroPageX(0))
        .test();
}

#[test]
fn absolute() {
    TestOpcodeOptions::new(OpCode::StaAbsolute, 4, verify(0x69, Preset::Absolute(0)))
        .with_prepare(prepare(0x69))
        .with_preset(Preset::Absolute(0))
        .test();
}

#[test]
fn absolute_x() {
    TestOpcodeOptions::new(OpCode::StaAbsoluteX, 5, verify(0x69, Preset::AbsoluteX(0)))
        .with_prepare(prepare(0x69))
        .with_preset(Preset::AbsoluteX(0))
        .test();
}

#[test]
fn absolute_x_overflow() {
    TestOpcodeOptions::new(
        OpCode::StaAbsoluteX,
        5,
        verify(0x69, Preset::AbsoluteXOverflow(0)),
    )
    .with_prepare(prepare(0x69))
    .with_preset(Preset::AbsoluteXOverflow(0))
    .test();
}

#[test]
fn absolute_y() {
    TestOpcodeOptions::new(OpCode::StaAbsoluteY, 5, verify(0x69, Preset::AbsoluteY(0)))
        .with_prepare(prepare(0x69))
        .with_preset(Preset::AbsoluteY(0))
        .test();
}

#[test]
fn absolute_y_overflow() {
    TestOpcodeOptions::new(
        OpCode::StaAbsoluteY,
        5,
        verify(0x69, Preset::AbsoluteYOverflow(0)),
    )
    .with_prepare(prepare(0x69))
    .with_preset(Preset::AbsoluteYOverflow(0))
    .test();
}

#[test]
fn indirect_x() {
    TestOpcodeOptions::new(OpCode::StaIndirectX, 6, verify(0x69, Preset::IndirectX(0)))
        .with_prepare(prepare(0x69))
        .with_preset(Preset::IndirectX(0))
        .test();
}

#[test]
fn indirect_x_overflow() {
    TestOpcodeOptions::new(
        OpCode::StaIndirectX,
        6,
        verify(0x69, Preset::IndirectXOverflow(0)),
    )
    .with_prepare(prepare(0x69))
    .with_preset(Preset::IndirectXOverflow(0))
    .test();
}

#[test]
fn indirect_x_page_split() {
    TestOpcodeOptions::new(
        OpCode::StaIndirectX,
        6,
        verify(0x69, Preset::IndirectXPageSplit(0)),
    )
    .with_prepare(prepare(0x69))
    .with_preset(Preset::IndirectXPageSplit(0))
    .test();
}

#[test]
fn indirect_y() {
    TestOpcodeOptions::new(OpCode::StaIndirectY, 6, verify(0x69, Preset::IndirectY(0)))
        .with_prepare(prepare(0x69))
        .with_preset(Preset::IndirectY(0))
        .test();
}

#[test]
fn indirect_y_overflow() {
    TestOpcodeOptions::new(
        OpCode::StaIndirectY,
        6,
        verify(0x69, Preset::IndirectYOverflow(0)),
    )
    .with_prepare(prepare(0x69))
    .with_preset(Preset::IndirectYOverflow(0))
    .test();
}

#[test]
fn indirect_y_page_split() {
    TestOpcodeOptions::new(
        OpCode::StaIndirectY,
        6,
        verify(0x69, Preset::IndirectYPageSplit(0)),
    )
    .with_prepare(prepare(0x69))
    .with_preset(Preset::IndirectYPageSplit(0))
    .test();
}
