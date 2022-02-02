pub struct Memory {
    internal_ram: Vec<u8>,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            internal_ram: Vec::with_capacity(0x400)
        }
    }

    pub fn read_rom_16(&mut self, address: u16) -> u16 {
        0
    }

    pub fn read_rom(&mut self, address: u16) -> u8 {
        0
    }
}