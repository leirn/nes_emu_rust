//! PPU Component of the NES
//! NSTC implementation
mod screen;
use crate::cartridge::Cartridge;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Status{
    pub col: u16,
    pub line: u16,
}

pub struct Ppu {
    screen: screen::Screen,
    cartridge: Rc<RefCell<Cartridge>>,

    // internal registers
    register_v: u16,    // Current VRAM address, 15 bits
    register_t: u16,    // Temporary VRAM address, 15 bits. Can be thought of as address of top left onscreen tile
    register_x: u8,     // Fine X Scroll, 3 bits
    register_w: bool,   // First or second write toggle, 1 bit

    // Sprite registers
    primary_oam: Vec<u8>,
    secondary_oam: Vec<u8>,
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
    vram: Vec<u8>,
    palette_vram: Vec<u8>,
}

impl Ppu {
    /// Instantiate the PPU
    pub fn new(cartridge: Rc<RefCell<Cartridge>>, sdl_context: Rc<RefCell<sdl2::Sdl>>) -> Ppu {
        Ppu {
            screen : screen::Screen::new(sdl_context),
            cartridge: cartridge,

            // internal registers
            register_v: 0,      // Current VRAM address, 15 bits
            register_t: 0,      // Temporary VRAM address, 15 bits. Can be thought of as address of top left onscreen tile
            register_x: 0,      // Fine X Scroll, 3 bits
            register_w: false,  // First or second write toggle, 1 bit

            // Sprite registers
            primary_oam: Vec::with_capacity(0x100),
            secondary_oam: Vec::with_capacity(0x40),
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
            vram: Vec::with_capacity(0x2000),
            palette_vram: Vec::with_capacity(0x20),
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

    /// lecture des addresses PPU Memory map
    ///
    /// 0x0000 to 0x2000 - 1 : Pattern table
    /// 0x2000 to 0x3000 - 1 : Nametable
    /// 0x3000 to 0x3eff :  Nametable Mirror
    /// 0x3f00 to 0x3f20 - 1 : Palette ram index
    /// 0x3f20 to 0x3fff = Palette ram mirror
    fn read_ppu_memory(&mut self, address: u16) -> u8 {
        match address {
            0..=0x1fff => self.cartridge.borrow_mut().read_chr_rom(address),
            0x2000..=0x2fff => self.vram[(address - 0x2000) as usize],
            0x3000..=0x3eff => self.vram[(address - 0x3000) as usize],
            0x3f00..=0x3fff => {let mut palette_address = 0;
                if address % 4 != 0 {
                    palette_address = address % 0x20;
                }
                self.palette_vram[palette_address as usize]
            },
            _ => panic!("Out of PPU memory range, address : {:04x}", address),
        }
    }

    /// ecriture des addresses PPU Memory map
    ///
    /// 0x0000 to 0x2000 - 1 : Pattern table
    /// 0x2000 to 0x3000 - 1 : Nametable
    /// 0x3000 to 0x3eff :  Nametable Mirror
    /// 0x3f00 to 0x3f20 - 1 : Palette ram index
    /// 0x3f20 to 0x3fff = Palette ram mirror
    fn write_ppu_memory(&mut self, address: u16, value: u8) {
        match address {
            0..=0x1fff => self.cartridge.borrow_mut().write_chr_rom(address, value),
            0x2000..=0x2fff => self.vram[(address - 0x2000) as usize] = value,
            0x3000..=0x3eff => self.vram[(address - 0x3000) as usize] = value,
            0x3f00..=0x3fff => {let mut palette_address = 0;
                if address % 4 != 0 {
                    palette_address = address % 0x20;
                }
                self.palette_vram[palette_address as usize] = value
            },
            _ => panic!("Out of PPU memory range, address : {:04x}", address),
        }
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
