use super::mapper::Mapper;
use crate::cartridge::Cartridge;

pub struct Mapper0 {

}

unsafe impl Send for Mapper0 {}
/*
impl Mapper0 {
    pub fn new() -> Mapper0 {
        if CARTRIDGE.lock().unwrap().prg_rom_size == 16 * 1024 {
            CARTRIDGE.lock().unwrap().prg_rom.extend(CARTRIDGE.lock().unwrap().prg_rom.iter());
        }
        if CARTRIDGE.lock().unwrap().prg_rom_size == 0x1000 {
            CARTRIDGE.lock().unwrap().chr_rom.extend(CARTRIDGE.lock().unwrap().chr_rom.iter());
        }
        Mapper0 {}
    }
}
*/
impl Mapper for Cartridge {
    /// Read cartridge RAM
    fn read_ram(&mut self, address: u16) -> u8 {
        self.prg_ram[address as usize]
    }

    /// Read cartridge PRG ROM
    fn read_prg_rom(&mut self, address: u16) -> u8 {
        self.prg_rom[address as usize]
    }

    /// Read cartridge CHR ROM
    fn read_chr_rom(&mut self, address: u16) -> u8 {
        self.chr_rom[address as usize]
    }

    /// Write cartridge RAM
    fn write_ram(&mut self, address: u16, value: u8) {
        self.prg_ram[address as usize] = value;
    }

    /// Write cartridge PRG ROM
    fn write_prg_rom(&mut self, address: u16, value: u8) {
        self.prg_rom[address as usize] = value;
    }

    /// Write cartridge CHR ROM
    fn write_chr_rom(&mut self, address: u16, value: u8) {
        self.chr_rom[address as usize] = value;
    }

}