#![allow(clippy::arithmetic_side_effects)]

use utils::TestOpcodeOptions;

use crate::cpu::dispatch::OpCode;

#[allow(dead_code)]
mod utils;

mod adc {
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
}

mod lda {
    use super::*;

    #[test]
    fn immediate() {
        const TARGET: u8 = 0x42;

        TestOpcodeOptions::new(OpCode::LdaImmediate, 2, |cpu, _memory| {
            assert_eq!(cpu.accumulator, TARGET);
        })
        .with_arguments(&[TARGET])
        .test();
    }

    #[test]
    fn zeropage() {
        const TARGET: u8 = 0x21;
        const ADDR: u16 = 0x42;

        TestOpcodeOptions::new(OpCode::LdaZeroPage, 3, |cpu, _memory| {
            assert_eq!(cpu.accumulator, TARGET);
        })
        .with_arguments(&[ADDR as u8])
        .with_additional_values(&[(ADDR, TARGET)])
        .test();
    }

    #[test]
    fn zeropage_x() {
        const TARGET: u8 = 0x69;
        const BASE_ADDR: u16 = 0x42;
        const OFFSET: u8 = 0x3;
        const FINAL_ADDR: u16 = (BASE_ADDR as u8).wrapping_add(OFFSET) as u16;

        TestOpcodeOptions::new(OpCode::LdaZeroPageX, 4, |cpu, _memory| {
            assert_eq!(cpu.accumulator, TARGET);
        })
        .with_arguments(&[BASE_ADDR as u8])
        .with_prepare(|cpu| cpu.x_index = OFFSET)
        .with_additional_values(&[(FINAL_ADDR, TARGET)])
        .test();
    }

    #[test]
    fn zeropage_x_overflow() {
        const TARGET: u8 = 0x69;
        const BASE_ADDR: u16 = 0x42;
        const OFFSET: u8 = 0xFA;
        const FINAL_ADDR: u16 = (BASE_ADDR as u8).wrapping_add(OFFSET) as u16;

        TestOpcodeOptions::new(OpCode::LdaZeroPageX, 4, |cpu, _memory| {
            assert_eq!(cpu.accumulator, TARGET);
        })
        .with_arguments(&[BASE_ADDR as u8])
        .with_prepare(|cpu| cpu.x_index = OFFSET)
        .with_additional_values(&[(FINAL_ADDR, TARGET)])
        .test();
    }

    #[test]
    fn absolute() {
        const TARGET: u8 = 0x75;
        const ADDR: u16 = 0x0457;

        TestOpcodeOptions::new(OpCode::LdaAbsolute, 4, |cpu, _memory| {
            assert_eq!(cpu.accumulator, TARGET);
        })
        .with_arguments(&ADDR.to_le_bytes())
        .with_additional_values(&[(ADDR, TARGET)])
        .test();
    }

    #[test]
    fn absolute_x() {
        const TARGET: u8 = 0x34;
        const BASE_ADDR: u16 = 0x0361;
        const OFFSET: u8 = 0x12;
        const FINAL_ADDR: u16 = BASE_ADDR.wrapping_add(OFFSET as u16);

        TestOpcodeOptions::new(OpCode::LdaAbsoluteX, 4, |cpu, _memory| {
            assert_eq!(cpu.accumulator, TARGET);
        })
        .with_arguments(&BASE_ADDR.to_le_bytes())
        .with_prepare(|cpu| cpu.x_index = OFFSET)
        .with_additional_values(&[(FINAL_ADDR, TARGET)])
        .test();
    }

    #[test]
    fn absolute_x_overflow() {
        const TARGET: u8 = 0x34;
        const BASE_ADDR: u16 = 0x0361;
        const OFFSET: u8 = 0xFE;
        const FINAL_ADDR: u16 = BASE_ADDR.wrapping_add(OFFSET as u16);

        TestOpcodeOptions::new(OpCode::LdaAbsoluteX, 5, |cpu, _memory| {
            assert_eq!(cpu.accumulator, TARGET);
        })
        .with_arguments(&BASE_ADDR.to_le_bytes())
        .with_prepare(|cpu| cpu.x_index = OFFSET)
        .with_additional_values(&[(FINAL_ADDR, TARGET)])
        .test();
    }

    #[test]
    fn absolute_y() {
        const TARGET: u8 = 0x33;
        const BASE_ADDR: u16 = 0x0365;
        const OFFSET: u8 = 0x12;
        const FINAL_ADDR: u16 = BASE_ADDR.wrapping_add(OFFSET as u16);

        TestOpcodeOptions::new(OpCode::LdaAbsoluteY, 4, |cpu, _memory| {
            assert_eq!(cpu.accumulator, TARGET);
        })
        .with_arguments(&BASE_ADDR.to_le_bytes())
        .with_prepare(|cpu| cpu.y_index = OFFSET)
        .with_additional_values(&[(FINAL_ADDR, TARGET)])
        .test();
    }

