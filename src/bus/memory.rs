//! Bus and CPU RAM component
use crate::apu::Apu;
use crate::bus::controller::Controller;
use crate::cartridge::Cartridge;
use crate::ppu::Ppu;
use std::cell::RefCell;
use std::rc::Rc;

use super::interrupt::Interrupt;

pub struct Bus {
    internal_ram: [u8; 0x800],
    pub apu: Apu,
    pub ppu: Ppu,
    pub controller_1: Controller,
    pub controller_2: Controller,
    controller_1_status: u8,
    controller_2_status: u8,
    cartridge: Rc<RefCell<Cartridge>>,
    pub interrupt: Rc<RefCell<Interrupt>>,
}

impl Bus {
    /// Instantiate new Memory component
    pub fn new(_sdl_context: Rc<RefCell<sdl2::Sdl>>, _cartridge: Rc<RefCell<Cartridge>>) -> Bus {
        let _interrupt = Rc::new(RefCell::new(Interrupt::new()));
        Bus {
            internal_ram: [0; 0x800], // 2kB or internal RAM
            apu: Apu::new(_sdl_context.clone(), _interrupt.clone()),
            ppu: Ppu::new(_sdl_context.clone(), _cartridge.clone(), _interrupt.clone()),
            cartridge: _cartridge,
            controller_1: Controller::new(),
            controller_2: Controller::new(),
            controller_1_status: 0,
            controller_2_status: 0,
            interrupt: _interrupt,
        }
    }

    /// Read 16-bit little endian address from memory
    pub fn read_rom_16(&mut self, address: u16) -> u16 {
        let high;
        let low;
        if address > 0x7fff {
            low = self.cartridge.borrow_mut().read_prg_rom(address - 0x8000);
            high = self
                .cartridge
                .borrow_mut()
                .read_prg_rom(address + 1 - 0x8000);
        } else {
            low = self.internal_ram[address as usize];
            high = self.internal_ram[(address + 1) as usize]; // So that reading never cross pages
        }
        low as u16 + ((high as u16) << 8)
    }

    /// Read 16-bit little endian address from memory without crossing memory page
    pub fn read_rom_16_no_crossing_page(&mut self, address: u16) -> u16 {
        let high_address = (address & 0xFF00) + ((address + 1) & 0xff);
        let high;
        let low;
        if address > 0x7fff {
            low = self.cartridge.borrow_mut().read_prg_rom(address - 0x8000);
            high = self
                .cartridge
                .borrow_mut()
                .read_prg_rom(high_address - 0x8000);
        } else {
            low = self.internal_ram[address as usize];
            high = self.internal_ram[high_address as usize]; // So that reading never cross pages
        }
        low as u16 + ((high as u16) << 8)
    }

    /// Lecture de la mémoire, à restucturer comme suit:
    /// 0x0000 to 0x1fff : internal ram
    /// 0x2000 to 0x3fff : PPU registers
    /// 0x4000 to 0x4017 : APU and I/O registers
    /// 0x4018 to 0x401f : APU and I/O funcitonality normally disabled
    /// 0x4020 to 0x5fff : Cartridge space but for what ??
    /// 0x6000 to 0x7fff : Cartridge ram
    /// 0x8000 to 0xffff : Cartridge prg_rom
    pub fn read_rom(&mut self, address: u16) -> u8 {
        match address {
            0..=0x1fff => self.internal_ram[(address % 0x800) as usize],
            0x2000..=0x3fff => {
                let local_address = 0x2000 + (address % 8);
                match local_address {
                    0x2002 => self.ppu.read_0x2002(),
                    0x2004 => self.ppu.read_0x2004(),
                    0x2007 => self.ppu.read_0x2007(),
                    _ => panic!("Write only address : {}", address),
                }
            }
            0x4000..=0x4017 => {
                match address {
                    // Read input 1
                    0x4016 => {
                        let value = self.controller_1_status & 1;
                        self.controller_1_status >>= 1;
                        value
                    }
                    // Read input 2
                    0x4017 => {
                        let value = self.controller_2_status & 1;
                        self.controller_2_status >>= 1;
                        value
                    }
                    // Read APU
                    _ => self.apu.read_registers(address),
                }
            }
            0x4018..=0x401f => 0, // Normally disabled
            0x4020..=0x5fff => 0, // Cartridge space, but for what ?
            0x6000..=0x7fff => self.cartridge.borrow_mut().read_ram(address - 0x6000),
            0x8000..=0xffff => self.cartridge.borrow_mut().read_prg_rom(address - 0x8000),
        }
    }

