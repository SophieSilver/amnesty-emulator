use crate::cpu::{
    instructions::opcode::{self, OpCode},
    Cpu, StatusFlags,
};
use utils::possible_pairs_with_carry;

use super::*;

fn verify(a: u8, b: u8, carry: bool) -> impl Fn(&mut Cpu, &mut TestMemory) {
    let unsigned_result = (a as u32)
        .wrapping_sub(b as u32)
        .wrapping_sub(!carry as u32);
    let signed_result = a as i8 as i32 - b as i8 as i32 - !carry as i32;

    let should_carry = (u8::MIN as u32..=u8::MAX as u32).contains(&unsigned_result);
    let should_overflow = !(i8::MIN as i32..=i8::MAX as i32).contains(&signed_result);

    let truncated_result = unsigned_result as i8 as u8;

    move |cpu, _memory| {
        assert_eq!(
            cpu.a, truncated_result,
            "Addition result incorrect a = {a}, b = {b}, carry = {carry}"
        );
        assert_eq!(
            cpu.flags.contains(StatusFlags::CARRY),
            should_carry,
            "CARRY flag set incorrectly a = {a}, b = {b}, carry = {carry}"
        );
        assert_eq!(
            cpu.flags.contains(StatusFlags::OVERFLOW),
            should_overflow,
            "OVERFLOW flag set incorrectly a = {a}, b = {b}, carry = {carry}"
        );
        assert_eq!(
            cpu.flags.contains(StatusFlags::NEGATIVE),
            (cpu.a as i8).is_negative(),
            "NEGATIVE flag set incorrectly a = {a}, b = {b}, carry = {carry}"
        );
        assert_eq!(
            cpu.flags.contains(StatusFlags::ZERO),
            cpu.a == 0,
            "ZERO flag set incorrectly a = {a}, b = {b}, carry = {carry}"
        );
    }
}

#[test]
fn immediate() {
    for (a, b, carry) in possible_pairs_with_carry() {
        TestOpcodeOptions::new(OpCode::SbcImmediate, 2, verify(a, b, carry))
            .with_prepare(|cpu| {
                cpu.a = a;
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

        TestOpcodeOptions::new(OpCode::SbcZeropage, 3, verify(a, b, carry))
            .with_prepare(|cpu| {
                cpu.a = a;
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

        TestOpcodeOptions::new(OpCode::SbcZeropageX, 4, verify(a, b, carry))
            .with_prepare(|cpu| {
                cpu.a = a;
                cpu.x = offset;
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

        TestOpcodeOptions::new(OpCode::SbcZeropageX, 4, verify(a, b, carry))
            .with_prepare(|cpu| {
                cpu.a = a;
                cpu.x = offset;
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

        TestOpcodeOptions::new(OpCode::SbcAbsolute, 4, verify(a, b, carry))
            .with_prepare(|cpu| {
                cpu.a = a;
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

        TestOpcodeOptions::new(OpCode::SbcAbsoluteX, 4, verify(a, b, carry))
            .with_prepare(|cpu| {
                cpu.a = a;
                cpu.x = offset;
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

        TestOpcodeOptions::new(OpCode::SbcAbsoluteY, 4, verify(a, b, carry))
            .with_prepare(|cpu| {
                cpu.a = a;
                cpu.y = offset;
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

        TestOpcodeOptions::new(OpCode::SbcAbsoluteX, 5, verify(a, b, carry))
            .with_prepare(|cpu| {
                cpu.a = a;
                cpu.x = offset;
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

        TestOpcodeOptions::new(OpCode::SbcAbsoluteY, 5, verify(a, b, carry))
            .with_prepare(|cpu| {
                cpu.a = a;
                cpu.y = offset;
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

        TestOpcodeOptions::new(OpCode::SbcIndirectX, 6, verify(a, b, carry))
            .with_prepare(|cpu| {
                cpu.a = a;
                cpu.x = offset;
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

        TestOpcodeOptions::new(OpCode::SbcIndirectY, 5, verify(a, b, carry))
            .with_prepare(|cpu| {
                cpu.a = a;
                cpu.y = offset;
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
fn indirect_x_overflow() {
    for (a, b, carry) in possible_pairs_with_carry() {
        let ptr_base: u8 = 0x3F;
        let offset: u8 = 0xFA;
        let final_ptr = ptr_base.wrapping_add(offset) as u16;
        let addr: u16 = 0x0458;

        TestOpcodeOptions::new(OpCode::SbcIndirectX, 6, verify(a, b, carry))
            .with_prepare(|cpu| {
                cpu.a = a;
                cpu.x = offset;
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
fn indirect_y_overflow() {
    for (a, b, carry) in possible_pairs_with_carry() {
        let ptr: u8 = 0x3F;
        let offset: u8 = 0xFA;
        let base_addr: u16 = 0x0458;
        let final_addr = base_addr.wrapping_add(offset as u16);

        TestOpcodeOptions::new(OpCode::SbcIndirectY, 6, verify(a, b, carry))
            .with_prepare(|cpu| {
                cpu.a = a;
                cpu.y = offset;
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

        TestOpcodeOptions::new(OpCode::SbcIndirectX, 6, verify(a, b, carry))
            .with_prepare(|cpu| {
                cpu.a = a;
                cpu.x = offset;
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

        TestOpcodeOptions::new(OpCode::SbcIndirectY, 5, verify(a, b, carry))
            .with_prepare(|cpu| {
                cpu.a = a;
                cpu.y = offset;
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
