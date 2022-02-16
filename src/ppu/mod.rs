//! PPU Component of the NES
//! NSTC implementation
mod screen;
use crate::cartridge::Cartridge;
use crate::bus::interrupt::Interrupt;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::VecDeque;

pub struct Status {
    pub col: u16,
    pub line: u16,
}

pub struct Ppu {
    screen: screen::Screen,
    interrupt_bus: Rc<RefCell<Interrupt>>,
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
    sprite_fetcher_count: usize,
    secondary_oam_pointer: usize,

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
    ppuaddr: u16,
    ppudata: u8,
    vram: [u8; 0x2000],
    palette_vram: [u8; 0x20],

    // Pixel generator part
    // Start with two empty tiles
    bg_palette_register: VecDeque<u8>,
    bg_low_byte_table_register: VecDeque<u8>,
    bg_high_byte_table_register: VecDeque<u8>,
    bg_attribute_table_register: VecDeque<u8>,
    bg_nt_table_register: VecDeque<u8>,

    sprite_low_byte_table_register: VecDeque<u8>,
    sprite_high_byte_table_register: VecDeque<u8>,
    sprite_attribute_table_register: VecDeque<u8>,
    sprite_x_coordinate_table_register: VecDeque<u8>,
}

impl Ppu {
    /// Instantiate the PPU
    pub fn new(_cartridge: Rc<RefCell<Cartridge>>, _sdl_context: Rc<RefCell<sdl2::Sdl>>, _interrupt_bus: Rc<RefCell<Interrupt>>) -> Ppu {
        Ppu {
            screen : screen::Screen::new(_sdl_context),
            cartridge: _cartridge,
            interrupt_bus: _interrupt_bus,

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

            // Pixel generator variables
            bg_palette_register: VecDeque::from([0, 0]),
            bg_low_byte_table_register: VecDeque::from([0, 0]),
            bg_high_byte_table_register: VecDeque::from([0, 0]),
            bg_attribute_table_register: VecDeque::from([0, 0]),
            bg_nt_table_register: VecDeque::from([0, 0]),

            sprite_low_byte_table_register: VecDeque::new(),
            sprite_high_byte_table_register: VecDeque::new(),
            sprite_attribute_table_register: VecDeque::new(),
            sprite_x_coordinate_table_register: VecDeque::new(),
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

    /// Next function that implement the almost exact PPU rendering workflow
    pub fn next(&mut self) {
        // Pixel rendering
        match self.line {
            0..=239u16 => {
                if self.col > 0 && self.col < 257 {
                    if self.line < 240 && self.is_bg_rendering_enabled() {
                        let pixel_color = self.compute_next_pixel();
                        self.screen.update_pixel(pixel_color, (self.col - 1) as u8, self.line as u8);
                    }
                    // Nothing happens during Vblank
                    self.next_background_evaluation();
                    self.next_sprite_evaluation();
                }
            },
            241u16 => {
                if self.col == 1 {
                    self.screen.present();
                    self.set_vblank();
                    if self.is_nmi_bit_set() {
                        self.interrupt_bus.borrow_mut().raise_nmi();
                    }
                }
            },
            261u16 => {
                if self.col > 0 && self.col < 257 {
                    self.next_background_evaluation();
                    self.next_sprite_evaluation();
                }
                if self.col == 1 {
                    self.clear_vblank();
                    self.clear_sprite0_hit();
                    self.clear_sprite_overflow();
                }
            },
            _ => ()
        }

        self.col  = (self.col + 1) % 341;
        if self.col == 0 {
            // End of scan line
            self.line = (self.line + 1) % 262;
        }
        if (self.col, self.line) == (0, 0) {
            self.interrupt_bus.borrow_mut().set_frame_updated();
            if self.is_odd_frame {
                self.col = 1;
            }
            self.is_odd_frame = !self.is_odd_frame;
        }
    }

    /// Execute next instruction
    pub fn next_background_evaluation(&mut self) {
        if self.line < 240 || self.line == 261 { // Normal line
            if self.col > 0 && self.col < 257 {
                self.load_tile_data();
            }
            if self.col == 257 {
                self.copy_hor_t_to_hor_v();
            }
            if self.col > 320 && self.col < 337 {
                self.load_tile_data();
            }
        }
        if self.is_rendering_enabled() && self.line == 261 && self.col > 279 && self.col < 305 {
            self.copy_vert_t_to_vert_v();
        }
    }

    /// 8 cycle operation to load next tile data
    pub fn load_tile_data(&mut self) {
        match self.col % 8 {
            1 => {
                // read NT Byte for N+2 tile
                let tile_address = 0x2000 | (self.register_v & 0xfff); // Is it NT or tile address ?
                let nt_byte = self.read_ppu_memory(tile_address);
                self.set_nt_byte(nt_byte);
            },
            3 => {
                // read AT Byte for N+2 tile
                let attribute_address = 0x23c0 | (self.register_v & 0xC00) | ((self.register_v >> 4) & 0x38) | ((self.register_v >> 2) & 0x07);
                let at_byte = self.read_ppu_memory(attribute_address);
                self.set_at_byte(at_byte);
            },
            5 => {
                // read low BG Tile Byte for N+2 tile
                let chr_bank = ((self.ppuctrl as u16 >> 4) & 1) * 0x1000;
                let fine_y = self.register_v >> 12;
                let tile_address = *self.bg_nt_table_register.back().unwrap() as u16;
                let low_bg_tile_byte = self.read_ppu_memory(chr_bank + 16 * tile_address + fine_y);
                self.set_low_bg_tile_byte(low_bg_tile_byte);
            },
            7 => {
                // read high BG Tile Byte for N+2 tile
                let chr_bank = ((self.ppuctrl as u16 >> 4) & 1) * 0x1000;
                let fine_y = self.register_v >> 12;
                let tile_address = *self.bg_nt_table_register.back().unwrap() as u16;
                let high_bg_tile_byte = self.read_ppu_memory(chr_bank + 16 * tile_address + 8 + fine_y);
                self.set_high_bg_tile_byte(high_bg_tile_byte);
            },
            0 => {
                self.shift_registers();
                if self.col == 256 {
                    self.inc_vert_v();
                }
                else {
                    self.inc_hor_v();
                }
            },
            _ => (),
        }
    }

    /// Handle the sprite evaluation process
    pub fn next_sprite_evaluation(&mut self) {
        if self.col > 0 && self.col < 65 {
            // During those cycles, Secondary OAM is clear on byte after another
            self.secondary_oam[(self.col - 1) as usize] = 0xff;
        }
        if self.col == 64 {
            self.sprite_count = 0;
            self.secondary_oam_pointer = 0;
        }

        if self.secondary_oam_pointer > 7 {
            return; // Maximum 8 sprites found per frame
        }
        if self.col > 64 && self.col < 256 && self.sprite_count < 64 {
            // During those cycles, sprites are actually evaluated
            // Fetch next sprite first byte (y coordinate)
            let sprite_y_coordinate = self.primary_oam[(4 * self.sprite_count) as usize];
            self.secondary_oam[self.secondary_oam_pointer * 4] = sprite_y_coordinate;
            let sprite_y_coordinate = sprite_y_coordinate as u16;
            if self.line >= sprite_y_coordinate && self.line < sprite_y_coordinate + 7 {
                // Le sprite traverse la scanline, on le copy dans  le secondary oam
                self.secondary_oam[self.secondary_oam_pointer * 4 + 1] = self.primary_oam[(4 * self.sprite_count + 1) as usize];
                self.secondary_oam[self.secondary_oam_pointer * 4 + 2] = self.primary_oam[(4 * self.sprite_count + 2) as usize];
                self.secondary_oam[self.secondary_oam_pointer * 4 + 3] = self.primary_oam[(4 * self.sprite_count + 3) as usize];
                self.secondary_oam_pointer += 1;
            }
            self.sprite_count += 1;
        }

        if self.col == 256 {
            self.sprite_fetcher_count: usize = 0;
            self.clear_sprite_registers();
        }

        if self.sprite_fetcher_count < self.secondary_oam_pointer && self.col > 256 && self.col < 321 {
            // During those cycles sprites are actually fetched for rendering in the next line
            match self.col % 8 {
                // TODO : Split in 1, 3, 5, 7 to closer respect the real NES process
                1 => (),
                3 => {
                    let attribute = self.secondary_oam[self.sprite_fetcher_count * 4 + 2];
                    self.sprite_attribute_table_register.push_back(attribute);
                },
                5 => {
                    // Fetch sprite low and high byte at the same time on 7 instead of spreading over 8 cycles
                    let y_coordinate    = self.secondary_oam[self.sprite_fetcher_count * 4 + 0] as u16;
                    let tile_address    = self.secondary_oam[self.sprite_fetcher_count * 4 + 1] as u16;
                    let attribute       = *self.sprite_attribute_table_register.back().unwrap();
                    let x_coordinate    = self.secondary_oam[self.sprite_fetcher_count * 4 + 3];

                    let mut fine_y      = self.line - y_coordinate;

                    // Flipping
                    let flip_horizontally = (attribute >> 6) & 1 != 0;
                    let flip_vertically = (attribute >> 7) & 1 != 0;

                    let mut flipping_offset: u16 = 0;
                    if flip_vertically {
                        flipping_offset = 8;
                    }
                    if flip_horizontally {
                        fine_y = 7 - fine_y;
                    }

                    let chr_bank = ((self.ppuctrl as u16 >> 3) & 1) * 0x1000;
                    let low_sprite_tile_byte = self.read_ppu_memory(chr_bank + 16u16 * tile_address + fine_y + flipping_offset);

                    self.sprite_x_coordinate_table_register.push_back(x_coordinate);
                    self.sprite_low_byte_table_register.push_back(low_sprite_tile_byte);
                },
                7 => {

                    // Fetch sprite low and high byte at the same time on 7 instead of spreading over 8 cycles
                    let y_coordinate    = self.secondary_oam[self.sprite_fetcher_count * 4 + 0] as u16;
                    let tile_address    = self.secondary_oam[self.sprite_fetcher_count * 4 + 1] as u16;
                    let attribute       = *self.sprite_attribute_table_register.back().unwrap();
                    let x_coordinate    = self.secondary_oam[self.sprite_fetcher_count * 4 + 3];

                    let mut fine_y      = self.line - y_coordinate;

                    // Flipping
                    let flip_horizontally = (attribute >> 6) & 1 != 0;
                    let flip_vertically = (attribute >> 7) & 1 != 0;

                    let mut flipping_offset: u16 = 0;
                    if flip_vertically {
                        flipping_offset = 8;
                    }
                    if flip_horizontally {
                        fine_y = 7 - fine_y;
                    }

                    let chr_bank = ((self.ppuctrl as u16 >> 3) & 1) * 0x1000;

                    let mut flipping_offset: u16 = 8;
                    if flip_vertically {
                        flipping_offset = 0;
                    }

                    let high_sprite_tile_byte = self.read_ppu_memory(chr_bank + 16u16 * tile_address + fine_y + flipping_offset);
                    self.sprite_high_byte_table_register.push_back(high_sprite_tile_byte);

                    self.sprite_fetcher_count += 1;
                },
                _ => ()
            }
        }
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
        self.ppuaddr += self.get_ram_step_forward();
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
        self.ppuscroll = (self.ppuscroll << 8 ) + value as u16;
        if !self.register_w {
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
        self.ppuaddr = (self.ppuaddr << 8 ) + value as u16;
        if !self.register_w {
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
        self.ppuaddr += self.get_ram_step_forward();
    }

    fn read_or_write_0x2007(&mut self) {
        if !self.is_rendering_enabled() {
            self.register_v += self.get_ram_step_forward();
        }
        else {
            self.inc_vert_v();
            self.inc_hor_v();
        }
    }

    /// Write OAM with memory from main vram passed in value
    pub fn write_oamdma(&mut self, value: [u8]) {
        let max = 0xff - self.oamaddr;
        for i in 0..=max {
            self.primary_oam[(self.oamaddr + i) as usize] = *value[i as usize];
        }
    }

    /// Increment Horizontal part of v register
    ///
    /// Implementation base on nevdev PPU_scrolling#Wrapping around
    fn inc_hor_v(&mut self) {
        if (self.register_v & 0x1f) == 31 {
            self.register_v &= 0b111111111100000;   // hor_v = 0
            self.register_v ^= 0x400;               // switch horizontal nametable
        }
        else {
            self.register_v += 1;
        }
    }

    /// Increment Vertical part of v register
    ///
    /// Implementation base on nevdev PPU_scrolling#Wrapping around
    fn inc_vert_v(&mut self) {
        if (self.register_v & 0x7000) != 0x7000 {
            self.register_v += 0x1000;
        }
        else {
            self.register_v &= 0xfff;                                       // Fine Y = 0
            let mut coarse_y = (self.register_v & 0x3e0 ) >> 5;     // coarse_y = vert_v
            if coarse_y == 29 {
                coarse_y = 0;
                self.register_v ^= 0x800;                                   // switch vertical nametable
            }
            else if coarse_y == 31 {
                coarse_y = 0;
            }
            else {
                coarse_y += 1;
            }
            self.register_v = (self.register_v & 0b111110000011111) | (coarse_y << 5);
        }
    }

    /// Copy hor part of t to v
    fn copy_hor_t_to_hor_v(&mut self) {
        self.register_v = (self.register_v & 0b111101111100000) | (self.register_t & 0b000010000011111);
    }

    /// Copy hor part of t to v///
    fn copy_vert_t_to_vert_v(&mut self) {
        self.register_v = (self.register_v & 0b000010000011111) | (self.register_t & 0b111101111100000);
    }

    /// Return 1 is rendering is enabled, 0 otherwise
    fn is_rendering_enabled(&self) -> bool {
        self.is_bg_rendering_enabled() && self.is_sprite_rendering_enabled()
    }

    /// Return 1 is rendering is enabled, 0 otherwise
    fn is_bg_rendering_enabled(&self) -> bool {
        (self.ppumask >> 3) & 1 != 0
    }

    /// Return 1 is rendering is enabled, 0 otherwise
    fn is_sprite_rendering_enabled(&self) -> bool {
        (self.ppumask >> 4) & 1 != 0
    }

    /// RAM step foward on bus access depending on PPUCTRL bu 1
    fn get_ram_step_forward(&self) -> u16 {
        if (self.ppuctrl >> 2) & 1 == 0 {1} else {0x20}
    }

    /// RAM step foward on bus access depending on PPUCTRL bu 1
    fn is_nmi_bit_set(&self) -> bool {
        (self.ppuctrl >> 7) & 1 != 0
    }

    // https://wiki.nesdev.org/w/index.php?title=PPU_registers
    // https://bugzmanov.github.io/nes_ebook/chapter_6_4.html

    /// Set vblank bit in ppustatus register
    /// TODO : Vlbank status should be cleared after reading by CPU
    fn set_vblank(&mut self) {
        self.ppustatus |= 0b10000000;
    }

    /// Clear vblank bit in ppustatus register
    fn clear_vblank(&mut self) {
        self.ppustatus &= 0b11111111;
    }

    /// Set sprite 0 bit in ppustatus register
    fn set_sprite0_hit(&mut self) {
        self.ppustatus |= 0b01000000;
    }

    /// Clear sprite 0 bit in ppustatus register
    fn clear_sprite0_hit(&mut self) {
        self.ppustatus &= 0b10111111;
    }

    /// Set sprite overflow bit in ppustatus register
    fn set_sprite_overflow(&mut self) {
        self.ppustatus |= 0b00100000;
    }

    /// Clear sprite overflow bit in ppustatus register
    fn clear_sprite_overflow(&mut self) {
        self.ppustatus &= 0b11011111;
    }

    /// Return a dictionnary containing the current PPU Status. Usefull for debugging
    pub fn get_status(&self) -> Status {
        Status {
            col: self.col,
            line: self.line,
        }
    }
}

impl Ppu {
    // Return a color_index which is a palette index
    pub fn compute_next_pixel(&mut self) -> u8 {
        let (bg_color_code, bg_color_palette) = self.compute_bg_pixel();
        let (sprite_color_code, sprite_color_palette, priority) = self.compute_sprite_pixel();

        self.multiplexer_decision(bg_color_code, bg_color_palette, sprite_color_code, sprite_color_palette, priority)
    }

    /// Compute the elements for the bg pixel
    fn compute_bg_pixel(&mut self) -> (u8, u8) {
        let mut fine_x = (self.col - 1) % 8 + self.register_x as u16; // Pixel 0 is outputed at col == 1
        let mut register_level = 0;
        if fine_x > 7 {
            register_level += 1;
            fine_x -= 8;
        }

        let bit1 = (self.bg_low_byte_table_register[register_level] >> (7-fine_x)) & 1;
        let bit2 = (self.bg_high_byte_table_register[register_level] >> (7-fine_x)) & 1;
        let bg_color_code = bit1 | (bit2 << 1);

        let attribute = self.bg_attribute_table_register[register_level];

        // la position réelle x et y dépendent du coin en haut à gauche défini par register_t + fine x ou y  + la position réelle sur l'écran
        let shift_x = (self.register_t & 0x1f) + (self.col - 1) + self.register_x as u16;
        let shift_y = ((self.register_t & 0x3e0) >> 5) + self.line + ((self.register_t & 0x7000) >> 12);

        // Compute which zone to select in the attribute byte
        let shift = ((if shift_x % 32 > 15 {1} else {0}) + (if shift_y % 32 > 15 {2} else {0})) * 2;
        let bg_color_palette = (attribute >> shift) & 0b11;
        (bg_color_code, bg_color_palette)
    }

    /// Compute the elements for the sprite pixel if there is one at that position
    fn compute_sprite_pixel(&mut self) -> (u8, u8, u8) {
        for i in 0..(self.sprite_x_coordinate_table_register.len()) {
            let sprite_x = self.sprite_x_coordinate_table_register[i] as u16;
            // TODO : self.col must only wrok where no scrolling, use register_v instead ?
            if self.col >= sprite_x && self.col < sprite_x + 8 {
                let x_offset = self.col % 8;
                let bit1 = (self.sprite_low_byte_table_register[i as usize] >> (7-x_offset)) & 1;
                let bit2 = (self.sprite_high_byte_table_register[i as usize] >> (7-x_offset)) & 1;
                let sprite_color_code = bit1 | (bit2 << 1);

                let attribute = self.sprite_attribute_table_register[i as usize];
                let priority = (attribute >> 5) & 0x1;
                let sprite_color_palette = attribute & 0b11;

                return (sprite_color_code, sprite_color_palette, priority);
            }
        }
        (0, 0, 1) // Means no sprite, transparente color
    }

    /// Implement PPU Priority Multiplexer decision table
    fn multiplexer_decision(&mut self, bg_color_code: u8, bg_color_palette: u8, sprite_color_code: u8, sprite_color_palette: u8, priority: u8) -> u8 {
        let bg_palette_address = bg_color_palette << 2;
        let sprite_palette_address = sprite_color_palette << 2;

        if bg_color_code == 0 && sprite_color_code == 0 {
            return self.palette_vram[0]; // Palette BG Color
        }
        if bg_color_code == 0 && sprite_color_code > 0 {
            return self.palette_vram[(0x10 + sprite_palette_address + sprite_color_code) as usize]; // Sprite color > 0
        }
        if sprite_color_code == 0 {
            return self.palette_vram[(bg_palette_address + bg_color_code) as usize]; // bg color
        }
        if priority == 0 {
            return self.palette_vram[(0x10 + sprite_palette_address + sprite_color_code) as usize];
        }
        self.palette_vram[(bg_palette_address + bg_color_code) as usize] // bg color
    }

    /// Shift registers every 8 cycles
    pub fn shift_registers(&mut self) {
        self.bg_low_byte_table_register.pop_front();
        self.bg_high_byte_table_register.pop_front();
        self.bg_attribute_table_register.pop_front();
        self.bg_nt_table_register.pop_front();
    }

    /// Reset the sprite registers
    pub fn clear_sprite_registers(&mut self) {
        self.sprite_low_byte_table_register.clear();
        self.sprite_high_byte_table_register.clear();
        self.sprite_attribute_table_register.clear();
        self.sprite_x_coordinate_table_register.clear();
    }

    /// Set nt_byte into registers
    pub fn set_nt_byte(&mut self, nt_byte: u8) {
        self.bg_nt_table_register.push_back(nt_byte);
    }

    /// Set at_byte into registers
    pub fn set_at_byte(&mut self, at_byte: u8) {
        self.bg_attribute_table_register.push_back(at_byte);
    }

    /// Set low_bg_tile_byte into registers
    pub fn set_low_bg_tile_byte(&mut self, low_bg_tile_byte: u8) {
        self.bg_low_byte_table_register.push_back(low_bg_tile_byte);
    }

    /// Set high_bg_tile_byte into registers
    pub fn set_high_bg_tile_byte(&mut self, high_bg_tile_byte: u8) {
        self.bg_high_byte_table_register.push_back(high_bg_tile_byte);
    }
}