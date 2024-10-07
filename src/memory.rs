use ram::Ram;
pub mod ram;

/// Console's memory mapping.
/// Allows the cpu to read and write to mapped addresses
///
/// This struct collects references to hardware that can be mapped to memory,
/// such as IO registers, cartridge ROMs and regular RAM.
#[derive(Debug)]
pub struct MemoryMapping<'a> {
    pub ram: &'a mut Ram,
}

impl MemoryMapping<'_> {
    pub fn load(&mut self, address: u16) -> u8 {
        match address {
            0x0000..0x1000 => self.ram.load(address % 0x800),
            _ => unimplemented!(),
        }
    }

    pub fn store(&mut self, address: u16, value: u8) {
        match address {
            0x0000..0x1000 => self.ram.store(address % 0x800, value),
            _ => unimplemented!(),
        };
    }
}
