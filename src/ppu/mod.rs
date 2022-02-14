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
    vram: [u8; 0x400],
    palette_vram: [u8; 0x20],
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
            vram: [0; 0x400],
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

    /// Update PPU internal register when CPU read 0x2002 memory address
    pub fn read_0x2002(&mut self) -> u8 {
        self.register_w = false;
        self.ppuaddr = 0;
        let value = self.ppustatus;
        self.ppustatus = value & 0b1111111;
        value
    }

    /// Read PPU internal register at 0x2004 memory address - read OAM at oamaddr
    pub fn read_0x2004(&self) -> u8 {
        self.primary_oam[self.oamaddr as usize]
    }

    /// Read PPU internal register at 0x2007 memory address
    pub fn read_0x2007(&mut self) -> u8 {
        let mut value: u8;
        if self.ppuaddr % 0x4000 < 0x3f00 { // Delayed buffering requiring dummy read
            value = self.ppudata;
            self.ppudata = self.read_ppu_memory(self.ppuaddr % 0x4000); // Address above 0x3fff are mirrored down
        }
        else {
            self.ppudata = self.read_ppu_memory(self.ppuaddr % 0x4000); // Address above 0x3fff are mirrored down
            value = self.ppudata;
        }
        self.read_or_write_0x2007();
        self.ppuaddr += if (self.ppuctrl >> 2) & 1 == 0 {1} else {0x20};
        value
    }

    /// Update PPU internal register when CPU write 0x2000 memory address
    pub fn write_0x2000(&mut self, value: u8) {
        self.ppuctrl = value;
        let t = self.register_t & 0b111001111111111;
        self.register_t = t | ((value as u16 & 0b11) << 10);
    }

    /// Update PPU internal register when CPU write 0x2001 memory address - ppumask
    pub fn write_0x2001(&mut self, value: u8) {
        self.ppumask = value;
    }

    /// Update PPU internal register when CPU write 0x2003 memory address - oamaddr
    pub fn write_0x2003(&mut self, value: u8) {
        self.oamaddr = value;
    }

    /// Update PPU internal register when CPU write 0x2004 memory address - read OAM at oamaddr
    pub fn write_0x2004(&mut self, value: u8) {
        self.primary_oam[self.oamaddr as usize] = value;
    }

    /// Update PPU internal register when CPU write 0x2005 memory address
    pub fn write_0x2005(&mut self, value: u8) {
        self.ppuscroll = ((self.ppuscroll << 8 ) + value as u16 ) & 0xffff;
        if self.register_w == false {
            self.register_t = (self.register_t & 0b111111111100000) | ((value as u16) >> 5);
            self.register_x = value & 0b111;
            self.register_w = true;
        }
        else {
            self.register_t = (self.register_t & 0b000110000011111) | ((value as u16 & 0b11111000) << 2) | ((value as u16 & 0b111) << 12);
            self.register_w = false;
        }
    }

    /// Update PPU internal register when CPU write 0x2006 memory address
    pub fn write_0x2006(&mut self, value: u8) {
        self.ppuaddr = ((self.ppuaddr << 8 ) + value as u16 ) & 0xffff;
        if self.register_w == false {
            self.register_t = (self.register_t & 0b000000011111111) | ((value as u16 & 0b00111111) << 8);
            self.register_w = true;
        }
        else {
            self.register_t = (self.register_t & 0b111111100000000) | value as u16;
            self.register_v = self.register_t;
            self.register_w = false;
        }
    }

    /// Write PPU internal register at 0x2007 memory address
    pub fn write_0x2007(&mut self, value: u8) {
        self.write_ppu_memory(self.ppuaddr % 0x4000, value); // Address above 0x3fff are mirrored down
        self.read_or_write_0x2007();
        self.ppuaddr += if (self.ppuctrl >> 2) & 1 == 0 {1} else {0x20};
    }

    fn read_or_write_0x2007(&mut self) {
        if not self.is_rendering_enabled {
            self.register_v += if (self.ppuctrl >> 2) & 1 == 0 {1} else {0x20};
        }
        else {
            self.inc_vert_v();
            self.inc_hor_v();
        }
    }

    /// Return a dictionnary containing the current PPU Status. Usefull for debugging
    pub fn get_status(&self) -> Status {
        Status {
            col: self.col,
            line: self.line,
        }
    }
}
