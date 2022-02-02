extern crate lazy_static;
use crate::apu;
use crate::cpu;
use crate::ppu;
use crate::cartridge;
use crate::nes_emulator;

lazy_static::lazy_static! {
    pub static ref APU: apu::Apu = apu::Apu::new();
    pub static ref CPU: cpu::Cpu = cpu::Cpu::new();
    pub static ref PPU: std::sync::Mutex<ppu::Ppu> = std::sync::Mutex::new(ppu::Ppu::new());
    pub static ref CARTRIDGE: std::sync::Mutex<cartridge::Cartridge> = std::sync::Mutex::new(cartridge::Cartridge::new());
    pub static ref EMULATOR: nes_emulator::NesEmulator = nes_emulator::NesEmulator::new();
}