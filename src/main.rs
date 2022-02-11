extern crate argparse;
use argparse::{ArgumentParser, StoreTrue, Store};

use config::Config;
mod cartridge;
mod apu;
mod cpu;
mod ppu;
mod memory;
mod nes_emulator;

fn main() {
    let mut verbose = false;
    let mut rom_file = "";
    let mut test_name: Option<String> = None;
    { // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("Yet another NES Emulator in Rust");
        ap.refer(&mut verbose)
            .add_option(&["-v", "--verbose"], StoreTrue,
            "Be verbose");
        ap.refer(&mut test_name)
            .add_option(&["-t", "--test"], Store,
            "Launch a test instead of classic ROM. Tests are defined in config.yaml");
        ap.refer($mut rom_file)
            .add_argument("rom_file", Store,
            "File path to ROM File");
        ap.parse_args_or_exit();
    }

    let settings = Config::builder()
        // Add in `config.yaml`
        .add_source(config::File::with_name("config.yaml"))
        .build()
        .unwrap();

    // Print out our settings (as a HashMap) for testing purpose
    println!(
        "{:?}",
        settings
            .try_deserialize::<HashMap<String, String>>()
            .unwrap()
    );

    let mut emulator = nes_emulator::NesEmulator::new(rom_file);
    emulator.start();
}