    /// Ecriture de la mémoire, à restucturer comme suit:
    /// 0x0000 to 0x1fff : internal ram
    /// 0x2000 to 0x3fff : PPU registers
    /// 0x4000 to 0x4017 : APU and I/O registers
    /// 0x4018 to 0x401f : APU and I/O funcitonality normally disabled
    /// 0x4020 to 0x5fff : Cartridge space but for what ??
    /// 0x6000 to 0x7fff : Cartridge ram
    /// 0x8000 to 0xffff : Cartridge prg_rom
    pub fn write_rom(&mut self, address: u16, value: u8) -> u32 {
        match address {
            0..=0x1fff => self.internal_ram[(address % 0x800) as usize] = value,
            0x2000..=0x3fff => {
                let local_address = 0x2000 + (address % 8);
                match local_address {
                    0x2000 => self.ppu.write_0x2000(value),
                    0x2001 => self.ppu.write_0x2001(value),
                    0x2002 => panic!("Read only address {}", address), // Read-only
                    0x2003 => self.ppu.write_0x2003(value),
                    0x2004 => self.ppu.write_0x2004(value),
                    0x2005 => self.ppu.write_0x2005(value),
                    0x2006 => self.ppu.write_0x2006(value),
                    0x2007 => self.ppu.write_0x2007(value),
                    _ => (), // won"t happen based on local_address computation
                }
            }
            0x4000..=0x4017 => {
                match address {
                    // OAMDMA
                    0x4014 => {
                        let start = (value as usize) << 8;
                        let end = start + 0x100 - 1;
                        self.ppu.write_oamdma(&self.internal_ram[start..=end]);
                        return 514;
                    }
                    // Save inputs 1 and 2
                    0x4016 => {
                        if value & 1 == 0 {
                            self.controller_1_status = self.controller_1.get_status();
                            self.controller_2_status = self.controller_2.get_status();
                        }
                    }
                    // Read APU
                    _ => self.apu.write_registers(address, value),
                }
            }
            0x4018..=0x401f => (), // Normally disabled
            0x4020..=0x5fff => (), // Cartridge space, but for what ?
            0x6000..=0x7fff => self
                .cartridge
                .borrow_mut()
                .write_ram(address - 0x6000, value),
            0x8000..=0xffff => self
                .cartridge
                .borrow_mut()
                .write_prg_rom(address - 0x8000, value),
        }
        0
    }

    /// Get xor CRC of zero page memory
    pub fn xor_zero_page(&self) -> u8 {
        let mut xor = 0;
        for i in self.internal_ram[0..=255].iter() {
            xor ^= i;
        }
        xor
    }

    /// Print 0x20 long memory chunk
    pub fn _get_memory_as_string(&self, address: u16) -> String {
        let address = address as usize;
        format!("{:04x}:{:04x}    {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x}",
            address,
            address + 0x1f,
            self.internal_ram[address],
            self.internal_ram[address+1],
            self.internal_ram[address+2],
            self.internal_ram[address+3],
            self.internal_ram[address+4],
            self.internal_ram[address+5],
            self.internal_ram[address+6],
            self.internal_ram[address+7],
            self.internal_ram[address+8],
            self.internal_ram[address+9],
            self.internal_ram[address+10],
            self.internal_ram[address+11],
            self.internal_ram[address+12],
            self.internal_ram[address+13],
            self.internal_ram[address+14],
            self.internal_ram[address+15],
            self.internal_ram[address+16],
            self.internal_ram[address+17],
            self.internal_ram[address+18],
            self.internal_ram[address+19],
            self.internal_ram[address+20],
            self.internal_ram[address+21],
            self.internal_ram[address+22],
            self.internal_ram[address+23],
            self.internal_ram[address+24],
            self.internal_ram[address+25],
            self.internal_ram[address+26],
            self.internal_ram[address+27],
            self.internal_ram[address+28],
            self.internal_ram[address+29],
            self.internal_ram[address+30],
            self.internal_ram[address+31],
        )
    }
}
