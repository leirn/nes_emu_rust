#[macro_use]
mod components;
mod cartridge;
mod apu;
mod cpu;
mod ppu;
mod memory;
mod nes_emulator;

use components::{CARTRIDGE, EMULATOR};


fn main() {
    println!("Hello, world!");

    let rom_file:String = std::env::args().nth(1).expect("No file given");

    CARTRIDGE.lock().unwrap().parse_rom(rom_file);

    println!("{}", CARTRIDGE.lock().unwrap().file_name);

    EMULATOR.lock().unwrap().start();
}