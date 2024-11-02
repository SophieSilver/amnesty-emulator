use super::*;
use crate::{
    cpu::{Cpu, StatusFlags},
    memory::MemoryMapping,
};
use utils::{possible_pairs_with_carry, TestOpcodePreset as Preset};

fn verify(a: u8, b: u8, carry: bool) -> impl Fn(&mut Cpu, &mut MemoryMapping) {
    // just dump everything into u32 and see what's out of range
    let unsigned_result = a as u32 + b as u32 + carry as u32;
    // first casting to i8 to have it sign extended
    let signed_result = a as i8 as i32 + b as i8 as i32 + carry as i32;

    let should_carry = !(u8::MIN as u32..=u8::MAX as u32).contains(&unsigned_result);
    let should_overflow = !(i8::MIN as i32..=i8::MAX as i32).contains(&signed_result);

    move |cpu, _memory| {
        assert_eq!(
            cpu.accumulator,
            a.wrapping_add(b).wrapping_add(carry as u8),
            "Addition result incorrect"
        );
        assert_eq!(
            cpu.flags.contains(StatusFlags::CARRY),
            should_carry,
            "CARRY flag set incorrectly"
        );
        assert_eq!(
            cpu.flags.contains(StatusFlags::OVERFLOW),
            should_overflow,
            "OVERFLOW flag set incorrectly"
        );
        assert_eq!(
            cpu.flags.contains(StatusFlags::NEGATIVE),
            (cpu.accumulator as i8).is_negative(),
            "NEGATIVE flag set incorrectly"
        );
        assert_eq!(
            cpu.flags.contains(StatusFlags::ZERO),
            cpu.accumulator == 0,
            "ZERO flag set incorrectly"
        );
    }
}

fn prepare(a: u8, carry: bool) -> impl Fn(&mut Cpu) {
    move |cpu| {
        cpu.accumulator = a;
        cpu.flags.set(StatusFlags::CARRY, carry);
    }
}

#[test]
fn immediate() {
    for (a, b, carry) in possible_pairs_with_carry() {
        TestOpcodeOptions::new(OpCode::AdcImmediate, 2, verify(a, b, carry))
            .with_prepare(prepare(a, carry))
            .with_preset(Preset::Immediate(b))
            .test();
    }
}

#[test]
fn zeropage() {
    for (a, b, carry) in possible_pairs_with_carry() {
        TestOpcodeOptions::new(OpCode::AdcZeroPage, 3, verify(a, b, carry))
            .with_prepare(prepare(a, carry))
            .with_preset(Preset::ZeroPage(b))
            .test();
    }
}

#[test]
fn zeropage_x() {
    for (a, b, carry) in possible_pairs_with_carry() {
        TestOpcodeOptions::new(OpCode::AdcZeroPageX, 4, verify(a, b, carry))
            .with_prepare(prepare(a, carry))
            .with_preset(Preset::ZeroPageX(b))
            .test();
    }
}

#[test]
fn zeropage_x_overflow() {
    for (a, b, carry) in possible_pairs_with_carry() {
        TestOpcodeOptions::new(OpCode::AdcZeroPageX, 4, verify(a, b, carry))
            .with_prepare(prepare(a, carry))
            .with_preset(Preset::ZeroPageXOverflow(b))
            .test();
    }
}

#[test]
fn absolute() {
    for (a, b, carry) in possible_pairs_with_carry() {
        TestOpcodeOptions::new(OpCode::AdcAbsolute, 4, verify(a, b, carry))
            .with_prepare(prepare(a, carry))
            .with_preset(Preset::Absolute(b))
            .test();
    }
}

#[test]
fn absolute_x() {
    for (a, b, carry) in possible_pairs_with_carry() {
        TestOpcodeOptions::new(OpCode::AdcAbsoluteX, 4, verify(a, b, carry))
            .with_prepare(prepare(a, carry))
            .with_preset(Preset::AbsoluteX(b))
            .test();
    }
}

#[test]
fn absolute_y() {
    for (a, b, carry) in possible_pairs_with_carry() {
        TestOpcodeOptions::new(OpCode::AdcAbsoluteY, 4, verify(a, b, carry))
            .with_prepare(prepare(a, carry))
            .with_preset(Preset::AbsoluteY(b))
            .test();
    }
}

#[test]
fn absolute_x_overflow() {
    for (a, b, carry) in possible_pairs_with_carry() {
        TestOpcodeOptions::new(OpCode::AdcAbsoluteX, 5, verify(a, b, carry))
            .with_prepare(prepare(a, carry))
            .with_preset(Preset::AbsoluteXOverflow(b))
            .test();
    }
}

#[test]
fn absolute_y_overflow() {
    for (a, b, carry) in possible_pairs_with_carry() {
        TestOpcodeOptions::new(OpCode::AdcAbsoluteY, 5, verify(a, b, carry))
            .with_prepare(prepare(a, carry))
            .with_preset(Preset::AbsoluteYOverflow(b))
            .test();
    }
}

#[test]
fn indirect_x() {
    for (a, b, carry) in possible_pairs_with_carry() {
        TestOpcodeOptions::new(OpCode::AdcIndirectX, 6, verify(a, b, carry))
            .with_prepare(prepare(a, carry))
            .with_preset(Preset::IndirectX(b))
            .test();
    }
}

#[test]
fn indirect_y() {
    for (a, b, carry) in possible_pairs_with_carry() {
        TestOpcodeOptions::new(OpCode::AdcIndirectY, 5, verify(a, b, carry))
            .with_prepare(prepare(a, carry))
            .with_preset(Preset::IndirectY(b))
            .test();
    }
}

#[test]
fn indirect_x_overflow() {
    for (a, b, carry) in possible_pairs_with_carry() {
        TestOpcodeOptions::new(OpCode::AdcIndirectX, 6, verify(a, b, carry))
            .with_prepare(prepare(a, carry))
            .with_preset(Preset::IndirectXOverflow(b))
            .test();
    }
}

#[test]
fn indirect_y_overflow() {
    for (a, b, carry) in possible_pairs_with_carry() {
        TestOpcodeOptions::new(OpCode::AdcIndirectY, 6, verify(a, b, carry))
            .with_prepare(prepare(a, carry))
            .with_preset(Preset::IndirectYOverflow(b))
            .test();
    }
}

#[test]
fn indirect_x_page_split() {
    for (a, b, carry) in possible_pairs_with_carry() {
        TestOpcodeOptions::new(OpCode::AdcIndirectX, 6, verify(a, b, carry))
            .with_prepare(prepare(a, carry))
            .with_preset(Preset::IndirectXPageSplit(b))
            .test();
    }
}

#[test]
fn indirect_y_page_split() {
    for (a, b, carry) in possible_pairs_with_carry() {
        TestOpcodeOptions::new(OpCode::AdcIndirectY, 5, verify(a, b, carry))
            .with_prepare(prepare(a, carry))
            .with_preset(Preset::IndirectYPageSplit(b))
            .test();
    }
}
