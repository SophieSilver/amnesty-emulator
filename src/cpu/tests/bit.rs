use utils::{possible_byte_pairs, TestOpcodePreset as Preset};

use crate::cpu::{Cpu, StatusFlags};

use super::*;

fn verify(a: u8, b: u8) -> impl Fn(&mut Cpu, &mut TestMemory) {
    let result = a & b;
    let zero = result == 0;
    let negative_flag = (b as i8) < 0;
    let overflow_flag = (b >> 6) & 1 == 1;

    move |cpu, _memory| {
        assert_eq!(
            cpu.flags.contains(StatusFlags::ZERO),
            zero,
            "ZERO flag set incorrectly"
        );
        assert_eq!(
            cpu.flags.contains(StatusFlags::NEGATIVE),
            negative_flag,
            "NEGATIVE flag set incorrectly"
        );
        assert_eq!(
            cpu.flags.contains(StatusFlags::OVERFLOW),
            overflow_flag,
            "OVERFLOW flag set incorrectly"
        );
    }
}

#[test]
fn zeropage() {
    for (a, b) in possible_byte_pairs() {
        TestOpcodeOptions::new(OpCode::BitZeroPage, 3, verify(a, b))
            .with_prepare(|cpu| cpu.accumulator = a)
            .with_preset(Preset::ZeroPage(b))
            .test();
    }
}

#[test]
fn absolute() {
    for (a, b) in possible_byte_pairs() {
        TestOpcodeOptions::new(OpCode::BitAbsolute, 4, verify(a, b))
            .with_prepare(|cpu| cpu.accumulator = a)
            .with_preset(Preset::Absolute(b))
            .test();
    }
}
