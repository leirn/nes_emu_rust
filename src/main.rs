mod cartridge;
mod nes_emulator;

fn main() {
    println!("Hello, world!");

    let rom_file:String = std::env::args().nth(1).expect("No file given");

    let mut the_cartridge: cartridge::Cartridge = cartridge::Cartridge::new();

    the_cartridge.parse_rom(rom_file);

    println!("{}", the_cartridge.file_name);

    let the_emulator: nes_emulator::NesEmulator = nes_emulator::NesEmulator::new();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("Window", 256, 240)
        .opengl() // this line DOES NOT enable opengl, but allows you to create/get an OpenGL context from your window.
        .build()
        .unwrap();
    let canvas = window.into_canvas()
        .index(find_sdl_gl_driver().unwrap())
        .build()
        .unwrap();
        
    the_emulator.start(sdl_context);
}

fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Some(index as u32);
        }
    }
    None
}