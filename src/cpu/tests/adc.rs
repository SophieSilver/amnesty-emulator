use crate::{
    cpu::{Cpu, StatusFlags},
    memory::MemoryMapping,
};
use utils::possible_pairs_with_carry;

use super::*;

fn verify_adc(a: u8, b: u8, carry: bool) -> impl Fn(&mut Cpu, &mut MemoryMapping) {
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
        assert_eq!(cpu.flags.contains(StatusFlags::ZERO), cpu.accumulator == 0);
    }
}

#[test]
fn immediate() {
    for (a, b, carry) in possible_pairs_with_carry() {
        TestOpcodeOptions::new(OpCode::AdcImmediate, 2, verify_adc(a, b, carry))
            .with_prepare(|cpu| {
                cpu.accumulator = a;
                cpu.flags.set(StatusFlags::CARRY, carry);
            })
            .with_arguments(&[b])
            .test();
    }
}

#[test]
fn zeropage() {
    for (a, b, carry) in possible_pairs_with_carry() {
        let addr = 0x25;

        TestOpcodeOptions::new(OpCode::AdcZeroPage, 3, verify_adc(a, b, carry))
            .with_prepare(|cpu| {
                cpu.accumulator = a;
                cpu.flags.set(StatusFlags::CARRY, carry);
            })
            .with_arguments(&[addr])
            .with_additional_values(&[(addr as u16, b)])
            .test();
    }
}

#[test]
fn zeropage_x() {
    for (a, b, carry) in possible_pairs_with_carry() {
        let base_addr = 0x25;
        let offset = 0x20;

        TestOpcodeOptions::new(OpCode::AdcZeroPageX, 4, verify_adc(a, b, carry))
            .with_prepare(|cpu| {
                cpu.accumulator = a;
                cpu.x_index = offset;
                cpu.flags.set(StatusFlags::CARRY, carry);
            })
            .with_arguments(&[base_addr])
            .with_additional_values(&[(base_addr.wrapping_add(offset) as u16, b)])
            .test();
    }
}

#[test]
fn zeropage_x_overflow() {
    for (a, b, carry) in possible_pairs_with_carry() {
        let base_addr = 0x85;
        let offset = 0xD0;

        TestOpcodeOptions::new(OpCode::AdcZeroPageX, 4, verify_adc(a, b, carry))
            .with_prepare(|cpu| {
                cpu.accumulator = a;
                cpu.x_index = offset;
                cpu.flags.set(StatusFlags::CARRY, carry);
            })
            .with_arguments(&[base_addr])
            .with_additional_values(&[(base_addr.wrapping_add(offset) as u16, b)])
            .test();
    }
}

#[test]
fn absolute() {
    for (a, b, carry) in possible_pairs_with_carry() {
        let addr: u16 = 0x0425;

        TestOpcodeOptions::new(OpCode::AdcAbsolute, 4, verify_adc(a, b, carry))
            .with_prepare(|cpu| {
                cpu.accumulator = a;
                cpu.flags.set(StatusFlags::CARRY, carry);
            })
            .with_arguments(&addr.to_le_bytes())
            .with_additional_values(&[(addr, b)])
            .test();
    }
}

#[test]
fn absolute_x() {
    for (a, b, carry) in possible_pairs_with_carry() {
        let addr: u16 = 0x0425;
        let offset: u8 = 0x5A;

        TestOpcodeOptions::new(OpCode::AdcAbsoluteX, 4, verify_adc(a, b, carry))
            .with_prepare(|cpu| {
                cpu.accumulator = a;
                cpu.x_index = offset;
                cpu.flags.set(StatusFlags::CARRY, carry);
            })
            .with_arguments(&addr.to_le_bytes())
            .with_additional_values(&[(addr.wrapping_add(offset as u16), b)])
            .test();
    }
}

#[test]
fn absolute_y() {
    for (a, b, carry) in possible_pairs_with_carry() {
        let addr: u16 = 0x0425;
        let offset: u8 = 0x5A;

        TestOpcodeOptions::new(OpCode::AdcAbsoluteY, 4, verify_adc(a, b, carry))
            .with_prepare(|cpu| {
                cpu.accumulator = a;
                cpu.y_index = offset;
                cpu.flags.set(StatusFlags::CARRY, carry);
            })
            .with_arguments(&addr.to_le_bytes())
            .with_additional_values(&[(addr.wrapping_add(offset as u16), b)])
            .test();
    }
}

