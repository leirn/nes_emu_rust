//! Cartridge object
use log::{info, trace, warn};
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

pub struct Cartridge {
    pub file_name: String,
    magic: Vec<u8>,
    prg_rom_size: usize,
    chr_rom_size: usize,
    f6: u8,
    is_trainer: bool,
    f7: u8,
    is_playchoice: bool,
    prg_ram_size: usize,
    f9: u8,
    f10: u8,

    prg_rom: Vec<u8>,
    chr_rom: Vec<u8>,
    prg_ram: Vec<u8>,
    trainer: Vec<u8>,
    mapper_id: u16,
    // mapper is last cause size is unknown at compile time
    //mapper: Box<dyn mapper::Mapper>,
}

impl Cartridge {
    /// Instantiate a new cartridge
    pub fn new(rom_file: String) -> Cartridge {
        let mut cartridge = Cartridge {
            file_name: String::new(),
            magic: vec![],
            prg_rom_size: 0,
            chr_rom_size: 0,
            f6: 0,
            is_trainer: false,
            f7: 0,
            is_playchoice: false,
            prg_ram_size: 0,
            f9: 0,
            f10: 0,

            prg_rom: vec![],
            chr_rom: vec![],
            prg_ram: vec![],
            trainer: vec![],
            mapper_id: 0, /*
                          mapper: Box::new(
                              mapper0::Mapper0::new()
                          )*/
        };
        cartridge.parse_rom(rom_file);
        cartridge
    }

    /// Parse a rom
    pub fn parse_rom(&mut self, file_name: String) {
        info!("Attemp to read file : {}", file_name);
        let file = File::open(file_name).unwrap();
        let mut buf_reader = BufReader::new(file);
        buf_reader = self.parse_header(buf_reader);

        info!("PRG Rom size is {}", self.prg_rom_size);

        self.prg_rom = vec![];
        self.chr_rom = vec![];
        self.prg_ram = vec![];

        if self.is_trainer {
            buf_reader
                .by_ref()
                .take(512)
                .read_to_end(&mut self.trainer)
                .expect("File too short, check your file for error");
        }

        buf_reader
            .by_ref()
            .take(self.prg_rom_size as u64)
            .read_to_end(&mut self.prg_rom)
            .expect("File too short, check your file for error");
        buf_reader
            .by_ref()
            .take(self.chr_rom_size as u64)
            .read_to_end(&mut self.chr_rom)
            .expect("File too short, check your file for error");

        // Mapper 0 trick
        if self.prg_rom_size == 0x4000 {
            info!("PRG Rom extension");
            let tmp_vec = self.prg_rom.clone();
            self.prg_rom.extend(tmp_vec.iter());
            self.prg_rom_size = self.prg_rom.len();
            assert_eq!(self.prg_rom[0], self.prg_rom[0x4000]);
        }
        if self.chr_rom_size == 0x1000 {
            let tmp_vec = self.chr_rom.clone();
            self.chr_rom.extend(tmp_vec.iter());
        }

        //self.file_name = file_name.clone();
        info!("{}", self.file_name)
    }

