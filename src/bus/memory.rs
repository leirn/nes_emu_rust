//! Bus and CPU RAM component
use std::cell::RefCell;
use std::rc::Rc;
use crate::apu::Apu;
use crate::ppu::Ppu;
use crate::cartridge::Cartridge;
use crate::bus::controller::Controller;

pub struct Memory {
    internal_ram: [u8; 0x800],
    apu: Rc<RefCell<Apu>>,
    ppu: Rc<RefCell<Ppu>>,
    controller_1: Rc<RefCell<Controller>>,
    controller_2: Rc<RefCell<Controller>>,
    controller_1_status: u8,
    controller_2_status: u8,
    cartridge: Rc<RefCell<Cartridge>>,
}

impl Memory {
    /// Instantiate new Memory component
    pub fn new(_cartridge: Rc<RefCell<Cartridge>>, _ppu: Rc<RefCell<Ppu>>, _apu: Rc<RefCell<Apu>>, _controller_1: Rc<RefCell<Controller>>, _controller_2: Rc<RefCell<Controller>>) -> Memory {
        Memory {
            internal_ram: [0; 0x800], // 2kB or internal RAM
            apu: _apu,
            ppu: _ppu,
            cartridge: _cartridge,
            controller_1: _controller_1,
            controller_2: _controller_2,
            controller_1_status: 0,
            controller_2_status: 0,
        }
    }

    /// Read 16-bit little endian address from memory
    pub fn read_rom_16(&mut self, address: u16) -> u16 {
        let mut high = 0;
        let mut low = 0;
        if address > 0x7fff {
            low = self.cartridge.borrow_mut().read_prg_rom(address - 0x8000);
            high = self.cartridge.borrow_mut().read_prg_rom(address + 1 - 0x8000);
        }
        else {
            low = self.internal_ram[address as usize];
            high = self.internal_ram[(address + 1) as usize]; // So that reading never cross pages
        }
        low as u16 + ((high as u16) <<8)
    }

    /// Read 16-bit little endian address from memory without crossing memory page
    pub fn read_rom_16_no_crossing_page(&mut self, address: u16) -> u16 {
        let high_address = (address & 0xFF00) +((address + 1) & 0xff);
        let mut high = 0;
        let mut low = 0;
        if address > 0x7fff {
            low = self.cartridge.borrow_mut().read_prg_rom(address - 0x8000);
            high = self.cartridge.borrow_mut().read_prg_rom(high_address - 0x8000);
        }
        else {
            low = self.internal_ram[address as usize];
            high = self.internal_ram[high_address as usize]; // So that reading never cross pages
        }
        low as u16 + ((high as u16) <<8)
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
                    0x2000 => 0,
                    0x2001 => 0,
                    0x2002 => self.ppu.borrow_mut().read_0x2002(),
                    0x2003 => 0,
                    0x2004 => self.ppu.borrow_mut().read_0x2004(),
                    0x2005 => 0,
                    0x2006 => 0,
                    0x2007 => self.ppu.borrow_mut().read_0x2007(),
                    _ => 0, // won"t happen based on local_address computation
                }
            },
            0x4000..=0x4017 => {
                match address {
                    // Read input 1
                    0x4016 => {
                        let value = self.controller_1_status & 1;
                        self.controller_1_status >>= 1;
                        return value;
                    },
                    // Read input 2
                    0x4017 => {
                        let value = self.controller_2_status & 1;
                        self.controller_2_status >>= 1;
                        return value;
                    },
                    // Read APU
                    _ => self.apu.borrow_mut().read_registers(address),
                }
            },
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
                    0x2000 => self.ppu.borrow_mut().write_0x2000(value),
                    0x2001 => self.ppu.borrow_mut().write_0x2001(value),
                    0x2002 => (), // Read-only
                    0x2003 => self.ppu.borrow_mut().write_0x2003(value),
                    0x2004 => self.ppu.borrow_mut().write_0x2004(value),
                    0x2005 => self.ppu.borrow_mut().write_0x2005(value),
                    0x2006 => self.ppu.borrow_mut().write_0x2006(value),
                    0x2007 => self.ppu.borrow_mut().write_0x2007(value),
                    _ => (), // won"t happen based on local_address cimputation
                }
            },
            0x4000..=0x4017 => {
                match address {
                    // Save inputs 1 and 2
                    0x4016 => {
                        if value & 1 == 0 {
                            self.controller_1_status = self.controller_1.borrow_mut().get_status();
                            self.controller_2_status = self.controller_2.borrow_mut().get_status();
                        }
                    },
                    // OAMDMA
                    0x4014 => {
                        let start = address as usize;
                        let end = start + 0x100;
                        self.ppu.borrow_mut().write_oamdma(self.internal_ram[start..end]);
                        return 514;
                    },
                    // Read APU
                    _ => self.apu.borrow_mut().write_registers(address, value),
                }
            },
            0x4018..=0x401f => (), // Normally disabled
            0x4020..=0x5fff => (), // Cartridge space, but for what ?
            0x6000..=0x7fff => self.cartridge.borrow_mut().write_ram(address - 0x6000, value),
            0x8000..=0xffff => self.cartridge.borrow_mut().write_prg_rom(address - 0x8000, value),
        }
        0
    }
}
