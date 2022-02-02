extern crate lazy_static;
use crate::apu;
use crate::cpu;
use crate::ppu;
use crate::memory;
use crate::cartridge;
use crate::nes_emulator;

lazy_static::lazy_static! {
    pub static ref APU: std::sync::Mutex<apu::Apu> = std::sync::Mutex::new(apu::Apu::new());
    pub static ref CPU: std::sync::Mutex<cpu::Cpu> = std::sync::Mutex::new(cpu::Cpu::new());
    pub static ref PPU: std::sync::Mutex<ppu::Ppu> = std::sync::Mutex::new(ppu::Ppu::new());
    pub static ref MEMORY: std::sync::Mutex<memory::Memory> = std::sync::Mutex::new(memory::Memory::new());
    pub static ref CARTRIDGE: std::sync::Mutex<cartridge::Cartridge> = std::sync::Mutex::new(cartridge::Cartridge::new());
    pub static ref EMULATOR: nes_emulator::NesEmulator = nes_emulator::NesEmulator::new();
}