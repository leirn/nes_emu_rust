//! APU Component
pub struct Apu {
}

unsafe impl Sync for Apu {}

impl Apu {
    /// Instantiate APU component
    pub fn new() -> Apu {
        Apu {
        }
    }

    /// Read APU registers
    pub fn read_registers(&mut self, address: u16) -> u8 {
        0
    }

    /// Write APU registers
    pub fn write_registers(&mut self, address: u16, value: u8) {
        
    }
}
