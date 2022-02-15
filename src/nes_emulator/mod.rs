//! Emulator main engine
mod clock;
use std::cell::RefCell;
use std::rc::Rc;
use std::fs::File;
use std::io::BufReader;
use crate::cpu::opcodes::OPCODES;
use regex::Regex;
use std::io::BufRead;

pub struct NesEmulator {
    is_nmi: bool,
    is_irq: bool,
    pause: bool,
    is_frame_updated: bool,
    is_test_mode: bool,
    pub sdl_context: Rc<RefCell<sdl2::Sdl>>,
    clock: clock::Clock,
    cartridge: Rc<RefCell<crate::cartridge::Cartridge>>,
    memory: Rc<RefCell<crate::bus::memory::Memory>>,
    apu: Rc<RefCell<crate::apu::Apu>>,
    ppu: Rc<RefCell<crate::ppu::Ppu>>,
    cpu: Rc<RefCell<crate::cpu::Cpu>>,
    interrupt_bus: Rc<RefCell<crate::bus::Interrupt>>,
    lines: Vec<String>,
    line_index: usize,
    parity: bool,
}

impl NesEmulator {
    /// Instantiate the Emulator
    pub fn new(rom_file: String) -> NesEmulator {
        let _sdl_context = Rc::new(RefCell::new(sdl2::init().unwrap()));
        println!("SDL Context initialized");


        let cartridge = Rc::new(RefCell::new(crate::cartridge::Cartridge::new(rom_file)));
        let apu = Rc::new(RefCell::new(crate::apu::Apu::new()));
        let ppu = Rc::new(RefCell::new(crate::ppu::Ppu::new(cartridge.clone(), _sdl_context.clone())));
        let memory = Rc::new(RefCell::new(crate::bus::memory::Memory::new(Rc::clone(&cartridge), Rc::clone(&ppu), Rc::clone(&apu))));
        let cpu = Rc::new(RefCell::new(crate::cpu::Cpu::new(Rc::clone(&memory))));

        let interrupt_bus: Rc::new(RefCell::new(crate::bus::Interrupt::new()));

        NesEmulator{
            is_nmi: false,
            is_irq: false,
            pause: false,
            is_frame_updated: false,
            is_test_mode: false,
            sdl_context: _sdl_context,
            clock: clock::Clock::new(),
            cartridge: cartridge,
            memory: memory,
            apu: apu,
            ppu: ppu,
            cpu: cpu,
            interrupt_bus: interrupt_bus,
            lines: vec![],
            line_index: 0,
            parity: false,
        }
    }

    /// Starts and runs the Emulator execution
    pub fn start(&mut self, entry_point: Option<u16>) {
        self.ppu.borrow_mut().start();
        self.cpu.borrow_mut().start(entry_point);
        self.ppu.borrow_mut().next();
        self.ppu.borrow_mut().next();
        self.ppu.borrow_mut().next();

        let mut continuer:bool = true;

        while continuer {
            if !self.pause {
                if self.interrupt_bus.borrow_mut().check_and_clear_nmi() {
                    self.cpu.borrow_mut().nmi();
                }
                if self.interrupt_bus.borrow_mut().check_and_clear_irq() && self.cpu.borrow_mut().get_interrupt_flag() {
                    self.cpu.borrow_mut().irq();
                }
                if self.parity {
                    self.apu.borrow_mut().next();
                }
                self.cpu.borrow_mut().next();
                self.ppu.borrow_mut().next();
                self.ppu.borrow_mut().next();
                self.ppu.borrow_mut().next();

                // Odd or even cycle. Needed to trigger the apu one every two cpu cycles.
                self.parity = !self.parity;

                let is_frame_updated = self.interrupt.borrow_mut().check_and_clear_frame_updated();
                if is_frame_updated && self.cpu.borrow_mut().get_remaining_cycles() == 0 {
                    let cpu_status = self.cpu.borrow_mut().get_status();
                    let ppu_status = self.ppu.borrow_mut().get_status();
                    self.check_test(cpu_status, ppu_status);
                }

                if is_frame_updated {
                    self.clock.tick(60);
                }
            }

            let mut event_pump = self.sdl_context.borrow_mut().event_pump().unwrap();
            for event in event_pump.poll_iter() {
                use sdl2::event::Event;
                match event {
                    Event::Quit {..} |
                    Event::KeyDown {
                        keycode: Some(sdl2::keyboard::Keycode::Q), ..
                    } => { continuer = false},
                    _ => ()
                }
            }
        }
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
        let current_line = self.lines[self.line_index].clone();
        self.line_index += 1;

        //let current_line = "C000  4C F5 C5  JMP $C5F5                       A:00 X:00 Y:00 P:24 SP:FD PPU:  0, 21 CYC:7";

        let opcode = self.memory.borrow_mut().read_rom(cpu_status.program_counter);
        let mut opcode_arg_1 = "  ".to_string();
        let mut opcode_arg_2 = "  ".to_string();
        if OPCODES[&opcode].len > 1 {
            let tmp_string = format!("{:02x}", self.memory.borrow_mut().read_rom(cpu_status.program_counter + 1));
            opcode_arg_1 = tmp_string.clone();
        }
        if OPCODES[&opcode].len > 2 {
            let tmp_string = format!("{:02x}", self.memory.borrow_mut().read_rom(cpu_status.program_counter + 2));
            opcode_arg_2 = tmp_string.clone();
        }

        let log_status = LogFileLine::new(current_line.as_str());
        println!("{}", current_line);
        println!("{:x}  {:02x} {} {}  {:30}  A:{:02x} X:{:02x} Y:{:02x} P:{:02x} SP:{:02x} PPU:{},{} CYC:{}",
            cpu_status.program_counter,
            opcode,
            opcode_arg_1,
            opcode_arg_2,
            OPCODES[&opcode].syntax,
            cpu_status.accumulator,
            cpu_status.x_register,
            cpu_status.y_register,
            cpu_status.status_register,
            cpu_status.stack_pointer,
            ppu_status.col,
            ppu_status.line,
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
        //assert_eq!(ppu_status.col, log_status.col);
        //assert_eq!(ppu_status.line, log_status.line);

        println!();

    }
}

struct LogFileLine {
    pub program_counter: u16,
    pub opcode: u8,
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
            opcode: 0,
            program_counter: u16::from_str_radix(&line[0..4], 16).unwrap(),
            stack_pointer: u8::from_str_radix(&result1["SP"], 16).unwrap(),
            accumulator: u8::from_str_radix(&result1["A"], 16).unwrap(),
            x_register: u8::from_str_radix(&result1["X"], 16).unwrap(),
            y_register: u8::from_str_radix(&result1["Y"], 16).unwrap(),
            status_register: u8::from_str_radix(&result1["P"], 16).unwrap(),
            total_cycles: result2["CYC"].to_string().parse::<u32>().unwrap(),
            col: result3[1].to_string().parse::<u16>().unwrap(),
            line: result3[2].to_string().parse::<u16>().unwrap(),
        }
    }
}