    #[test]
    fn absolute_y_overflow() {
        const TARGET: u8 = 0x33;
        const BASE_ADDR: u16 = 0x0365;
        const OFFSET: u8 = 0xFE;
        const FINAL_ADDR: u16 = BASE_ADDR.wrapping_add(OFFSET as u16);

        TestOpcodeOptions::new(OpCode::LdaAbsoluteY, 5, |cpu, _memory| {
            assert_eq!(cpu.accumulator, TARGET);
        })
        .with_arguments(&BASE_ADDR.to_le_bytes())
        .with_prepare(|cpu| cpu.y_index = OFFSET)
        .with_additional_values(&[(FINAL_ADDR, TARGET)])
        .test();
    }

    #[test]
    fn indirect_x() {
        const TARGET: u8 = 0x57;
        const BASE_PTR: u8 = 0x77;
        const OFFSET: u8 = 0x32;
        const FINAL_PTR: u8 = BASE_PTR.wrapping_add(OFFSET);
        const ADDR: u16 = 0x0432;

        TestOpcodeOptions::new(OpCode::LdaIndirectX, 6, |cpu, _memory| {
            assert_eq!(cpu.accumulator, TARGET);
        })
        .with_arguments(&[BASE_PTR])
        .with_prepare(|cpu| cpu.x_index = OFFSET)
        .with_additional_values(&[
            (FINAL_PTR as u16, ADDR.to_le_bytes()[0]),
            (FINAL_PTR.wrapping_add(1) as u16, ADDR.to_le_bytes()[1]),
            (ADDR, TARGET),
        ])
        .test();
    }

    #[test]
    fn indirect_x_overflow() {
        const TARGET: u8 = 0x57;
        const BASE_PTR: u8 = 0x77;
        const OFFSET: u8 = 0xFF;
        const FINAL_PTR: u8 = BASE_PTR.wrapping_add(OFFSET);
        const ADDR: u16 = 0x0432;

        TestOpcodeOptions::new(OpCode::LdaIndirectX, 6, |cpu, _memory| {
            assert_eq!(cpu.accumulator, TARGET);
        })
        .with_arguments(&[BASE_PTR])
        .with_prepare(|cpu| cpu.x_index = OFFSET)
        .with_additional_values(&[
            (FINAL_PTR as u16, ADDR.to_le_bytes()[0]),
            (FINAL_PTR.wrapping_add(1) as u16, ADDR.to_le_bytes()[1]),
            (ADDR, TARGET),
        ])
        .test();
    }

    #[test]
    fn indirect_x_page_split() {
        const TARGET: u8 = 0x57;
        // the low byte of ADDR will be at 0xFF
        // the high byte -- at 0x00
        const BASE_PTR: u8 = 0xFF;
        const OFFSET: u8 = 0x0;
        const FINAL_PTR: u8 = BASE_PTR.wrapping_add(OFFSET);
        const ADDR: u16 = 0x0432;

        TestOpcodeOptions::new(OpCode::LdaIndirectX, 6, |cpu, _memory| {
            assert_eq!(cpu.accumulator, TARGET);
        })
        .with_arguments(&[BASE_PTR])
        .with_prepare(|cpu| cpu.x_index = OFFSET)
        .with_additional_values(&[
            (FINAL_PTR as u16, ADDR.to_le_bytes()[0]),
            (FINAL_PTR.wrapping_add(1) as u16, ADDR.to_le_bytes()[1]),
            (ADDR, TARGET),
        ])
        .test();
    }

    #[test]
    fn indirect_y() {
        const TARGET: u8 = 0xFD;
        const PTR: u8 = 0xD7;
        const BASE_ADDR: u16 = 0x057A;
        const OFFSET: u8 = 0x1D;
        const FINAL_ADDR: u16 = BASE_ADDR.wrapping_add(OFFSET as u16);

        TestOpcodeOptions::new(OpCode::LdaIndirectY, 5, |cpu, _memory| {
            assert_eq!(cpu.accumulator, TARGET);
        })
        .with_arguments(&[PTR])
        .with_additional_values(&[
            (PTR as u16, BASE_ADDR.to_le_bytes()[0]),
            (PTR.wrapping_add(1) as u16, BASE_ADDR.to_le_bytes()[1]),
            (FINAL_ADDR, TARGET),
        ])
        .with_prepare(|cpu| cpu.y_index = OFFSET)
        .test();
    }

