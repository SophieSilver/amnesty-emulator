pub const ZEROPAGE_ADDR: u8 = 0x25;

pub const ZEROPAGE_X_BASE_ADDR: u8 = 0x27;
pub const ZEROPAGE_X_OFFSET: u8 = 0x20;
pub const ZEROPAGE_X_FINAL_ADDR: u16 = ZEROPAGE_X_BASE_ADDR.wrapping_add(ZEROPAGE_X_OFFSET) as u16;

pub const ZEROPAGE_X_BASE_ADDR_OVERFLOW: u8 = 0x85;
pub const ZEROPAGE_X_OFFSET_OVERFLOW: u8 = 0xD0;
pub const ZEROPAGE_X_FINAL_ADDR_OVERFLOW: u16 =
    ZEROPAGE_X_BASE_ADDR_OVERFLOW.wrapping_add(ZEROPAGE_X_OFFSET_OVERFLOW) as u16;

pub const ABSOLUTE_ADDR: u16 = 0x0425;

pub const ABSOLUTE_X_BASE_ADDR: u16 = 0x427;
pub const ABSOLUTE_X_OFFSET: u8 = 0x5A;
pub const ABSOLUTE_X_FINAL_ADDR: u16 = ABSOLUTE_X_BASE_ADDR.wrapping_add(ABSOLUTE_X_OFFSET as u16);

pub const ABSOLUTE_X_BASE_ADDR_OVERFLOW: u16 = 0x04A5;
pub const ABSOLUTE_X_OFFSET_OVERFLOW: u8 = 0x6A;
pub const ABSOLUTE_X_FINAL_ADDR_OVERFLOW: u16 =
    ABSOLUTE_X_BASE_ADDR_OVERFLOW.wrapping_add(ABSOLUTE_X_OFFSET_OVERFLOW as u16);

pub const INDIRECT_X_PTR_BASE: u8 = 0x3F;
pub const INDIRECT_X_OFFSET: u8 = 0x5A;
pub const INDIRECT_X_FINAL_PTR: u8 = INDIRECT_X_PTR_BASE.wrapping_add(INDIRECT_X_OFFSET);
pub const INDIRECT_X_ADDR: u16 = 0x0458;

pub const INDIRECT_X_PTR_BASE_OVERFLOW: u8 = 0x3D;
pub const INDIRECT_X_OFFSET_OVERFLOW: u8 = 0xFA;
pub const INDIRECT_X_FINAL_PTR_OVERFLOW: u8 =
    INDIRECT_X_PTR_BASE_OVERFLOW.wrapping_add(INDIRECT_X_OFFSET_OVERFLOW);

pub const INDIRECT_X_ADDR_OVERFLOW: u16 = 0x0498;

pub const INDIRECT_X_PTR_BASE_PAGE_SPLIT: u8 = 0xFF;
pub const INDIRECT_X_OFFSET_PAGE_SPLIT: u8 = 0x00;
pub const INDIRECT_X_FINAL_PTR_PAGE_SPLIT: u8 =
    INDIRECT_X_PTR_BASE_PAGE_SPLIT.wrapping_add(INDIRECT_X_OFFSET_PAGE_SPLIT);

pub const INDIRECT_X_ADDR_PAGE_SPLIT: u16 = 0x0428;

pub const INDIRECT_Y_PTR: u8 = 0x4F;
pub const INDIRECT_Y_OFFSET: u8 = 0x5B;
pub const INDIRECT_Y_BASE_ADDR: u16 = 0x0478;
pub const INDIRECT_Y_FINAL_ADDR: u16 = INDIRECT_Y_BASE_ADDR.wrapping_add(INDIRECT_Y_OFFSET as u16);

pub const INDIRECT_Y_PTR_OVERFLOW: u8 = 0x5F;
pub const INDIRECT_Y_OFFSET_OVERFLOW: u8 = 0xFB;
pub const INDIRECT_Y_BASE_ADDR_OVERFLOW: u16 = 0x04A8;
pub const INDIRECT_Y_FINAL_ADDR_OVERFLOW: u16 =
    INDIRECT_Y_BASE_ADDR_OVERFLOW.wrapping_add(INDIRECT_Y_OFFSET_OVERFLOW as u16);

pub const INDIRECT_Y_PTR_PAGE_SPLIT: u8 = 0xFF;
pub const INDIRECT_Y_OFFSET_PAGE_SPLIT: u8 = 0x5C;
pub const INDIRECT_Y_BASE_ADDR_PAGE_SPLIT: u16 = 0x0411;
pub const INDIRECT_Y_FINAL_ADDR_PAGE_SPLIT: u16 =
    INDIRECT_Y_BASE_ADDR_PAGE_SPLIT.wrapping_add(INDIRECT_Y_OFFSET_PAGE_SPLIT as u16);
