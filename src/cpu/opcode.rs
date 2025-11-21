use num_enum::{FromPrimitive, IntoPrimitive};

#[derive(Debug, Clone, Copy, IntoPrimitive, FromPrimitive, Default)]
#[non_exhaustive]
#[repr(u8)]
pub enum Opcode {
    // ADC
    AdcImmediate = 0x69,
    AdcZeropage = 0x65,
    AdcZeropageX = 0x75,
    AdcAbsolute = 0x6D,
    AdcAbsoluteX = 0x7D,
    AdcAbsoluteY = 0x79,
    AdcIndirectX = 0x61,
    AdcIndirectY = 0x71,

    // AND
    AndImmediate = 0x29,
    AndZeropage = 0x25,
    AndZeropageX = 0x35,
    AndAbsolute = 0x2D,
    AndAbsoluteX = 0x3D,
    AndAbsoluteY = 0x39,
    AndIndirectX = 0x21,
    AndIndirectY = 0x31,

    // ASL
    AslAccumulator = 0x0A,
    AslZeropage = 0x06,
    AslZeropageX = 0x16,
    AslAbsolute = 0x0E,
    AslAbsoluteX = 0x1E,

    // BIT
    BitZeropage = 0x24,
    BitAbsolute = 0x2C,

    // CL*
    Clc = 0x18,
    Cld = 0xD8,
    Cli = 0x58,
    Clv = 0xB8,

    // CMP
    CmpImmediate = 0xC9,
    CmpZeropage = 0xC5,
    CmpZeropageX = 0xD5,
    CmpAbsolute = 0xCD,
    CmpAbsoluteX = 0xDD,
    CmpAbsoluteY = 0xD9,
    CmpIndirectX = 0xC1,
    CmpIndirectY = 0xD1,

    // CPX
    CpxImmediate = 0xE0,
    CpxZeropage = 0xE4,
    CpxAbsolute = 0xEC,

    // CPY
    CpyImmediate = 0xC0,
    CpyZeropage = 0xC4,
    CpyAbsolute = 0xCC,

    // DEX/Y
    Dex = 0xCA,
    Dey = 0x88,

    // EOR
    EorImmediate = 0x49,
    EorZeropage = 0x45,
    EorZeropageX = 0x55,
    EorAbsolute = 0x4D,
    EorAbsoluteX = 0x5D,
    EorAbsoluteY = 0x59,
    EorIndirectX = 0x41,
    EorIndirectY = 0x51,

    // IN*
    Inx = 0xE8,
    Iny = 0xC8,

    // LDA
    LdaImmediate = 0xA9,
    LdaZeropage = 0xA5,
    LdaZeropageX = 0xB5,
    LdaAbsolute = 0xAD,
    LdaAbsoluteX = 0xBD,
    LdaAbsoluteY = 0xB9,
    LdaIndirectX = 0xA1,
    LdaIndirectY = 0xB1,

    // LDX
    LdxImmediate = 0xA2,
    LdxZeropage = 0xA6,
    LdxZeropageY = 0xB6,
    LdxAbsolute = 0xAE,
    LdxAbsoluteY = 0xBE,

    // LDY
    LdyImmediate = 0xA0,
    LdyZeropage = 0xA4,
    LdyZeropageX = 0xB4,
    LdyAbsolute = 0xAC,
    LdyAbsoluteX = 0xBC,

    // LSR
    LsrAccumulator = 0x4A,
    LsrZeropage = 0x46,
    LsrZeropageX = 0x56,
    LsrAbsolute = 0x4E,
    LsrAbsoluteX = 0x5E,

    Nop = 0xEA,

    // ORA
    OraImmediate = 0x09,
    OraZeropage = 0x05,
    OraZeropageX = 0x15,
    OraAbsolute = 0x0D,
    OraAbsoluteX = 0x1D,
    OraAbsoluteY = 0x19,
    OraIndirectX = 0x01,
    OraIndirectY = 0x11,

    // ROR
    RorAccumulator = 0x6A,
    RorZeropage = 0x66,
    RorZeropageX = 0x76,
    RorAbsolute = 0x6E,
    RorAbsoluteX = 0x7E,

    // SBC
    SbcImmediate = 0xE9,
    SbcZeropage = 0xE5,
    SbcZeropageX = 0xF5,
    SbcAbsolute = 0xED,
    SbcAbsoluteX = 0xFD,
    SbcAbsoluteY = 0xF9,
    SbcIndirectX = 0xE1,
    SbcIndirectY = 0xF1,

    // SE*
    Sec = 0x38,
    Sed = 0xF8,
    Sei = 0x78,

    // STA
    StaZeropage = 0x85,
    StaZeropageX = 0x95,
    StaAbsolute = 0x8D,
    StaAbsoluteX = 0x9D,
    StaAbsoluteY = 0x99,
    StaIndirectX = 0x81,
    StaIndirectY = 0x91,

    // STX
    StxZeropage = 0x86,
    StxZeropageY = 0x96,
    StxAbsolute = 0x8E,

    // STY
    StyZeropage = 0x84,
    StyZeropageX = 0x94,
    StyAbsolute = 0x8C,

    // T**
    Tax = 0xAA,
    Tay = 0xA8,
    Tsx = 0xBA,
    Txa = 0x8A,
    Txs = 0x9A,
    Tya = 0x98,

    #[default]
    Unimplemented = 0x0,
}
