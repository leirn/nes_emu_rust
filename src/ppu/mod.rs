//! PPU Component of the NES
//! NSTC implementation
mod screen;
use crate::cartridge::Cartridge;
use std::cell::RefCell;
use std::rc::Rc;

struct Status{
    col: u16,
    line: u16,
}

pub struct Ppu {
    screen: screen::Screen,

    // internal registers
    register_v: u16,    // Current VRAM address, 15 bits
    register_t: u16,    // Temporary VRAM address, 15 bits. Can be thought of as address of top left onscreen tile
    register_x: u8,     // Fine X Scroll, 3 bits
    register_w: bool,   // First or second write toggle, 1 bit

    // Sprite registers
    primary_oam: [u8; 0x100],
    secondary_oam: [u8; 0x40],
    sprite_count: u8,
    sprite_fetcher_count: u8,
    secondary_oam_pointer: u8,

    // Cycle management
    col: u16,
    line: u16,
    is_odd_frame: bool,

    // general registers
    ppuctrl: u8,
    ppumask: u8,
    ppustatus: u8,
    oamaddr: u8,
    ppuscroll: u16,
    ppuaddr: u8,
    ppudata: u8,
    vram: [u8; 0x2000],
    palette_vram: [u8; 0x20],
}

impl Ppu {
    /// Instantiate the PPU
    pub fn new(cartridge: Rc<RefCell<Cartridge>>, sdl_context: Rc<RefCell<sdl2::Sdl>>) -> Ppu {
        Ppu {
            screen : screen::Screen::new(sdl_context),

            // internal registers
            register_v: 0,      // Current VRAM address, 15 bits
            register_t: 0,      // Temporary VRAM address, 15 bits. Can be thought of as address of top left onscreen tile
            register_x: 0,      // Fine X Scroll, 3 bits
            register_w: false,  // First or second write toggle, 1 bit

            // Sprite registers
            primary_oam: [0; 0x100],
            secondary_oam: [0; 0x40],
            sprite_count: 0,
            sprite_fetcher_count: 0,
            secondary_oam_pointer: 0,

            // Cycle management
            col: 0,
            line: 0,
            is_odd_frame: true,

            // general registers
            ppuctrl: 0,
            ppumask: 0,
            ppustatus: 0b10100000,
            oamaddr: 0,
            ppuscroll: 0,
            ppuaddr: 0,
            ppudata: 0,
            vram: [0; 0x2000],
            palette_vram: [0; 0x20],
        }
    }

    /// Start the PPU at NES start up
    pub fn start(&mut self) {
        self.screen.start();

        for i in 1..25 {
            for j in 1..25 {
                self.screen.update_pixel(110 + i, 105 + j, 5);
            }
        }
        self.screen.present();
        println!("PPU started, screen initialized");
    }

    /// Execute next instruction
    pub fn next(&self) {

    }

    pub fn read_0x2002(&self) -> u8 {
        0
    }

    pub fn read_0x2004(&self) -> u8 {
        0
    }

    pub fn read_0x2007(&self) -> u8 {
        0
    }

    pub fn write_0x2000(&self, value: u8) {

    }

    pub fn write_0x2001(&self, value: u8) {

    }

    pub fn write_0x2003(&self, value: u8) {

    }

    pub fn write_0x2004(&self, value: u8) {

    }

    pub fn write_0x2005(&self, value: u8) {

    }

    pub fn write_0x2006(&self, value: u8) {

    }

    pub fn write_0x2007(&self, value: u8) {

    }

    /// Return a dictionnary containing the current PPU Status. Usefull for debugging
    pub fn get_status(&self) -> Status {
        Status {
            col: self.col,
            line: self.line,
        }
    }
}