    #[test]
    fn indirect_y_overflow() {
        const TARGET: u8 = 0xFD;
        const PTR: u8 = 0xD7;
        const BASE_ADDR: u16 = 0x057A;
        const OFFSET: u8 = 0xCE;
        const FINAL_ADDR: u16 = BASE_ADDR.wrapping_add(OFFSET as u16);

        TestOpcodeOptions::new(OpCode::LdaIndirectY, 6, |cpu, _memory| {
            assert_eq!(cpu.accumulator, TARGET);
        })
        .with_arguments(&[PTR])
        .with_additional_values(&[
            (PTR as u16, BASE_ADDR.to_le_bytes()[0]),
            (PTR.wrapping_add(1) as u16, BASE_ADDR.to_le_bytes()[1]),
            (FINAL_ADDR, TARGET),
        ])
        .with_prepare(|cpu| cpu.y_index = OFFSET)
        .test();
    }

    #[test]
    fn indirect_y_page_split() {
        const TARGET: u8 = 0xFD;
        // high byte of BASE_ADDR should be at 0x00
        const PTR: u8 = 0xFF;
        const BASE_ADDR: u16 = 0x057A;
        const OFFSET: u8 = 0x0;
        const FINAL_ADDR: u16 = BASE_ADDR.wrapping_add(OFFSET as u16);

        TestOpcodeOptions::new(OpCode::LdaIndirectY, 5, |cpu, _memory| {
            assert_eq!(cpu.accumulator, TARGET);
        })
        .with_arguments(&[PTR])
        .with_additional_values(&[
            (PTR as u16, BASE_ADDR.to_le_bytes()[0]),
            (PTR.wrapping_add(1) as u16, BASE_ADDR.to_le_bytes()[1]),
            (FINAL_ADDR, TARGET),
        ])
        .with_prepare(|cpu| cpu.y_index = OFFSET)
        .test();
    }
}

mod ldx {
    use super::*;

    #[test]
    fn immediate() {
        const TARGET: u8 = 0x42;

        TestOpcodeOptions::new(OpCode::LdxImmediate, 2, |cpu, _memory| {
            assert_eq!(cpu.x_index, TARGET);
        })
        .with_arguments(&[TARGET])
        .test();
    }

    #[test]
    fn zeropage() {
        const TARGET: u8 = 0x21;
        const ADDR: u16 = 0x42;

        TestOpcodeOptions::new(OpCode::LdxZeroPage, 3, |cpu, _memory| {
            assert_eq!(cpu.x_index, TARGET);
        })
        .with_arguments(&[ADDR as u8])
        .with_additional_values(&[(ADDR, TARGET)])
        .test();
    }

    #[test]
    fn zeropage_y() {
        const TARGET: u8 = 0x69;
        const BASE_ADDR: u16 = 0x42;
        const OFFSET: u8 = 0x3;
        const FINAL_ADDR: u16 = (BASE_ADDR as u8).wrapping_add(OFFSET) as u16;

        TestOpcodeOptions::new(OpCode::LdxZeroPageY, 4, |cpu, _memory| {
            assert_eq!(cpu.x_index, TARGET);
        })
        .with_arguments(&[BASE_ADDR as u8])
        .with_prepare(|cpu| cpu.y_index = OFFSET)
        .with_additional_values(&[(FINAL_ADDR, TARGET)])
        .test();
    }

    #[test]
    fn zeropage_y_overflow() {
        const TARGET: u8 = 0x69;
        const BASE_ADDR: u16 = 0x42;
        const OFFSET: u8 = 0xFA;
        const FINAL_ADDR: u16 = (BASE_ADDR as u8).wrapping_add(OFFSET) as u16;

        TestOpcodeOptions::new(OpCode::LdxZeroPageY, 4, |cpu, _memory| {
            assert_eq!(cpu.x_index, TARGET);
        })
        .with_arguments(&[BASE_ADDR as u8])
        .with_prepare(|cpu| cpu.y_index = OFFSET)
        .with_additional_values(&[(FINAL_ADDR, TARGET)])
        .test();
    }

    #[test]
    fn absolute() {
        const TARGET: u8 = 0x75;
        const ADDR: u16 = 0x0457;

        TestOpcodeOptions::new(OpCode::LdxAbsolute, 4, |cpu, _memory| {
            assert_eq!(cpu.x_index, TARGET);
        })
        .with_arguments(&ADDR.to_le_bytes())
        .with_additional_values(&[(ADDR, TARGET)])
        .test();
    }

    #[test]
    fn absolute_y() {
        const TARGET: u8 = 0x33;
        const BASE_ADDR: u16 = 0x0365;
        const OFFSET: u8 = 0x12;
        const FINAL_ADDR: u16 = BASE_ADDR.wrapping_add(OFFSET as u16);

        TestOpcodeOptions::new(OpCode::LdxAbsoluteY, 4, |cpu, _memory| {
            assert_eq!(cpu.x_index, TARGET);
        })
        .with_arguments(&BASE_ADDR.to_le_bytes())
        .with_prepare(|cpu| cpu.y_index = OFFSET)
        .with_additional_values(&[(FINAL_ADDR, TARGET)])
        .test();
    }

