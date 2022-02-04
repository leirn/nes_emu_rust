//! Define trait for mappers

// Mapper trait
pub trait Mapper {
    /// Read cartridge RAM
    fn read_ram(&self, address: u16) -> u8 {
        0
    }

    /// Read cartridge PRG ROM
    fn read_prg_rom(&self, address: u16) -> u8 {
        0
    }

    /// Read cartridge CHR ROM
    fn read_chr_rom(&self, address: u16) -> u8 {
        0
    }

    /// Write cartridge RAM
    fn write_ram(&self, address: u16, value: u8) {
        
    }

    /// Write cartridge PRG ROM
    fn write_prg_rom(&self, address: u16, value: u8) {
        
    }

    /// Write cartridge CHR ROM
    fn write_chr_rom(&self, address: u16, value: u8) {
        
    }
}