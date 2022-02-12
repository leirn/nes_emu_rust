#![allow(arithmetic_overflow)]

extern crate argparse;
extern crate yaml_rust;
use argparse::{ArgumentParser, StoreTrue, Store};

use std::collections::HashMap;
use std::fs::File;
use yaml_rust::YamlLoader;
mod cartridge;
mod apu;
mod cpu;
mod ppu;
mod memory;
mod nes_emulator;

fn main() {
    let mut verbose = false;
    let mut rom_file: String = String::new();
    let mut test_name: String = String::new();
    { // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("Yet another NES Emulator in Rust");
        ap.refer(&mut verbose)
            .add_option(&["-v", "--verbose"], StoreTrue,
            "Be verbose");
        ap.refer(&mut test_name)
            .add_option(&["-t", "--test"], Store,
            "Launch a test instead of classic ROM. Tests are defined in config.yaml");
        ap.refer(&mut rom_file)
            .add_argument("rom_file", Store,
            "File path to ROM File");
        ap.parse_args_or_exit();
    }

    let settings = load_file("C:/Users/laure/OneDrive/Documents/GitHub/nes_emu_rust/src/config.yaml");

    // Print out our settings (as a HashMap) for testing purpose
    println!(
        "{:?}",
        settings
    );

    let mut emulator = nes_emulator::NesEmulator::new(rom_file);
    emulator.start();
}


fn load_file(file: &str) -> Vec<yaml_rust::Yaml> {
    //let mut file = File::open(file).expect("Unable to open file");

    let contents = std::fs::read_to_string(file).unwrap();

    let docs = YamlLoader::load_from_str(&contents).unwrap();

    // iterate / process doc[s] ..
    docs
}