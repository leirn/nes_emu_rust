//! Emulator main engine
mod clock;
use log::info;
use regex::Regex;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::cell::RefCell;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Write;
use std::rc::Rc;

use crate::cartridge::Cartridge;
use crate::cpu::instructions::INSTRUCTION_TABLE;
use crate::cpu::opcodes::OPCODES;
use crate::cpu::Cpu;

pub struct NesEmulator<'a> {
    pause: bool,
    is_test_mode: bool,
    pub sdl_context: Rc<RefCell<sdl2::Sdl>>,
    clock: clock::Clock,
    _cartridge: Rc<RefCell<Cartridge>>,
    cpu: Cpu<'a>,
    lines: Vec<String>,
    line_index: usize,
    parity: bool,
    log_activated: bool,
    log_file: Option<std::io::BufWriter<std::fs::File>>,
}

impl NesEmulator<'_> {
    /// Instantiate the Emulator
    pub fn new(rom_file: String) -> NesEmulator<'static> {
        let _sdl_context = Rc::new(RefCell::new(sdl2::init().unwrap()));
        info!("SDL Context initialized");

        let _cartridge = Rc::new(RefCell::new(Cartridge::new(rom_file)));
        let _cpu = Cpu::new(_sdl_context.clone(), _cartridge.clone());

        NesEmulator {
            pause: false,
            is_test_mode: false,
            sdl_context: _sdl_context,
            clock: clock::Clock::new(60), // 60 fps target
            _cartridge: _cartridge,
            cpu: _cpu,
            lines: vec![],
            line_index: 0,
            parity: false,
            log_activated: false,
            log_file: None,
        }
    }

    /// Starts and runs the Emulator execution
    pub fn start(&mut self, entry_point: Option<u16>) {
        self.log_activated = false;
        if self.log_activated {
            self.log_file = Some(BufWriter::new(
                File::create(r#"C:\Users\lvromman\Documents\GitHub\nes_emu_rust\log.txt"#)
                    .expect("Cannot create message"),
            ));
        }
        self.cpu.bus.apu.start();
        self.cpu.bus.ppu.start();
        self.cpu.start(entry_point);
        self.cpu.bus.ppu.next();
        self.cpu.bus.ppu.next();
        self.cpu.bus.ppu.next();

        let mut continuer: bool = true;

        while continuer {
            if !self.pause {
                if self.cpu.bus.interrupt.borrow_mut().check_and_clear_nmi() {
                    self.cpu.nmi();
                }
                if self.cpu.bus.interrupt.borrow_mut().check_and_clear_irq()
                    && self.cpu.get_interrupt_flag()
                {
                    self.cpu.irq();
                }
                if self.parity {
                    self.cpu.bus.apu.next();
                }
                self.cpu.next();
                self.cpu.bus.ppu.next();
                self.cpu.bus.ppu.next();
                self.cpu.bus.ppu.next();

                // Odd or even cycle. Needed to trigger the apu one every two cpu cycles.
                self.parity = !self.parity;

                if self.is_test_mode && self.cpu.get_remaining_cycles() == 0 {
                    let cpu_status = self.cpu.get_status();
                    let ppu_status = self.cpu.bus.ppu.get_status();
                    self.check_test(cpu_status, ppu_status);
                }

                if self.log_activated
                    && self.log_file.is_some()
                    && self.cpu.get_remaining_cycles() == 0
                {
                    let log = self.get_status_log();
                    //self.log("{}", log);
                    self.log_file
                        .as_mut()
                        .unwrap()
                        .write_all(log.as_bytes())
                        .unwrap();
                    self.log_file.as_mut().unwrap().write_all(b"\n").unwrap();

                    //if self.cpu.get_total_cycles() > 87529 {
                    if self.clock.get_clock_count() > 10 {
                        self.cpu.bus.ppu.print_primary_oam();
                        self.cpu.bus.ppu.print_secondary_oam();
                        std::process::exit(0);
                    }
                }

                if self
                    .cpu
                    .bus
                    .interrupt
                    .borrow_mut()
                    .check_and_clear_frame_updated()
                {
                    self.clock.tick();
                    info!("FPS : {}", self.clock.get_fps());
                }
            }

            let mut event_pump = self.sdl_context.borrow_mut().event_pump().unwrap();
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Q),
                        ..
                    } => continuer = false,
                    Event::KeyDown {
                        keycode: Some(Keycode::P),
                        ..
                    } => self.toggle_pause(),
                    Event::KeyDown {
                        keycode: Some(Keycode::S),
                        ..
                    } => println!("{}", self.get_status_log()),
                    Event::KeyDown {
                        keycode: Some(Keycode::Up),
                        ..
                    } => self.cpu.bus.controller_1.set_up(),
                    Event::KeyUp {
                        keycode: Some(Keycode::Up),
                        ..
                    } => self.cpu.bus.controller_1.clear_up(),
                    Event::KeyDown {
                        keycode: Some(Keycode::Down),
                        ..
                    } => self.cpu.bus.controller_1.set_down(),
                    Event::KeyUp {
                        keycode: Some(Keycode::Down),
                        ..
                    } => self.cpu.bus.controller_1.clear_down(),
                    Event::KeyDown {
                        keycode: Some(Keycode::Left),
                        ..
                    } => self.cpu.bus.controller_1.set_left(),
                    Event::KeyUp {
                        keycode: Some(Keycode::Left),
                        ..
                    } => self.cpu.bus.controller_1.clear_left(),
                    Event::KeyDown {
                        keycode: Some(Keycode::Right),
                        ..
                    } => self.cpu.bus.controller_1.set_right(),
                    Event::KeyUp {
                        keycode: Some(Keycode::Right),
                        ..
                    } => self.cpu.bus.controller_1.clear_right(),
                    Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => self.cpu.bus.controller_1.set_select(),
                    Event::KeyUp {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => self.cpu.bus.controller_1.clear_select(),
                    Event::KeyDown {
                        keycode: Some(Keycode::Return),
                        ..
                    } => self.cpu.bus.controller_1.set_start(),
                    Event::KeyUp {
                        keycode: Some(Keycode::Return),
                        ..
                    } => self.cpu.bus.controller_1.clear_start(),
                    Event::KeyDown {
                        keycode: Some(Keycode::Space),
                        ..
                    } => self.cpu.bus.controller_1.set_a(),
                    Event::KeyUp {
                        keycode: Some(Keycode::Space),
                        ..
                    } => self.cpu.bus.controller_1.clear_a(),
                    Event::KeyDown {
                        keycode: Some(Keycode::LCtrl),
                        ..
                    } => self.cpu.bus.controller_1.set_b(),
                    Event::KeyUp {
                        keycode: Some(Keycode::LCtrl),
                        ..
                    } => self.cpu.bus.controller_1.clear_b(),
                    _ => (),
                }
            }
        }
    }

    fn get_status_log(&mut self) -> String {
        let cpu_status = self.cpu.get_status();
        let ppu_status = self.cpu.bus.ppu.get_status();
        let ppu_full_status = self.cpu.bus.ppu.get_ppustatus_as_string();
        let opcode = self.cpu.bus.read_rom(cpu_status.program_counter);
        let mut opcode_arg_1 = "  ".to_string();
        let mut opcode_arg_2 = "  ".to_string();
        if OPCODES[&opcode].len > 1 {
            opcode_arg_1 = format!(
                "{:02x}",
                self.cpu.bus.read_rom(cpu_status.program_counter + 1)
            );
        }
        if OPCODES[&opcode].len > 2 {
            opcode_arg_2 = format!(
                "{:02x}",
                self.cpu.bus.read_rom(cpu_status.program_counter + 2)
            );
        }

        let zero_page_xor = self.cpu.bus.xor_zero_page();

        format!("{:x}  {:02x} {} {}  {:30}  A:{:02x} X:{:02x} Y:{:02x} P:{:02x} SP:{:02x} PPU:{},{} CYC:{}, ZeroPage:{:02x},{}",
            cpu_status.program_counter,
            opcode,
            opcode_arg_1,
            opcode_arg_2,
            INSTRUCTION_TABLE[opcode as usize].get_syntax(&mut self.cpu),
            cpu_status.accumulator,
            cpu_status.x_register,
            cpu_status.y_register,
            cpu_status.status_register,
            cpu_status.stack_pointer,
            ppu_status.line,
            ppu_status.col,
            cpu_status.total_cycles,
            zero_page_xor,
            ppu_full_status,
        )
    }

    /// Toggles pause on the emulator execution
    pub fn toggle_pause(&mut self) {
        self.pause = !self.pause;
    }

    /// Activate test mode and set the execution reference file
    pub fn set_test_mode(&mut self, file_name: &str) {
        self.is_test_mode = true;
        let test_file = File::open(file_name).unwrap();
        let buffer = BufReader::new(test_file);
        for line in buffer.lines() {
            self.lines.push(line.unwrap());
        }
    }

    /// Performs test execution against reference execution log to find descrepancies
    fn check_test(&mut self, cpu_status: crate::cpu::Status, ppu_status: crate::ppu::Status) {
        if self.line_index == self.lines.len() {
            info!("Test déroulé sans erreur ! SUCCESS !!!");
            std::process::exit(0);
        }
        let current_line = self.lines[self.line_index].clone();
        self.line_index += 1;

        let opcode = self.cpu.bus.read_rom(cpu_status.program_counter);
        let mut opcode_arg_1 = "  ".to_string();
        let mut opcode_arg_2 = "  ".to_string();
        if OPCODES[&opcode].len > 1 {
            opcode_arg_1 = format!(
                "{:02x}",
                self.cpu.bus.read_rom(cpu_status.program_counter + 1)
            );
        }
        if OPCODES[&opcode].len > 2 {
            opcode_arg_2 = format!(
                "{:02x}",
                self.cpu.bus.read_rom(cpu_status.program_counter + 2)
            );
        }

        let log_status = LogFileLine::new(current_line.as_str());
        info!("{}", current_line);
        info!("{:x}  {:02x} {} {}  {:30}  A:{:02x} X:{:02x} Y:{:02x} P:{:02x} SP:{:02x} PPU:{},{} CYC:{}",
            cpu_status.program_counter,
            opcode,
            opcode_arg_1,
            opcode_arg_2,
            INSTRUCTION_TABLE[opcode as usize].get_syntax(&mut self.cpu),
            cpu_status.accumulator,
            cpu_status.x_register,
            cpu_status.y_register,
            cpu_status.status_register,
            cpu_status.stack_pointer,
            ppu_status.line,
            ppu_status.col,
            cpu_status.total_cycles,
        );

        assert_eq!(cpu_status.program_counter, log_status.program_counter);
        //assert_eq!(opcode, log_status.opcode);
        assert_eq!(cpu_status.stack_pointer, log_status.stack_pointer);
        assert_eq!(cpu_status.accumulator, log_status.accumulator);
        assert_eq!(cpu_status.x_register, log_status.x_register);
        assert_eq!(cpu_status.y_register, log_status.y_register);
        assert_eq!(cpu_status.status_register, log_status.status_register);
        assert_eq!(cpu_status.total_cycles, log_status.total_cycles);
        assert_eq!(ppu_status.col, log_status.col);
        assert_eq!(ppu_status.line, log_status.line);
    }
}

