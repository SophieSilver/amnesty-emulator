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
