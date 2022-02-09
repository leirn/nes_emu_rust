#[macro_use]
mod cartridge;
mod apu;
mod cpu;
mod ppu;
mod memory;
mod nes_emulator;

fn main() {
    let rom_file:String = std::env::args().nth(1).expect("No file given");

    let mut emulator = nes_emulator::NesEmulator::new(rom_file);
    emulator.start();
}