struct LogFileLine {
    pub program_counter: u16,
    pub _opcode: u8,
    pub stack_pointer: u8,
    pub accumulator: u8,
    pub x_register: u8,
    pub y_register: u8,
    pub status_register: u8,
    pub total_cycles: u32,
    pub col: u16,
    pub line: u16,
}

impl LogFileLine {
    fn new(line: &str) -> LogFileLine {
        let re = Regex::new(r"A:(?P<A>[0-9A-Fa-f]{2}) X:(?P<X>[0-9A-Fa-f]{2}) Y:(?P<Y>[0-9A-Fa-f]{2}) P:(?P<P>[0-9A-Fa-f]{2}) SP:(?P<SP>[0-9A-Fa-f]{2})").unwrap();
        let result1 = re.captures(line).unwrap();
        let re = Regex::new(r"CYC:(?P<CYC>[0-9A-Fa-f]+)").unwrap();
        let result2 = re.captures(line).unwrap();
        let re = Regex::new(r"PPU:[ ]*([0-9]+),[ ]*([0-9]+)").unwrap();
        let result3 = re.captures(line).unwrap();

        LogFileLine {
            _opcode: 0,
            program_counter: u16::from_str_radix(&line[0..4], 16).unwrap(),
            stack_pointer: u8::from_str_radix(&result1["SP"], 16).unwrap(),
            accumulator: u8::from_str_radix(&result1["A"], 16).unwrap(),
            x_register: u8::from_str_radix(&result1["X"], 16).unwrap(),
            y_register: u8::from_str_radix(&result1["Y"], 16).unwrap(),
            status_register: u8::from_str_radix(&result1["P"], 16).unwrap(),
            total_cycles: result2["CYC"].to_string().parse::<u32>().unwrap(),
            line: result3[1].to_string().parse::<u16>().unwrap(),
            col: result3[2].to_string().parse::<u16>().unwrap(),
        }
    }
}
