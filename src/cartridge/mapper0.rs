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
        if CARTRIDGE.lock().unwrap().chg_rom_size == 0x1000 {
            CARTRIDGE.lock().unwrap().chr_rom.extend(CARTRIDGE.lock().unwrap().chr_rom.iter());
        }
        Mapper0 {}
    }
}
*/