    #[test]
    fn absolute_y_overflow() {
        const TARGET: u8 = 0x33;
        const BASE_ADDR: u16 = 0x0365;
        const OFFSET: u8 = 0xFE;
        const FINAL_ADDR: u16 = BASE_ADDR.wrapping_add(OFFSET as u16);

        TestOpcodeOptions::new(OpCode::LdxAbsoluteY, 5, |cpu, _memory| {
            assert_eq!(cpu.x_index, TARGET);
        })
        .with_arguments(&BASE_ADDR.to_le_bytes())
        .with_prepare(|cpu| cpu.y_index = OFFSET)
        .with_additional_values(&[(FINAL_ADDR, TARGET)])
        .test();
    }
}

mod ldy {
    use super::*;

    #[test]
    fn immediate() {
        const TARGET: u8 = 0x42;

        TestOpcodeOptions::new(OpCode::LdyImmediate, 2, |cpu, _memory| {
            assert_eq!(cpu.y_index, TARGET);
        })
        .with_arguments(&[TARGET])
        .test();
    }

    #[test]
    fn zeropage() {
        const TARGET: u8 = 0x21;
        const ADDR: u16 = 0x42;

        TestOpcodeOptions::new(OpCode::LdyZeroPage, 3, |cpu, _memory| {
            assert_eq!(cpu.y_index, TARGET);
        })
        .with_arguments(&[ADDR as u8])
        .with_additional_values(&[(ADDR, TARGET)])
        .test();
    }

    #[test]
    fn zeropage_x() {
        const TARGET: u8 = 0x69;
        const BASE_ADDR: u16 = 0x42;
        const OFFSET: u8 = 0x3;
        const FINAL_ADDR: u16 = (BASE_ADDR as u8).wrapping_add(OFFSET) as u16;

        TestOpcodeOptions::new(OpCode::LdyZeroPageX, 4, |cpu, _memory| {
            assert_eq!(cpu.y_index, TARGET);
        })
        .with_arguments(&[BASE_ADDR as u8])
        .with_prepare(|cpu| cpu.x_index = OFFSET)
        .with_additional_values(&[(FINAL_ADDR, TARGET)])
        .test();
    }

    #[test]
    fn zeropage_x_overflow() {
        const TARGET: u8 = 0x69;
        const BASE_ADDR: u16 = 0x42;
        const OFFSET: u8 = 0xFA;
        const FINAL_ADDR: u16 = (BASE_ADDR as u8).wrapping_add(OFFSET) as u16;

        TestOpcodeOptions::new(OpCode::LdyZeroPageX, 4, |cpu, _memory| {
            assert_eq!(cpu.y_index, TARGET);
        })
        .with_arguments(&[BASE_ADDR as u8])
        .with_prepare(|cpu| cpu.x_index = OFFSET)
        .with_additional_values(&[(FINAL_ADDR, TARGET)])
        .test();
    }

    #[test]
    fn absolute() {
        const TARGET: u8 = 0x75;
        const ADDR: u16 = 0x0457;

        TestOpcodeOptions::new(OpCode::LdyAbsolute, 4, |cpu, _memory| {
            assert_eq!(cpu.y_index, TARGET);
        })
        .with_arguments(&ADDR.to_le_bytes())
        .with_additional_values(&[(ADDR, TARGET)])
        .test();
    }

    #[test]
    fn absolute_x() {
        const TARGET: u8 = 0x33;
        const BASE_ADDR: u16 = 0x0365;
        const OFFSET: u8 = 0x12;
        const FINAL_ADDR: u16 = BASE_ADDR.wrapping_add(OFFSET as u16);

        TestOpcodeOptions::new(OpCode::LdyAbsoluteX, 4, |cpu, _memory| {
            assert_eq!(cpu.y_index, TARGET);
        })
        .with_arguments(&BASE_ADDR.to_le_bytes())
        .with_prepare(|cpu| cpu.x_index = OFFSET)
        .with_additional_values(&[(FINAL_ADDR, TARGET)])
        .test();
    }

    #[test]
    fn absolute_x_overflow() {
        const TARGET: u8 = 0x33;
        const BASE_ADDR: u16 = 0x0365;
        const OFFSET: u8 = 0xFE;
        const FINAL_ADDR: u16 = BASE_ADDR.wrapping_add(OFFSET as u16);

        TestOpcodeOptions::new(OpCode::LdyAbsoluteX, 5, |cpu, _memory| {
            assert_eq!(cpu.y_index, TARGET);
        })
        .with_arguments(&BASE_ADDR.to_le_bytes())
        .with_prepare(|cpu| cpu.x_index = OFFSET)
        .with_additional_values(&[(FINAL_ADDR, TARGET)])
        .test();
    }
}