    /// Parse ROM header
    fn parse_header(&mut self, mut buf_reader: BufReader<File>) -> BufReader<File> {
        buf_reader
            .by_ref()
            .take(4)
            .read_to_end(&mut self.magic)
            .expect("File too short, check your file for error");

        info!(
            "Magic is {}-{}-{}-{}",
            self.magic[0], self.magic[1], self.magic[2], self.magic[3]
        );
        let mut tmp: Vec<u8> = vec![];
        buf_reader
            .by_ref()
            .take(1)
            .read_to_end(&mut tmp)
            .expect("File too short, check your file for error");
        info!("Byte for prg_rom_size is {}", tmp[0]);
        self.prg_rom_size = (tmp[0] as usize) * 16 * 1024;
        let mut tmp: Vec<u8> = vec![];
        buf_reader
            .by_ref()
            .take(1)
            .read_to_end(&mut tmp)
            .expect("File too short, check your file for error");
        self.chr_rom_size = (tmp[0] as usize) * 8 * 1024;
        let mut tmp: Vec<u8> = vec![];
        buf_reader
            .by_ref()
            .take(1)
            .read_to_end(&mut tmp)
            .expect("File too short, check your file for error");
        self.f6 = tmp[0];
        self.is_trainer = ((self.f6 >> 3) & 1) != 0;
        let mut tmp: Vec<u8> = vec![];
        buf_reader
            .by_ref()
            .take(1)
            .read_to_end(&mut tmp)
            .expect("File too short, check your file for error");
        self.f7 = tmp[0];
        self.is_playchoice = ((self.f7 >> 2) & 1) != 0;
        let mut tmp: Vec<u8> = vec![];
        buf_reader
            .by_ref()
            .take(1)
            .read_to_end(&mut tmp)
            .expect("File too short, check your file for error");
        self.prg_ram_size = (tmp[0] as usize) * 8 * 1024;
        let mut tmp: Vec<u8> = vec![];
        buf_reader
            .by_ref()
            .take(1)
            .read_to_end(&mut tmp)
            .expect("File too short, check your file for error");
        self.f9 = tmp[0];
        let mut tmp: Vec<u8> = vec![];
        buf_reader
            .by_ref()
            .take(1)
            .read_to_end(&mut tmp)
            .expect("File too short, check your file for error");
        self.f10 = tmp[0];
        // Read header padding bytes
        let mut tmp: Vec<u8> = vec![];
        buf_reader
            .by_ref()
            .take(5)
            .read_to_end(&mut tmp)
            .expect("File too short, check your file for error");

        self.mapper_id = (self.f7 as u16 & 0b11110000) + ((self.f6 as u16 & 0b11110000) >> 4);

        buf_reader
    }
    /*
    /// Read cartridge RAM
    pub fn read_ram(&self, address: u16) -> u8 {
        self.mapper.read_ram(address)
    }

    /// Read cartridge PRG ROM
    pub fn read_prg_rom(&self, address: u16) -> u8 {
        self.mapper.read_prg_rom(address)
    }

    /// Read cartridge CHR ROM
    pub fn read_chr_rom(&self, address: u16) -> u8 {
        self.mapper.read_chr_rom(address)
    }

    /// Write cartridge RAM
    pub fn write_ram(&self, address: u16, value: u8) {
        self.mapper.write_ram(address, value);
    }

    /// Write cartridge PRG ROM
    pub fn write_prg_rom(&self, address: u16, value: u8) {
        self.mapper.write_prg_rom(address, value);
    }

    /// Write cartridge CHR ROM
    pub fn write_chr_rom(&self, address: u16, value: u8) {
        self.mapper.write_chr_rom(address, value);
    }
    */
}

impl Cartridge {
    /// Read cartridge RAM
    pub fn read_ram(&mut self, address: u16) -> u8 {
        self.prg_ram[address as usize]
    }

    /// Read cartridge PRG ROM
    pub fn read_prg_rom(&mut self, address: u16) -> u8 {
        self.prg_rom[address as usize]
    }

    /// Read cartridge CHR ROM
    pub fn read_chr_rom(&mut self, address: u16) -> u8 {
        self.chr_rom[address as usize]
    }

    /// Write cartridge RAM
    pub fn write_ram(&mut self, address: u16, value: u8) {
        self.prg_ram[address as usize] = value;
    }

    /// Write cartridge PRG ROM
    pub fn write_prg_rom(&mut self, address: u16, value: u8) {
        self.prg_rom[address as usize] = value;
    }

    /// Write cartridge CHR ROM
    pub fn write_chr_rom(&mut self, address: u16, value: u8) {
        self.chr_rom[address as usize] = value;
    }
}
