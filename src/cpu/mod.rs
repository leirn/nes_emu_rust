use crate::components::{MEMORY};
use std::collections::HashMap;

pub struct Cpu {
    // Registers
    accumulator: u8,
    x_register: u8,
    y_register: u8,
    program_counter: u16,
    stack_pointer: u16,

    // Flags
    negative: bool,
    overflow: bool,
    break_flag: bool,
    decimal: bool,
    interrupt: bool,
    zero: bool,
    carry: bool,

    // Instructions calls
    instructions: HashMap<u8, fn(&Cpu) -> (u16, u32) >,

    // Other states
    total_cycles: u32,
    remaining_cycles: u32,
    additionnal_cycles: u32,
    compteur: u32,
}

unsafe impl Sync for Cpu {}

/// Dummy function to temporarly load the instruction array


impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            accumulator: 0,
            x_register: 0,
            y_register: 0,
            program_counter: 0,
            stack_pointer: 0,
            negative: false,
            overflow: false,
            break_flag: false,
            decimal: false,
            interrupt: false,
            zero: false,
            carry: false,
            instructions: HashMap::new(),
            total_cycles: 0,
            remaining_cycles: 0,
            additionnal_cycles: 0,
            compteur: 0,
        }
    }

    /// Function to populate instruction hashmap
    /// To be completed as functions are implemented
    fn populate_instructions_vector(&mut self) {
        for i in 0..=255 {
            self.instructions.insert(i, Cpu::dummy);
        }
        self.instructions.insert(10, Cpu::dummy);
        
    }
    fn dummy(&self) -> (u16, u32) {
        (0, 0)
    }
    /// CPU initialisation function
    pub fn start(&mut self, entry_point: Option<u16>) {
        //!Execute 6502 Start sequence

        self.populate_instructions_vector();

        // Default is equivalent to JMP ($FFFC)
        self.program_counter = entry_point.unwrap_or(MEMORY.lock().unwrap().read_rom_16(0xfffc));

        //Start sequence push stack three time
        self.push(0);
        self.push(0);
        self.push(0);
        
        self.total_cycles = 7; //# Cout de match'init
        self.remaining_cycles = 7 - 1;
    }

    /// Execute the next CPU cycles.
    ///
    /// If There are remaining cycles from previous opcode execution, does noting.
    /// Otherwise, execute the next opcode
    pub fn next(&mut self) {
        if self.remaining_cycles > 0 {
            self.remaining_cycles -= 1;
        }

        let opcode:u8  = MEMORY.lock().unwrap().read_rom(self.program_counter);

        let cpu_instruction = self.instructions[&opcode];

        let (step, remaining_cycles) = cpu_instruction(self);
        self.remaining_cycles = remaining_cycles + self.additionnal_cycles;
        self.total_cycles += self.remaining_cycles;
        self.remaining_cycles -= 1; // Do not count current cycle twice
        self.additionnal_cycles = 0;
        self.program_counter += step;
        self.compteur += 1;
    }

    /// Raises an NMI interruption
    pub fn nmi(&mut self) {
        self.general_interrupt(0xfffa);
    }

    /// Raises an IRQ interruption
    pub fn irq(&mut self) {
        self.general_interrupt(0xfffe);
    }

    /// General interruption sequence used for NMI and IRQ
    /// 
    /// Interruptions last for 7 CPU cycles
    fn general_interrupt(&mut self, address: u16) { 
        self.push(((self.program_counter >> 8) & 255) as u8);
        self.push((self.program_counter & 255) as u8);
        self.push(self.get_status_register());

        self.interrupt = false;

        self.program_counter = MEMORY.lock().unwrap().read_rom_16(address);
        self.remaining_cycles = 7 - 1; // do not count current cycle twice
        self.total_cycles += 7

    }

    /// Returns the P register which contains the flag status.
    /// 
    /// Bit 5 is always set to 1
    fn get_status_register(&self) -> u8 {
        return    ((self.negative as u8) << 7) 
                | ((self.overflow as u8) << 6) 
                | (1 << 5) 
                | ((self.break_flag as u8) << 4) 
                | ((self.decimal as u8) << 3) 
                | ((self.interrupt as u8) << 2) 
                | ((self.zero as u8) << 1) 
                | (self.carry as u8);
    }

    /// Set the P register which contains the flag status.
    /// 
    /// When setting the P Register, the break flag is not set.
    fn set_status_register(&mut self, status_register: u8) {
        self.carry =        (status_register & 1) != 0;
        self.zero =         ((status_register >> 1) & 1) != 0;
        self.interrupt =    ((status_register >> 2) & 1) != 0;
        self.decimal =      ((status_register >> 3) & 1) != 0;
        //self.flagB =      (status_register >> 4) & 1;
        self.overflow =     ((status_register >> 6) & 1) != 0;
        self.negative =     ((status_register >> 7) & 1) != 0;
    }

    fn push(&mut self, value:u8) {

    }

    fn pull(&mut self) -> u8 {
        0
    }
}
