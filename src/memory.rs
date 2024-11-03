pub mod ram;

/// Trait for anything that acts like memory, i.e. can be written to or read from by the CPU.
///
/// Both loads and stores can mutate the state of memory,
/// for example, some addresses are mapped to I/O ports
pub trait Memory {
    fn load(&mut self, address: u16) -> u8;

    fn store(&mut self, address: u16, value: u8);
}
