#![allow(arithmetic_overflow)]
#[macro_use]
extern crate log;

extern crate argparse;
extern crate yaml_rust;
use argparse::{ArgumentParser, Store, StoreTrue};
use log::info;
use nes_emu_rust::nes_emulator::NesEmulator;

fn main() {
    simple_logger::init().unwrap();

    let mut verbose = false;
    let mut rom_file: String = String::new();
    {
        // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("Yet another NES Emulator in Rust");
        ap.refer(&mut verbose)
            .add_option(&["-v", "--verbose"], StoreTrue, "Be verbose");
        ap.refer(&mut rom_file)
            .add_argument("rom_file", Store, "File path to ROM File");
        ap.parse_args_or_exit();
    }

    let mut emulator = NesEmulator::new(rom_file);
    emulator.start(None);
}
