pub struct Apu {
}


unsafe impl Sync for Apu {}


impl Apu {
    pub fn new() -> Apu {
        Apu {
        }
    }

    pub fn read_registers(&mut self, address: u16) -> u8 {
        0
    }

    pub fn write_registers(&mut self, address: u16, value: u8) {
        
    }
}