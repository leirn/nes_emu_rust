mod cartridge;

fn main() {
    println!("Hello, world!");

    let rom_file:String = std::env::args().nth(1).expect("No file given");

    let mut the_cartridge: cartridge::Cartridge = cartridge::Cartridge::new();

    the_cartridge.parse_rom(rom_file);

    println!("{}", the_cartridge.file_name);
}
