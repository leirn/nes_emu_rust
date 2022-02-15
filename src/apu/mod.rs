//! APU Component
pub struct Apu {
}

impl Apu {
    /// Instantiate APU component
    pub fn new() -> Apu {
        Apu {
        }
    }

    /// Next APU cycle
    pub fn next(&self) {

    }

    /// Read APU registers
    pub fn read_registers(&mut self, address: u16) -> u8 {
        0
    }

    /// Write APU registers
    pub fn write_registers(&mut self, address: u16, value: u8) {

    }
}