#[test]
fn absolute_x_overflow() {
    for (a, b, carry) in possible_pairs_with_carry() {
        let addr: u16 = 0x04A5;
        let offset: u8 = 0x6A;

        TestOpcodeOptions::new(OpCode::AdcAbsoluteX, 5, verify_adc(a, b, carry))
            .with_prepare(|cpu| {
                cpu.accumulator = a;
                cpu.x_index = offset;
                cpu.flags.set(StatusFlags::CARRY, carry);
            })
            .with_arguments(&addr.to_le_bytes())
            .with_additional_values(&[(addr.wrapping_add(offset as u16), b)])
            .test();
    }
}

#[test]
fn absolute_y_overflow() {
    for (a, b, carry) in possible_pairs_with_carry() {
        let addr: u16 = 0x04A5;
        let offset: u8 = 0x6A;

        TestOpcodeOptions::new(OpCode::AdcAbsoluteY, 5, verify_adc(a, b, carry))
            .with_prepare(|cpu| {
                cpu.accumulator = a;
                cpu.y_index = offset;
                cpu.flags.set(StatusFlags::CARRY, carry);
            })
            .with_arguments(&addr.to_le_bytes())
            .with_additional_values(&[(addr.wrapping_add(offset as u16), b)])
            .test();
    }
}
#[test]
fn indirect_x() {
    for (a, b, carry) in possible_pairs_with_carry() {
        let ptr_base: u8 = 0x3F;
        let offset: u8 = 0x5A;
        let final_ptr = ptr_base.wrapping_add(offset) as u16;
        let addr: u16 = 0x0458;

        TestOpcodeOptions::new(OpCode::AdcIndirectX, 6, verify_adc(a, b, carry))
            .with_prepare(|cpu| {
                cpu.accumulator = a;
                cpu.x_index = offset;
                cpu.flags.set(StatusFlags::CARRY, carry);
            })
            .with_arguments(&[ptr_base])
            .with_additional_values(&[
                (final_ptr, addr.to_le_bytes()[0]),
                (final_ptr.wrapping_add(1), addr.to_le_bytes()[1]),
                (addr, b),
            ])
            .test();
    }
}

#[test]
fn indirect_y() {
    for (a, b, carry) in possible_pairs_with_carry() {
        let ptr: u8 = 0x3F;
        let offset: u8 = 0x5A;
        let base_addr: u16 = 0x0458;
        let final_addr = base_addr.wrapping_add(offset as u16);

        TestOpcodeOptions::new(OpCode::AdcIndirectY, 5, verify_adc(a, b, carry))
            .with_prepare(|cpu| {
                cpu.accumulator = a;
                cpu.y_index = offset;
                cpu.flags.set(StatusFlags::CARRY, carry);
            })
            .with_arguments(&[ptr])
            .with_additional_values(&[
                (ptr as u16, base_addr.to_le_bytes()[0]),
                (ptr.wrapping_add(1) as u16, base_addr.to_le_bytes()[1]),
                (final_addr, b),
            ])
            .test();
    }
}

#[test]
fn indirect_x_page_split() {
    for (a, b, carry) in possible_pairs_with_carry() {
        let ptr_base: u8 = 0xFF;
        let offset: u8 = 0x0;
        let final_ptr = ptr_base.wrapping_add(offset);
        let addr: u16 = 0x0458;

        TestOpcodeOptions::new(OpCode::AdcIndirectX, 6, verify_adc(a, b, carry))
            .with_prepare(|cpu| {
                cpu.accumulator = a;
                cpu.x_index = offset;
                cpu.flags.set(StatusFlags::CARRY, carry);
            })
            .with_arguments(&[ptr_base])
            .with_additional_values(&[
                (final_ptr as u16, addr.to_le_bytes()[0]),
                (final_ptr.wrapping_add(1) as u16, addr.to_le_bytes()[1]),
                (addr, b),
            ])
            .test();
    }
}

#[test]
fn indirect_y_page_split() {
    for (a, b, carry) in possible_pairs_with_carry() {
        let ptr: u8 = 0xFF;
        let offset: u8 = 0x5A;
        let base_addr: u16 = 0x0458;
        let final_addr = base_addr.wrapping_add(offset as u16);

        TestOpcodeOptions::new(OpCode::AdcIndirectY, 5, verify_adc(a, b, carry))
            .with_prepare(|cpu| {
                cpu.accumulator = a;
                cpu.y_index = offset;
                cpu.flags.set(StatusFlags::CARRY, carry);
            })
            .with_arguments(&[ptr])
            .with_additional_values(&[
                (ptr as u16, base_addr.to_le_bytes()[0]),
                (ptr.wrapping_add(1) as u16, base_addr.to_le_bytes()[1]),
                (final_addr, b),
            ])
            .test();
    }
}
