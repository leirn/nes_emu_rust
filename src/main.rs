#![allow(arithmetic_overflow)]

extern crate argparse;
extern crate yaml_rust;
use argparse::{ArgumentParser, Store, StoreTrue};

use yaml_rust::YamlLoader;
mod apu;
mod cartridge;
mod cpu;
mod memory;
mod nes_emulator;
mod ppu;

fn main() {
    let mut verbose = false;
    let mut rom_file: String = String::new();
    let mut test_name: String = String::new();
    {
        // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("Yet another NES Emulator in Rust");
        ap.refer(&mut verbose)
            .add_option(&["-v", "--verbose"], StoreTrue, "Be verbose");
        ap.refer(&mut test_name).add_option(
            &["-t", "--test"],
            Store,
            "Launch a test instead of classic ROM. Tests are defined in config.yaml",
        );
        ap.refer(&mut rom_file)
            .add_argument("rom_file", Store, "File path to ROM File");
        ap.parse_args_or_exit();
    }

    let settings =
        load_file("C:/Users/laure/OneDrive/Documents/GitHub/nes_emu_rust/src/config.yaml");

    if test_name.len() > 0 {
        println!("Test ? {}", test_name);
        println!("{:?}", settings["tests"]["nestest"]);
        println!(
            "{}",
            settings["tests"][test_name.as_str()]["rom_file"]
                .as_str()
                .unwrap()
        );
        let current_dir = std::env::current_dir().unwrap();
        let rom_file = current_dir.join(
            settings["tests"][test_name.as_str()]["rom_file"]
                .as_str()
                .unwrap(),
        );
        let log_file = current_dir.join(
            settings["tests"][test_name.as_str()]["log_file"]
                .as_str()
                .unwrap(),
        );
        let entry_point = settings["tests"][test_name.as_str()]["entry_point"]
        .as_i64()
        .unwrap();
        println!("Rom test file : {}", rom_file.to_str().unwrap());
        println!("Log test file : {}", log_file.to_str().unwrap());
        println!("Entry point should be : {:x}", entry_point);
        let mut emulator = nes_emulator::NesEmulator::new(
            rom_file.to_str().unwrap().to_string(),
        );
        emulator.set_test_mode(log_file.to_str().unwrap());
        emulator.start(Some(
            entry_point as u16,
        ),);
    }

    let mut emulator = nes_emulator::NesEmulator::new(rom_file);
    emulator.start(None);
}

fn load_file(file: &str) -> yaml_rust::Yaml {
    let contents = std::fs::read_to_string(file).unwrap();
    let docs = YamlLoader::load_from_str(&contents).unwrap();
    let doc = &docs[0];
    doc.clone()
}
