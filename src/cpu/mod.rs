//! CPU component
use crate::memory::Memory;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

mod opcodes;

struct Status{
    program_counter: u16,
    stack_pointer: u8,
    accumulator: u8,
    x_register: u8,
    y_register: u8,
    status_register: u8,
    total_cycles: u32,
}

pub struct Cpu {
    // Access to BUS
    memory: Rc<RefCell<Memory>>,

    // Registers
    accumulator: u8,
    x_register: u8,
    y_register: u8,
    program_counter: u16,
    stack_pointer: u8,

    // Flags
    negative: bool,
    overflow: bool,
    break_flag: bool,
    decimal: bool,
    interrupt: bool,
    zero: bool,
    carry: bool,

    // Instructions calls
    instructions: HashMap<u8, fn(&mut Cpu) -> (u16, u32)>,

    // Other states
    total_cycles: u32,
    remaining_cycles: u32,
    additionnal_cycles: u32,
    compteur: u32,
}

impl Cpu {
    pub fn new(memory: Rc<RefCell<Memory>>) -> Cpu {
        Cpu {
            memory: memory,
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
        // ADC
        self.instructions.insert(0x61, Cpu::fn_0x61);
        self.instructions.insert(0x65, Cpu::fn_0x65);
        self.instructions.insert(0x69, Cpu::fn_0x69);
        self.instructions.insert(0x6d, Cpu::fn_0x6d);
        self.instructions.insert(0x71, Cpu::fn_0x71);
        self.instructions.insert(0x75, Cpu::fn_0x75);
        self.instructions.insert(0x79, Cpu::fn_0x79);
        self.instructions.insert(0x7d, Cpu::fn_0x7d);
        // AND
        self.instructions.insert(0x21, Cpu::fn_0x21);
        self.instructions.insert(0x25, Cpu::fn_0x25);
        self.instructions.insert(0x29, Cpu::fn_0x29);
        self.instructions.insert(0x2d, Cpu::fn_0x2d);
        self.instructions.insert(0x31, Cpu::fn_0x31);
        self.instructions.insert(0x35, Cpu::fn_0x35);
        self.instructions.insert(0x39, Cpu::fn_0x39);
        self.instructions.insert(0x3d, Cpu::fn_0x3d);
        // ASL
        self.instructions.insert(0x06, Cpu::fn_0x06);
        self.instructions.insert(0x0e, Cpu::fn_0x0e);
        self.instructions.insert(0x16, Cpu::fn_0x16);
        self.instructions.insert(0x1e, Cpu::fn_0x1e);
        // BIT
        self.instructions.insert(0x24, Cpu::fn_0x24);
        self.instructions.insert(0x2c, Cpu::fn_0x2c);
        // BRANCHES
        self.instructions.insert(0x10, Cpu::fn_0x10);
        self.instructions.insert(0x30, Cpu::fn_0x30);
        self.instructions.insert(0x50, Cpu::fn_0x50);
        self.instructions.insert(0x70, Cpu::fn_0x70);
        self.instructions.insert(0x90, Cpu::fn_0x90);
        self.instructions.insert(0xb0, Cpu::fn_0xb0);
        self.instructions.insert(0xd0, Cpu::fn_0xd0);
        self.instructions.insert(0xf0, Cpu::fn_0xf0);
        // BRK
        self.instructions.insert(0x00, Cpu::fn_0x00);
        // CMP
        self.instructions.insert(0xc9, Cpu::fn_0xc9);
        self.instructions.insert(0xc5, Cpu::fn_0xc5);
        self.instructions.insert(0xd5, Cpu::fn_0xd5);
        self.instructions.insert(0xcd, Cpu::fn_0xcd);
        self.instructions.insert(0xdd, Cpu::fn_0xdd);
        self.instructions.insert(0xd9, Cpu::fn_0xd9);
        self.instructions.insert(0xc1, Cpu::fn_0xc1);
        self.instructions.insert(0xd1, Cpu::fn_0xd1);
        // CPX
        self.instructions.insert(0xe0, Cpu::fn_0xe0);
        self.instructions.insert(0xe4, Cpu::fn_0xe4);
        self.instructions.insert(0xec, Cpu::fn_0xec);
        // CPY
        self.instructions.insert(0xc0, Cpu::fn_0xc0);
        self.instructions.insert(0xc4, Cpu::fn_0xc4);
        self.instructions.insert(0xcc, Cpu::fn_0xcc);
        // DEC
        self.instructions.insert(0xc6, Cpu::fn_0xc6);
        self.instructions.insert(0xd6, Cpu::fn_0xd6);
        self.instructions.insert(0xce, Cpu::fn_0xce);
        self.instructions.insert(0xde, Cpu::fn_0xde);
        // DCP
        self.instructions.insert(0xc7, Cpu::fn_0xc7);
        self.instructions.insert(0xd7, Cpu::fn_0xd7);
        self.instructions.insert(0xcf, Cpu::fn_0xcf);
        self.instructions.insert(0xdf, Cpu::fn_0xdf);
        self.instructions.insert(0xdb, Cpu::fn_0xdb);
        self.instructions.insert(0xc3, Cpu::fn_0xc3);
        self.instructions.insert(0xd3, Cpu::fn_0xd3);
        // ISC
        self.instructions.insert(0xe7, Cpu::fn_0xe7);
        self.instructions.insert(0xf7, Cpu::fn_0xf7);
        self.instructions.insert(0xef, Cpu::fn_0xef);
        self.instructions.insert(0xff, Cpu::fn_0xff);
        self.instructions.insert(0xfb, Cpu::fn_0xfb);
        self.instructions.insert(0xe3, Cpu::fn_0xe3);
        self.instructions.insert(0xf3, Cpu::fn_0xf3);
        // EOR
        self.instructions.insert(0x49, Cpu::fn_0x49);
        self.instructions.insert(0x45, Cpu::fn_0x45);
        self.instructions.insert(0x55, Cpu::fn_0x55);
        self.instructions.insert(0x4d, Cpu::fn_0x4d);
        self.instructions.insert(0x5d, Cpu::fn_0x5d);
        self.instructions.insert(0x59, Cpu::fn_0x59);
        self.instructions.insert(0x41, Cpu::fn_0x41);
        self.instructions.insert(0x51, Cpu::fn_0x51);
        // Flags
        self.instructions.insert(0x18, Cpu::fn_0x18);
        self.instructions.insert(0x38, Cpu::fn_0x38);
        self.instructions.insert(0x58, Cpu::fn_0x58);
        self.instructions.insert(0x78, Cpu::fn_0x78);
        self.instructions.insert(0xb8, Cpu::fn_0xb8);
        self.instructions.insert(0xd8, Cpu::fn_0xd8);
        self.instructions.insert(0xf8, Cpu::fn_0xf8);
        // INC
        self.instructions.insert(0xe6, Cpu::fn_0xe6);
        self.instructions.insert(0xf6, Cpu::fn_0xf6);
        self.instructions.insert(0xee, Cpu::fn_0xee);
        self.instructions.insert(0xfe, Cpu::fn_0xfe);
        // JMP / JSR
        self.instructions.insert(0x4c, Cpu::fn_0x4c);
        self.instructions.insert(0x6c, Cpu::fn_0x6c);
        self.instructions.insert(0x20, Cpu::fn_0x20);
        // LDA
        self.instructions.insert(0xa9, Cpu::fn_0xa9);
        self.instructions.insert(0xa5, Cpu::fn_0xa5);
        self.instructions.insert(0xb5, Cpu::fn_0xb5);
        self.instructions.insert(0xad, Cpu::fn_0xad);
        self.instructions.insert(0xbd, Cpu::fn_0xbd);
        self.instructions.insert(0xb9, Cpu::fn_0xb9);
        self.instructions.insert(0xa1, Cpu::fn_0xa1);
        self.instructions.insert(0xb1, Cpu::fn_0xb1);
        // LDX
        self.instructions.insert(0xa2, Cpu::fn_0xa2);
        self.instructions.insert(0xa6, Cpu::fn_0xa6);
        self.instructions.insert(0xb6, Cpu::fn_0xb6);
        self.instructions.insert(0xae, Cpu::fn_0xae);
        self.instructions.insert(0xbe, Cpu::fn_0xbe);
        // LDY
        self.instructions.insert(0xa0, Cpu::fn_0xa0);
        self.instructions.insert(0xa4, Cpu::fn_0xa4);
        self.instructions.insert(0xb4, Cpu::fn_0xb4);
        self.instructions.insert(0xac, Cpu::fn_0xac);
        self.instructions.insert(0xbc, Cpu::fn_0xbc);
        // LSR
        self.instructions.insert(0x4a, Cpu::fn_0x4a);
        self.instructions.insert(0x46, Cpu::fn_0x46);
        self.instructions.insert(0x56, Cpu::fn_0x56);
        self.instructions.insert(0x4e, Cpu::fn_0x4e);
        self.instructions.insert(0x5e, Cpu::fn_0x5e);
        // NOP
        self.instructions.insert(0xea, Cpu::fn_0xea);
        self.instructions.insert(0x1a, Cpu::fn_0x1a);
        self.instructions.insert(0x3a, Cpu::fn_0x3a);
        self.instructions.insert(0x5a, Cpu::fn_0x5a);
        self.instructions.insert(0x7a, Cpu::fn_0x7a);
        self.instructions.insert(0xda, Cpu::fn_0xda);
        self.instructions.insert(0xfa, Cpu::fn_0xfa);
        // DOP
        self.instructions.insert(0x04, Cpu::fn_0x04);
        self.instructions.insert(0x14, Cpu::fn_0x14);
        self.instructions.insert(0x34, Cpu::fn_0x34);
        self.instructions.insert(0x44, Cpu::fn_0x44);
        self.instructions.insert(0x54, Cpu::fn_0x54);
        self.instructions.insert(0x64, Cpu::fn_0x64);
        self.instructions.insert(0x74, Cpu::fn_0x74);
        self.instructions.insert(0x80, Cpu::fn_0x80);
        self.instructions.insert(0x82, Cpu::fn_0x82);
        self.instructions.insert(0x89, Cpu::fn_0x89);
        self.instructions.insert(0xc2, Cpu::fn_0xc2);
        self.instructions.insert(0xd4, Cpu::fn_0xd4);
        self.instructions.insert(0xe2, Cpu::fn_0xe2);
        self.instructions.insert(0xf4, Cpu::fn_0xf4);
        // TOP
        self.instructions.insert(0x0c, Cpu::fn_0x0c);
        self.instructions.insert(0x1c, Cpu::fn_0x1c);
        self.instructions.insert(0x3c, Cpu::fn_0x3c);
        self.instructions.insert(0x4c, Cpu::fn_0x4c);
        self.instructions.insert(0x5c, Cpu::fn_0x5c);
        self.instructions.insert(0x6c, Cpu::fn_0x6c);
        self.instructions.insert(0x7c, Cpu::fn_0x7c);
        self.instructions.insert(0xdc, Cpu::fn_0xdc);
        self.instructions.insert(0xfc, Cpu::fn_0xfc);
        // ORA
        self.instructions.insert(0x09, Cpu::fn_0x09);
        self.instructions.insert(0x05, Cpu::fn_0x05);
        self.instructions.insert(0x15, Cpu::fn_0x15);
        self.instructions.insert(0x0d, Cpu::fn_0x0d);
        self.instructions.insert(0x1d, Cpu::fn_0x1d);
        self.instructions.insert(0x19, Cpu::fn_0x19);
        self.instructions.insert(0x01, Cpu::fn_0x01);
        self.instructions.insert(0x11, Cpu::fn_0x11);
        // SLO
        self.instructions.insert(0x07, Cpu::fn_0x07);
        self.instructions.insert(0x17, Cpu::fn_0x17);
        self.instructions.insert(0x0f, Cpu::fn_0x0f);
        self.instructions.insert(0x1f, Cpu::fn_0x1f);
        self.instructions.insert(0x1b, Cpu::fn_0x1b);
        self.instructions.insert(0x03, Cpu::fn_0x03);
        self.instructions.insert(0x13, Cpu::fn_0x13);
        // RLA
        self.instructions.insert(0x27, Cpu::fn_0x27);
        self.instructions.insert(0x37, Cpu::fn_0x37);
        self.instructions.insert(0x2f, Cpu::fn_0x2f);
        self.instructions.insert(0x3f, Cpu::fn_0x3f);
        self.instructions.insert(0x3b, Cpu::fn_0x3b);
        self.instructions.insert(0x23, Cpu::fn_0x23);
        self.instructions.insert(0x33, Cpu::fn_0x33);
        // RRA
        self.instructions.insert(0x67, Cpu::fn_0x67);
        self.instructions.insert(0x77, Cpu::fn_0x77);
        self.instructions.insert(0x6f, Cpu::fn_0x6f);
        self.instructions.insert(0x7f, Cpu::fn_0x7f);
        self.instructions.insert(0x7b, Cpu::fn_0x7b);
        self.instructions.insert(0x63, Cpu::fn_0x63);
        self.instructions.insert(0x73, Cpu::fn_0x73);
        // SRE
        self.instructions.insert(0x47, Cpu::fn_0x47);
        self.instructions.insert(0x57, Cpu::fn_0x57);
        self.instructions.insert(0x4f, Cpu::fn_0x4f);
        self.instructions.insert(0x5f, Cpu::fn_0x5f);
        self.instructions.insert(0x5b, Cpu::fn_0x5b);
        self.instructions.insert(0x43, Cpu::fn_0x43);
        self.instructions.insert(0x53, Cpu::fn_0x53);
        // Registers operations
        self.instructions.insert(0xaa, Cpu::fn_0xaa);
        self.instructions.insert(0x8a, Cpu::fn_0x8a);
        self.instructions.insert(0xca, Cpu::fn_0xca);
        self.instructions.insert(0xe8, Cpu::fn_0xe8);
        self.instructions.insert(0xa8, Cpu::fn_0xa8);
        self.instructions.insert(0x98, Cpu::fn_0x98);
        self.instructions.insert(0x88, Cpu::fn_0x88);
        self.instructions.insert(0xc8, Cpu::fn_0xc8);
        // ROL
        self.instructions.insert(0x2a, Cpu::fn_0x2a);
        self.instructions.insert(0x26, Cpu::fn_0x26);
        self.instructions.insert(0x36, Cpu::fn_0x36);
        self.instructions.insert(0x2e, Cpu::fn_0x2e);
        self.instructions.insert(0x3e, Cpu::fn_0x3e);
        // ROR
        self.instructions.insert(0x6a, Cpu::fn_0x6a);
        self.instructions.insert(0x66, Cpu::fn_0x66);
        self.instructions.insert(0x76, Cpu::fn_0x76);
        self.instructions.insert(0x6e, Cpu::fn_0x6e);
        self.instructions.insert(0x7e, Cpu::fn_0x7e);
        // RTS / RTI
        self.instructions.insert(0x40, Cpu::fn_0x40);
        self.instructions.insert(0x60, Cpu::fn_0x60);
        // SBC
        self.instructions.insert(0xe9, Cpu::fn_0xe9);
        self.instructions.insert(0xeb, Cpu::fn_0xeb);
        self.instructions.insert(0xe5, Cpu::fn_0xe5);
        self.instructions.insert(0xf5, Cpu::fn_0xf5);
        self.instructions.insert(0xed, Cpu::fn_0xed);
        self.instructions.insert(0xfd, Cpu::fn_0xfd);
        self.instructions.insert(0xf9, Cpu::fn_0xf9);
        self.instructions.insert(0xe1, Cpu::fn_0xe1);
        self.instructions.insert(0xf1, Cpu::fn_0xf1);
        // STA
        self.instructions.insert(0x85, Cpu::fn_0x85);
        self.instructions.insert(0x95, Cpu::fn_0x95);
        self.instructions.insert(0x8d, Cpu::fn_0x8d);
        self.instructions.insert(0x9d, Cpu::fn_0x9d);
        self.instructions.insert(0x99, Cpu::fn_0x99);
        self.instructions.insert(0x81, Cpu::fn_0x81);
        self.instructions.insert(0x91, Cpu::fn_0x91);
        // Stack
        self.instructions.insert(0x9a, Cpu::fn_0x9a);
        self.instructions.insert(0xba, Cpu::fn_0xba);
        self.instructions.insert(0x48, Cpu::fn_0x48);
        self.instructions.insert(0x68, Cpu::fn_0x68);
        self.instructions.insert(0x08, Cpu::fn_0x08);
        self.instructions.insert(0x28, Cpu::fn_0x28);
        // STX
        self.instructions.insert(0x86, Cpu::fn_0x86);
        self.instructions.insert(0x96, Cpu::fn_0x96);
        self.instructions.insert(0x8e, Cpu::fn_0x8e);
        // STY
        self.instructions.insert(0x84, Cpu::fn_0x84);
        self.instructions.insert(0x94, Cpu::fn_0x94);
        self.instructions.insert(0x8c, Cpu::fn_0x8c);
        // LAX
        self.instructions.insert(0xa7, Cpu::fn_0xa7);
        self.instructions.insert(0xb7, Cpu::fn_0xb7);
        self.instructions.insert(0xaf, Cpu::fn_0xaf);
        self.instructions.insert(0xbf, Cpu::fn_0xbf);
        self.instructions.insert(0xa3, Cpu::fn_0xa3);
        self.instructions.insert(0xb3, Cpu::fn_0xb3);
        // SAX
        self.instructions.insert(0x87, Cpu::fn_0x87);
        self.instructions.insert(0x97, Cpu::fn_0x97);
        self.instructions.insert(0x8f, Cpu::fn_0x8f);
        self.instructions.insert(0x83, Cpu::fn_0x83);
    }

    /// Dummy function to temporarly load the instruction array
    fn dummy(&mut self) -> (u16, u32) {
        let opcode: u8 = self.memory.borrow_mut().read_rom(self.program_counter);
        panic!(
            "Function is not implemented yet at PC = {:x}, opcode = {:x}",
            self.program_counter, opcode
        );
        //(0, 0)
    }
    /// CPU initialisation function
    pub fn start(&mut self, entry_point: Option<u16>) {
        //!Execute 6502 Start sequence

        self.populate_instructions_vector();

        // Default is equivalent to JMP ($FFFC)
        self.program_counter = entry_point.unwrap_or(self.memory.borrow_mut().read_rom_16(0xfffc));

        println!("Entry point is {:x}", self.program_counter);

        //Start sequence push stack three time
        self.push(0);
        self.push(0);
        self.push(0);
        self.total_cycles = 7; // Cout de l'init
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

        let opcode: u8 = self.memory.borrow_mut().read_rom(self.program_counter);

        let cpu_instruction = self.instructions[&opcode];
        let (step, remaining_cycles) = cpu_instruction(self);
        self.remaining_cycles = remaining_cycles + self.additionnal_cycles;
        self.total_cycles += self.remaining_cycles;
        self.remaining_cycles -= 1; // Do not count current cycle twice
        self.additionnal_cycles = 0;
        self.program_counter += step;
        self.compteur += 1;
    }

    /// Get interrup flag status. Required for emulator to raise IRQ
    pub fn get_interrupt_flag(&self) -> bool {
        self.interrupt
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

        self.program_counter = self.memory.borrow_mut().read_rom_16(address);
        self.remaining_cycles = 7 - 1; // do not count current cycle twice
        self.total_cycles += 7
    }

    /// Returns the P register which contains the flag status.
    ///
    /// Bit 5 is always set to 1
    fn get_status_register(&self) -> u8 {
        ((self.negative as u8) << 7)
            | ((self.overflow as u8) << 6)
            | (1 << 5)
            | ((self.break_flag as u8) << 4)
            | ((self.decimal as u8) << 3)
            | ((self.interrupt as u8) << 2)
            | ((self.zero as u8) << 1)
            | (self.carry as u8)
    }

    /// Set the P register which contains the flag status.
    ///
    /// When setting the P Register, the break flag is not set.
    fn set_status_register(&mut self, status_register: u8) {
        self.carry = (status_register & 1) != 0;
        self.zero = ((status_register >> 1) & 1) != 0;
        self.interrupt = ((status_register >> 2) & 1) != 0;
        self.decimal = ((status_register >> 3) & 1) != 0;
        //self.flagB =      (status_register >> 4) & 1;
        self.overflow = ((status_register >> 6) & 1) != 0;
        self.negative = ((status_register >> 7) & 1) != 0;
    }

    /// Push value into stack
    fn push(&mut self, value: u8) {
        self.memory
            .borrow_mut()
            .write_rom(0x0100 | (self.stack_pointer as u16), value);
        self.stack_pointer = self.stack_pointer - 1; // Will eventually overflow on purpose
    }

    /// Pop/Pull value from stack
    fn pull(&mut self) -> u8 {
        self.stack_pointer = self.stack_pointer + 1; // Will eventually overflow on purpose
        self.memory
            .borrow_mut()
            .read_rom(0x0100 | (self.stack_pointer as u16))
    }

    /// Get 8 bit immediate value on PC + 1
    fn get_immediate(&mut self) -> u8 {
        self.memory.borrow_mut().read_rom(self.program_counter + 1)
    }

    /// Write val into Zero Page memory. Address is given as opcode 1-byte argument
    fn set_zero_page(&mut self, value: u8) {
        let address = self.get_zero_page_address();
        self.memory.borrow_mut().write_rom(address, value);
    }

    /// Get ZeroPage address to be used for current opcode. Alias to get_immediate
    fn get_zero_page_address(&mut self) -> u16 {
        self.get_immediate() as u16
    }

    /// Get val from Zero Page MEMORY. Address is given as opcode 1-byte argument
    fn get_zero_page_value(&mut self) -> u8 {
        let address = self.get_immediate() as u16;
        self.memory.borrow_mut().read_rom(address)
    }

    /// Write val into Zero Page MEMORY. Address is given as opcode 1-byte argument and X register
    fn set_zero_page_x(&mut self, value: u8) {
        let address = self.get_zero_page_x_address();
        self.memory.borrow_mut().write_rom(address, value);
    }

    /// Get ZeroPage address to be used for current opcode and X register
    fn get_zero_page_x_address(&mut self) -> u16 {
        ((self.memory.borrow_mut().read_rom(self.program_counter + 1) + self.x_register) & 255)
            as u16
    }

    /// Get value at ZeroPage address to be used for current opcode and X register
    fn get_zero_page_x_value(&mut self) -> u8 {
        let address = self.get_zero_page_x_address();
        self.memory.borrow_mut().read_rom(address)
    }

    /// Write val into Zero Page MEMORY. Address is given as opcode 1-byte argument and Y register
    fn set_zero_page_y(&mut self, value: u8) {
        let address = self.get_zero_page_y_address();
        self.memory.borrow_mut().write_rom(address, value);
    }

    /// Get ZeroPage address to be used for current opcode and Y register
    fn get_zero_page_y_address(&mut self) -> u16 {
        ((self.memory.borrow_mut().read_rom(self.program_counter + 1) + self.y_register) & 255)
            as u16
    }

    /// Get value at ZeroPage address to be used for current opcode and Y register
    fn get_zero_page_y_value(&mut self) -> u8 {
        let address = self.get_zero_page_y_address();
        self.memory.borrow_mut().read_rom(address)
    }

    /// Write val into MEMORY. Address is given as opcode 2-byte argument
    fn set_absolute(&mut self, value: u8) {
        let address = self.get_absolute_address();
        self.memory.borrow_mut().write_rom(address, value);
    }

    /// Get address given as opcode 2-byte argument
    fn get_absolute_address(&mut self) -> u16 {
        self.memory
            .borrow_mut()
            .read_rom_16(self.program_counter + 1)
    }

    /// Get val from MEMORY. Address is given as opcode 2-byte argument
    fn get_absolute_value(&mut self) -> u8 {
        let address = self.get_absolute_address();
        self.memory.borrow_mut().read_rom(address)
    }

    /// Write val into MEMORY. Address is given as opcode 2-byte argument and X register
    /// additionnal is boolean to fnine if this instruction will require extra cycles on page crossing
    fn set_absolute_x(&mut self, value: u8, is_additionnal: bool) {
        let address = self.get_absolute_x_address(is_additionnal);
        self.memory.borrow_mut().write_rom(address, value);
    }

    /// Get address given as opcode 2-byte argument and X register
    fn get_absolute_x_address(&mut self, is_additionnal: bool) -> u16 {
        let address = self
            .memory
            .borrow_mut()
            .read_rom_16(self.program_counter + 1);
        let target_address = address + self.x_register as u16;
        if is_additionnal && address & 0xFF00 != target_address & 0xff00 {
            self.additionnal_cycles += 1;
        }
        target_address
    }

    /// Get val from MEMORY. Address is given as opcode 2-byte argument and X register
    fn get_absolute_x_value(&mut self, is_additionnal: bool) -> u8 {
        let address = self.get_absolute_x_address(is_additionnal);
        self.memory.borrow_mut().read_rom(address)
    }

    /// Write val into MEMORY. Address is given as opcode 2-byte argument and Y register
    fn set_absolute_y(&mut self, value: u8, is_additionnal: bool) {
        let address = self.get_absolute_y_address(is_additionnal);
        self.memory.borrow_mut().write_rom(address, value);
    }

    /// Get address given as opcode 2-byte argument and Y register
    fn get_absolute_y_address(&mut self, is_additionnal: bool) -> u16 {
        let address = self
            .memory
            .borrow_mut()
            .read_rom_16(self.program_counter + 1);
        let target_address = address + self.y_register as u16;
        if is_additionnal && address & 0xff00 != target_address & 0xff00 {
            self.additionnal_cycles += 1;
        }
        target_address
    }

    /// Get val from MEMORY. Address is given as opcode 2-byte argument and Y register
    fn get_absolute_y_value(&mut self, is_additionnal: bool) -> u8 {
        let address = self.get_absolute_y_address(is_additionnal);
        self.memory.borrow_mut().read_rom(address)
    }

    /// Get indirect address given as opcode 2-byte argument and X register
    fn get_indirect_x_address(&mut self) -> u16 {
        let address = self.get_zero_page_x_address();
        self.memory
            .borrow_mut()
            .read_rom_16_no_crossing_page(address)
    }

    /// Get val from MEMORY. Indirect address is given as opcode 2-byte argument and X register
    fn get_indirect_x_value(&mut self) -> u8 {
        let address = self.get_indirect_x_address();
        self.memory.borrow_mut().read_rom(address)
    }

    /// Write val into MEMORY. Indirect address is given as opcode 2-byte argument and X register///
    fn set_indirect_x(&mut self, value: u8) {
        let address = self.get_indirect_x_address();
        self.memory.borrow_mut().write_rom(address, value);
    }

    /// Get indirect address given as opcode 2-byte argument and Y register
    fn get_indirect_y_address(&mut self, is_additionnal: bool) -> u16 {
        let address = self.get_zero_page_address();
        let address = self
            .memory
            .borrow_mut()
            .read_rom_16_no_crossing_page(address);
        let target_address = address + self.y_register as u16;
        if is_additionnal && address & 0xff00 != target_address & 0xff00 {
            self.additionnal_cycles += 1;
        }
        target_address
    }

    /// Get val from MEMORY. Indirect address is given as opcode 2-byte argument and Y register
    fn get_indirect_y_value(&mut self, is_additionnal: bool) -> u8 {
        let address = self.get_indirect_y_address(is_additionnal);
        self.memory.borrow_mut().read_rom(address)
    }

    /// Write val into MEMORY. Indirect address is given as opcode 2-byte argument and Y register
    fn set_indirect_y(&mut self, value: u8, is_additionnal: bool) {
        let address = self.get_indirect_y_address(is_additionnal);
        self.memory.borrow_mut().write_rom(address, value);
    }

    /// Sets flags N and Z according to value
    fn set_flags_nz(&mut self, value: u8) {
        self.set_negative(value);
        self.set_zero(value);
    }

    ///  Set Negative Flag according to value
    fn set_negative(&mut self, value: u8) {
        self.negative = (value >> 7) != 0;
    }

    ///  Set Zero Flag according to value
    fn set_zero(&mut self, value: u8) {
        self.zero = value == 0;
    }

    /// Perform ADC operation for val
    fn adc(&mut self, value: u8) {
        let adc: u16 = (value as u16) + (self.accumulator as u16) + (self.carry as u16);
        self.carry = ((adc >> 8) & 1) != 0;
        let result: u8 = (0xff & adc) as u8;

        self.overflow = (!!((self.accumulator ^ result) & (value ^ result) & 0x80)) != 0;
        self.accumulator = result;
        self.set_flags_nz(self.accumulator);
    }

    /// Function call for ADC #$xx. Immediate
    fn fn_0x69(&mut self) -> (u16, u32) {
        let immediate = self.get_immediate();
        self.adc(immediate);
        (2, 2)
    }
    /// Function call for ADC $xx. Zero Page
    fn fn_0x65(&mut self) -> (u16, u32) {
        let zeropage = self.get_zero_page_value();
        self.adc(zeropage);
        (2, 3)
    }

    /// Function call for ADC $xx, X. Zero Page, X
    fn fn_0x75(&mut self) -> (u16, u32) {
        let zeropage = self.get_zero_page_x_value();
        self.adc(zeropage);
        (2, 4)
    }

    /// Function call for ADC $xxxx. Absolute
    fn fn_0x6d(&mut self) -> (u16, u32) {
        let absolute = self.get_absolute_value();
        self.adc(absolute);
        (3, 4)
    }

    /// Function call for ADC $xxxx, X. Absolute, X
    fn fn_0x7d(&mut self) -> (u16, u32) {
        let absolute = self.get_absolute_x_value(true);
        self.adc(absolute);
        (3, 4)
    }

    /// Function call for ADC $xxxx, X. Absolute, X
    fn fn_0x7d_with_no_additionnal_cycles(&mut self) -> (u16, u32) {
        let absolute = self.get_absolute_x_value(false);
        self.adc(absolute);
        (3, 4)
    }

    /// Function call for ADC $xxxx, Y. Absolute, Y
    fn fn_0x79(&mut self) -> (u16, u32) {
        let absolute = self.get_absolute_y_value(true);
        self.adc(absolute);
        (3, 4)
    }

    /// Function call for ADC $xxxx, Y. Absolute, Y
    fn fn_0x79_with_no_additionnal_cycles(&mut self) -> (u16, u32) {
        let absolute = self.get_absolute_y_value(false);
        self.adc(absolute);
        (3, 4)
    }

    /// Function call for ADC ($xx, X). Indirect, X
    fn fn_0x61(&mut self) -> (u16, u32) {
        let indirect = self.get_indirect_x_value();
        self.adc(indirect);
        (2, 6)
    }

    /// Function call for ADC ($xx), Y. Indirect, Y
    fn fn_0x71(&mut self) -> (u16, u32) {
        let indirect = self.get_indirect_y_value(true);
        self.adc(indirect);
        (2, 5)
    }

    /// Function call for ADC ($xx), Y. Indirect, Y
    fn fn_0x71_with_no_additionnal_cycles(&mut self) -> (u16, u32) {
        let indirect = self.get_indirect_y_value(false);
        self.adc(indirect);
        (2, 5)
    }

    /// Function call for AND #$xx. Immediate
    fn fn_0x29(&mut self) -> (u16, u32) {
        self.accumulator &= self.get_immediate();
        self.set_flags_nz(self.accumulator);
        (2, 2)
    }

    /// Function call for AND $xx. Zero Page
    fn fn_0x25(&mut self) -> (u16, u32) {
        self.accumulator &= self.get_zero_page_value();
        self.set_flags_nz(self.accumulator);
        (2, 3)
    }

    /// Function call for AND $xx, X. Zero Page, X
    fn fn_0x35(&mut self) -> (u16, u32) {
        self.accumulator &= self.get_zero_page_x_value();
        self.set_flags_nz(self.accumulator);
        (2, 4)
    }

    /// Function call for AND $xxxx. Absolute
    fn fn_0x2d(&mut self) -> (u16, u32) {
        self.accumulator &= self.get_absolute_value();
        self.set_flags_nz(self.accumulator);
        (3, 4)
    }

    /// Function call for AND $xxxx, X. Absolute, X
    fn fn_0x3d(&mut self) -> (u16, u32) {
        self.accumulator &= self.get_absolute_x_value(true);
        self.set_flags_nz(self.accumulator);
        (3, 4)
    }

    /// Function call for AND $xxxx, X. Absolute, X
    fn fn_0x3d_with_no_additionnal_cycles(&mut self) -> (u16, u32) {
        self.accumulator &= self.get_absolute_x_value(false);
        self.set_flags_nz(self.accumulator);
        (3, 4)
    }

    /// Function call for AND $xxxx, Y. Absolute, Y
    fn fn_0x39(&mut self) -> (u16, u32) {
        self.accumulator &= self.get_absolute_y_value(true);
        self.set_flags_nz(self.accumulator);
        (3, 4)
    }

    /// Function call for AND $xxxx, Y. Absolute, Y
    fn fn_0x39_with_no_additionnal_cycles(&mut self) -> (u16, u32) {
        self.accumulator &= self.get_absolute_y_value(false);
        self.set_flags_nz(self.accumulator);
        (3, 4)
    }

    /// Function call for AND ($xx, X). Indirect, X
    fn fn_0x21(&mut self) -> (u16, u32) {
        self.accumulator &= self.get_indirect_x_value();
        self.set_flags_nz(self.accumulator);
        (2, 6)
    }

    /// Function call for AND ($xx), Y. Indirect, Y///
    fn fn_0x31(&mut self) -> (u16, u32) {
        self.accumulator &= self.get_indirect_y_value(true);
        self.set_flags_nz(self.accumulator);
        (2, 5)
    }

    /// Function call for AND ($xx), Y. Indirect, Y
    fn fn_0x31_with_no_additionnal_cycles(&mut self) -> (u16, u32) {
        self.accumulator &= self.get_indirect_y_value(false);
        self.set_flags_nz(self.accumulator);
        (2, 5)
    }

    /// Function call for ASL $xx. Zero Page
    fn fn_0x06(&mut self) -> (u16, u32) {
        let value = self.get_zero_page_value();
        self.carry = (value >> 7) != 0;
        let value = (value << 1) & 0b11111111;
        self.set_zero_page(value);
        self.set_flags_nz(value);
        (2, 5)
    }

    /// Function call for ASL $xx, X. Zero Page, X
    fn fn_0x16(&mut self) -> (u16, u32) {
        let value = self.get_zero_page_x_value();
        self.carry = (value >> 7) != 0;
        let value = (value << 1) & 0b11111111;
        self.set_zero_page_x(value);
        self.set_flags_nz(value);
        (2, 6)
    }

    /// Function call for ASL $xxxx. Absolute///
    fn fn_0x0e(&mut self) -> (u16, u32) {
        let value = self.get_absolute_value();
        self.carry = (value >> 7) != 0;
        let value = (value << 1) & 0b11111111;
        self.set_absolute(value);
        self.set_flags_nz(value);
        (3, 6)
    }

    /// Function call for ASL $xxxx, X. Absolute, X///
    fn fn_0x1e(&mut self) -> (u16, u32) {
        let value = self.get_absolute_x_value(true);
        self.carry = (value >> 7) != 0;
        let value = (value << 1) & 0b11111111;
        self.set_absolute_x(value, true);
        self.set_flags_nz(value);
        (3, 7)
    }

    /// Function call for ASL $xxxx, X. Absolute, X///
    fn fn_0x1e_with_no_additionnal_cycles(&mut self) -> (u16, u32) {
        let value = self.get_absolute_x_value(true);
        self.carry = (value >> 7) != 0;
        let value = (value << 1) & 0b11111111;
        self.set_absolute_x(value, true);
        self.set_flags_nz(value);
        (3, 7)
    }

    /// Function call for BIT $xx. Zero Page///
    fn fn_0x24(&mut self) -> (u16, u32) {
        let tocomp = self.get_zero_page_value();
        let value = tocomp & self.accumulator;
        self.set_zero(value);
        self.set_negative(tocomp);
        self.overflow = ((tocomp >> 6) & 1) != 0;
        (2, 3)
    }

    /// Function call for BIT $xxxx. Absolute///
    fn fn_0x2c(&mut self) -> (u16, u32) {
        let tocomp = self.get_absolute_value();
        let value = tocomp & self.accumulator;
        self.set_zero(value);
        self.set_negative(tocomp);
        self.overflow = ((tocomp >> 6) & 1) != 0;
        (3, 4)
    }

    /// Function call for BPL #$xx. Relative
    fn fn_0x10(&mut self) -> (u16, u32) {
        let old_pc = self.program_counter + 2;
        let signed: i8 = self.get_immediate() as i8;
        if !self.negative {
            self.program_counter = self.program_counter.wrapping_add(signed as u16);
            self.additionnal_cycles += 1;
            if self.program_counter & 0xff00 != old_pc & 0xff00 {
                self.additionnal_cycles += 1;
            }
        }
        (2, 2)
    }

    /// Function call for BMI #$xx. Relative
    fn fn_0x30(&mut self) -> (u16, u32) {
        let old_pc = self.program_counter + 2;
        let signed: i8 = self.get_immediate() as i8;
        if self.negative {
            self.program_counter = self.program_counter.wrapping_add(signed as u16);
            self.additionnal_cycles += 1;
            if self.program_counter & 0xff00 != old_pc & 0xff00 {
                self.additionnal_cycles += 1
            }
        }
        (2, 2)
    }

    /// Function call for BVC #$xx. Relative
    fn fn_0x50(&mut self) -> (u16, u32) {
        let signed: i8 = self.get_immediate() as i8;
        if !self.overflow {
            self.program_counter = self.program_counter.wrapping_add(signed as u16);
            self.additionnal_cycles += 1;
        }
        (2, 2)
    }

    /// Function call for BVS #$xx. Relative
    fn fn_0x70(&mut self) -> (u16, u32) {
        let old_pc = self.program_counter + 2;
        let signed: i8 = self.get_immediate() as i8;
        if self.overflow {
            self.program_counter = self.program_counter.wrapping_add(signed as u16);
            self.additionnal_cycles += 1;
            if self.program_counter & 0xff00 != old_pc & 0xff00 {
                self.additionnal_cycles += 1
            }
        }
        (2, 2)
    }

    /// Function call for BCC #$xx. Relative///
    fn fn_0x90(&mut self) -> (u16, u32) {
        let old_pc = self.program_counter + 2;
        let signed: i8 = self.get_immediate() as i8;
        if !self.carry {
            self.program_counter = self.program_counter.wrapping_add(signed as u16);
            self.additionnal_cycles += 1;
            if self.program_counter & 0xff00 != old_pc & 0xff00 {
                self.additionnal_cycles += 1
            }
        }
        (2, 2)
    }

    /// Function call for BCS #$xx. Relative
    fn fn_0xb0(&mut self) -> (u16, u32) {
        let old_pc = self.program_counter + 2;
        let signed: i8 = self.get_immediate() as i8;
        if self.carry {
            self.program_counter = self.program_counter.wrapping_add(signed as u16);
            self.additionnal_cycles += 1;
            if self.program_counter & 0xff00 != old_pc & 0xff00 {
                self.additionnal_cycles += 1
            }
        }
        (2, 2)
    }

    /// Function call for BNE #$xx. Relative
    fn fn_0xd0(&mut self) -> (u16, u32) {
        let old_pc = self.program_counter + 2;
        let signed: i8 = self.get_immediate() as i8;
        if !self.zero {
            self.program_counter = self.program_counter.wrapping_add(signed as u16);
            self.additionnal_cycles += 1;
            if self.program_counter & 0xff00 != old_pc & 0xff00 {
                self.additionnal_cycles = 1;
            }
        }
        (2, 2)
    }

    /// Function call for BEQ #$xx. Relative
    fn fn_0xf0(&mut self) -> (u16, u32) {
        let old_pc = self.program_counter + 2;
        let signed: i8 = self.get_immediate() as i8;
        if self.zero {
            self.program_counter = self.program_counter.wrapping_add(signed as u16);
            self.additionnal_cycles += 1;
            if (self.program_counter + 2) & 0xff00 != old_pc & 0xff00 {
                // PC+2 to take into account current instruction size
                self.additionnal_cycles += 1;
            }
        }
        (2, 2)
    }

    /// Function call for BRK. Implied
    ///TODO ! Should set Break flag to 1
    fn fn_0x00(&mut self) -> (u16, u32) {
        self.program_counter += 1;
        self.push((self.program_counter >> 8) as u8);
        self.push((self.program_counter & 0xff) as u8);
        self.push(self.get_status_register());
        self.program_counter = self.memory.borrow_mut().read_rom_16(0xfffe);
        (0, 7)
    }

    /// General implementation for CMP operation
    ///
    /// Args:
    ///     op1 -- First operand
    ///     op2 -- First operand
    ///
    fn cmp(&mut self, op1: u8, op2: u8) {
        if op1 > op2 {
            if op1 - op2 >= 0x80 {
                self.carry = true;
                self.negative = true;
                self.zero = false;
            } else {
                self.carry = true;
                self.negative = false;
                self.zero = false;
            }
        } else if op1 == op2 {
            self.carry = true;
            self.negative = false;
            self.zero = true;
        } else {
            if op2 - op1 >= 0x80 {
                self.carry = false;
                self.negative = false;
                self.zero = false;
            } else {
                self.carry = false;
                self.negative = true;
                self.zero = false;
            }
        }
    }

    /// Function call for CMP #$xx. Immediate
    fn fn_0xc9(&mut self) -> (u16, u32) {
        let immediate = self.get_immediate();
        self.cmp(self.accumulator, immediate);
        (2, 2)
    }

    /// Function call for CMP $xx. Zero Page
    fn fn_0xc5(&mut self) -> (u16, u32) {
        let zero_page = self.get_zero_page_value();
        self.cmp(self.accumulator, zero_page);
        (2, 3)
    }

    /// Function call for CMP $xx, X. Zero Page, X
    fn fn_0xd5(&mut self) -> (u16, u32) {
        let zero_page_x = self.get_zero_page_x_value();
        self.cmp(self.accumulator, zero_page_x);
        (2, 4)
    }

    /// Function call for CMP $xxxx. Absolute
    fn fn_0xcd(&mut self) -> (u16, u32) {
        let absolute = self.get_absolute_value();
        self.cmp(self.accumulator, absolute);
        (3, 4)
    }

    /// Function call for CMP $xxxx, X. Absolute, X
    fn fn_0xdd(&mut self) -> (u16, u32) {
        let absolute_x = self.get_absolute_x_value(true);
        self.cmp(self.accumulator, absolute_x);
        (3, 4)
    }

    /// Function call for CMP $xxxx, Y. Absolute, Y
    fn fn_0xd9(&mut self) -> (u16, u32) {
        let absolute_y = self.get_absolute_y_value(true);
        self.cmp(self.accumulator, absolute_y);
        (3, 4)
    }

    /// Function call for CMP ($xx, X). Indirect, X
    fn fn_0xc1(&mut self) -> (u16, u32) {
        let indirect_x = self.get_indirect_x_value();
        self.cmp(self.accumulator, indirect_x);
        (2, 6)
    }

    /// Function call for CMP ($xx), Y. Indirect, Y
    fn fn_0xd1(&mut self) -> (u16, u32) {
        let indirect_y = self.get_indirect_y_value(true);
        self.cmp(self.accumulator, indirect_y);
        (2, 5)
    }

    /// Function call for CPX #$xx. Immediate
    fn fn_0xe0(&mut self) -> (u16, u32) {
        let immediate = self.get_immediate();
        self.cmp(self.x_register, immediate);
        (2, 2)
    }

    /// Function call for CPX $xx. Zero Page
    fn fn_0xe4(&mut self) -> (u16, u32) {
        let zero_page = self.get_zero_page_value();
        self.cmp(self.x_register, zero_page);
        (2, 3)
    }

    /// Function call for CPX $xxxx. Absolute
    fn fn_0xec(&mut self) -> (u16, u32) {
        let absolute = self.get_absolute_value();
        self.cmp(self.x_register, absolute);
        (3, 4)
    }

    /// Function call for CPY #$xx. Immediate
    fn fn_0xc0(&mut self) -> (u16, u32) {
        let immediate = self.get_immediate();
        self.cmp(self.y_register, immediate);
        (2, 2)
    }

    /// Function call for CPY $xx. Zero Page
    fn fn_0xc4(&mut self) -> (u16, u32) {
        let zero_page = self.get_zero_page_value();
        self.cmp(self.y_register, zero_page);
        (2, 3)
    }

    /// Function call for CPY $xxxx. Absolute
    fn fn_0xcc(&mut self) -> (u16, u32) {
        let absolute = self.get_absolute_value();
        self.cmp(self.y_register, absolute);
        (3, 4)
    }

    /// Function call for DEC $xx. Zero Page
    fn fn_0xc6(&mut self) -> (u16, u32) {
        let value = self.get_zero_page_value();
        let value = value - 1;
        self.set_zero_page(value);
        self.set_flags_nz(value);
        (2, 5)
    }

    /// Function call for DEC $xx, X. Zero Page, X
    fn fn_0xd6(&mut self) -> (u16, u32) {
        let value = self.get_zero_page_x_value();
        let value = value - 1;
        self.set_zero_page_x(value);
        self.set_flags_nz(value);
        (2, 6)
    }

    /// Function call for DEC $xxxx. Absolute
    fn fn_0xce(&mut self) -> (u16, u32) {
        let value = self.get_absolute_value();
        let value = value - 1;
        self.set_absolute(value);
        self.set_flags_nz(value);
        (3, 6)
    }

    /// Function call for DEC $xxxx, X. Absolute, X
    fn fn_0xde(&mut self) -> (u16, u32) {
        let value = self.get_absolute_x_value(true);
        let value = value - 1;
        self.set_absolute_x(value, true);
        self.set_flags_nz(value);
        (3, 7)
    }

    /// Function call for DCP $xx. Zero Page
    fn fn_0xc7(&mut self) -> (u16, u32) {
        let value = self.get_zero_page_value();
        let value = value - 1;
        self.set_zero_page(value);
        self.cmp(self.accumulator, value);
        (2, 5)
    }

    /// Function call for DCP $xx, X. Zero Page, X
    fn fn_0xd7(&mut self) -> (u16, u32) {
        let value = self.get_zero_page_x_value();
        let value = value - 1;
        self.set_zero_page_x(value);
        self.cmp(self.accumulator, value);
        (2, 6)
    }

    /// Function call for DCP $xxxx. Absolute
    fn fn_0xcf(&mut self) -> (u16, u32) {
        let value = self.get_absolute_value();
        let value = value - 1;
        self.set_absolute(value);
        self.cmp(self.accumulator, value);
        (3, 6)
    }

    /// Function call for DCP $xxxx, X. Absolute, X
    fn fn_0xdf(&mut self) -> (u16, u32) {
        let value = self.get_absolute_x_value(false);
        let value = value - 1;
        self.set_absolute_x(value, false);
        self.cmp(self.accumulator, value);
        (3, 7)
    }

    /// Function call for DCP $xxxx, Y. Absolute, Y
    fn fn_0xdb(&mut self) -> (u16, u32) {
        let value = self.get_absolute_y_value(false);
        let value = value - 1;
        self.set_absolute_y(value, false);
        self.cmp(self.accumulator, value);
        (3, 7)
    }

    /// Function call for DCP ($xx, X). Indirect, X
    fn fn_0xc3(&mut self) -> (u16, u32) {
        let value = self.get_indirect_x_value();
        let value = value - 1;
        self.set_indirect_x(value);
        self.cmp(self.accumulator, value);
        (2, 8)
    }

    /// Function call for DCP ($xx), Y. Indirect, Y
    fn fn_0xd3(&mut self) -> (u16, u32) {
        let value = self.get_indirect_y_value(false);
        let value = value - 1;
        self.set_indirect_y(value, false);
        self.cmp(self.accumulator, value);
        (2, 8)
    }

    /// Function call for ISC $xx. Zero Page
    fn fn_0xe7(&mut self) -> (u16, u32) {
        let value = self.get_zero_page_value();
        let value = value + 1;
        self.set_zero_page(value);
        self.sbc(value);
        (2, 5)
    }

    /// Function call for ISC $xx, X. Zero Page, X
    fn fn_0xf7(&mut self) -> (u16, u32) {
        let value = self.get_zero_page_x_value();
        let value = value + 1;
        self.set_zero_page_x(value);
        self.sbc(value);
        (2, 6)
    }

    /// Function call for ISC $xxxx. Absolute
    fn fn_0xef(&mut self) -> (u16, u32) {
        let value = self.get_absolute_value();
        let value = value + 1;
        self.set_absolute(value);
        self.sbc(value);
        (3, 6)
    }

    /// Function call for ISC $xxxx, X. Absolute, X
    fn fn_0xff(&mut self) -> (u16, u32) {
        let value = self.get_absolute_x_value(false);
        let value = value + 1;
        self.set_absolute_x(value, false);
        self.sbc(value);
        (3, 7)
    }

    /// Function call for ISC $xxxx, Y. Absolute, Y
    fn fn_0xfb(&mut self) -> (u16, u32) {
        let value = self.get_absolute_y_value(false);
        let value = value + 1;
        self.set_absolute_y(value, false);
        self.sbc(value);
        (3, 7)
    }

    /// Function call for ISC ($xx), X. Indirect, X
    fn fn_0xe3(&mut self) -> (u16, u32) {
        let value = self.get_indirect_x_value();
        let value = value + 1;
        self.set_indirect_x(value);
        self.sbc(value);
        (2, 8)
    }

    /// Function call for ISC ($xx, Y). Indirect, Y
    fn fn_0xf3(&mut self) -> (u16, u32) {
        let value = self.get_indirect_y_value(false);
        let value = value + 1;
        self.set_indirect_y(value, false);
        self.sbc(value);
        (2, 6)
    }

    /// Function call for EOR #$xx. Immediate
    fn fn_0x49(&mut self) -> (u16, u32) {
        self.accumulator ^= self.get_immediate();
        self.set_flags_nz(self.accumulator);
        (2, 2)
    }

    /// Function call for EOR $xx. Zero Page
    fn fn_0x45(&mut self) -> (u16, u32) {
        self.accumulator ^= self.get_zero_page_value();
        self.set_flags_nz(self.accumulator);
        (2, 3)
    }

    /// Function call for EOR $xx, X. Zero Page, X
    fn fn_0x55(&mut self) -> (u16, u32) {
        self.accumulator ^= self.get_zero_page_x_value();
        self.set_flags_nz(self.accumulator);
        (2, 4)
    }

    /// Function call for EOR $xxxx. Absolute
    fn fn_0x4d(&mut self) -> (u16, u32) {
        self.accumulator ^= self.get_absolute_value();
        self.set_flags_nz(self.accumulator);
        (3, 4)
    }

    /// Function call for EOR $xxxx, X. Absolute, X
    fn fn_0x5d(&mut self) -> (u16, u32) {
        self.accumulator ^= self.get_absolute_x_value(true);
        self.set_flags_nz(self.accumulator);
        (3, 4)
    }

    /// Function call for EOR $xxxx, X. Absolute, X
    fn fn_0x5d_with_no_additionnal_cycles(&mut self) -> (u16, u32) {
        self.accumulator ^= self.get_absolute_x_value(false);
        self.set_flags_nz(self.accumulator);
        (3, 4)
    }

    /// Function call for EOR $xxxx, Y. Absolute, Y
    fn fn_0x59(&mut self) -> (u16, u32) {
        self.accumulator ^= self.get_absolute_y_value(true);
        self.set_flags_nz(self.accumulator);
        (3, 4)
    }

    /// Function call for EOR $xxxx, Y. Absolute, Y
    fn fn_0x59_with_no_additionnal_cycles(&mut self) -> (u16, u32) {
        self.accumulator ^= self.get_absolute_y_value(false);
        self.set_flags_nz(self.accumulator);
        (3, 4)
    }

    /// Function call for EOR ($xx, X). Indirect, X
    fn fn_0x41(&mut self) -> (u16, u32) {
        self.accumulator ^= self.get_indirect_x_value();
        self.set_flags_nz(self.accumulator);
        (2, 6)
    }

    /// Function call for EOR ($xx), Y. Indirect, Y
    fn fn_0x51(&mut self) -> (u16, u32) {
        self.accumulator ^= self.get_indirect_y_value(false);
        self.set_flags_nz(self.accumulator);
        (2, 5)
    }

    /// Function call for EOR ($xx), Y. Indirect, Y
    fn fn_0x51_with_no_additionnal_cycles(&mut self) -> (u16, u32) {
        self.accumulator ^= self.get_indirect_y_value(true);
        self.set_flags_nz(self.accumulator);
        (2, 5)
    }

    /// Function call for CLC. Implied
    /// Clear carry flag
    fn fn_0x18(&mut self) -> (u16, u32) {
        self.carry = false;
        (1, 2)
    }

    /// Function call for SEC. Implied
    /// Set carry flag
    fn fn_0x38(&mut self) -> (u16, u32) {
        self.carry = true;
        (1, 2)
    }

    /// Function call for CLI. Implied
    /// Clear interrupt flag
    fn fn_0x58(&mut self) -> (u16, u32) {
        self.interrupt = false;
        (1, 2)
    }

    /// Function call for SEI. Implied
    /// Set interrupt flag
    fn fn_0x78(&mut self) -> (u16, u32) {
        self.interrupt = true;
        (1, 2)
    }

    /// Function call for CLV. Implied
    /// Clear overflow flag
    fn fn_0xb8(&mut self) -> (u16, u32) {
        self.overflow = false;
        (1, 2)
    }

    /// Function call for CLD. Implied
    /// Clear decimal flag
    fn fn_0xd8(&mut self) -> (u16, u32) {
        self.decimal = false;
        (1, 2)
    }

    /// Function call for SED. Implied
    /// Set decimal flag
    fn fn_0xf8(&mut self) -> (u16, u32) {
        self.decimal = true;
        (1, 2)
    }

    /// Function call for INC $xx. Zero Page
    fn fn_0xe6(&mut self) -> (u16, u32) {
        let value = self.get_zero_page_value();
        let value = value + 1;
        self.set_zero_page(value);
        self.set_flags_nz(value);
        (2, 5)
    }

    /// Function call for INC $xx, X. Zero Page, X
    fn fn_0xf6(&mut self) -> (u16, u32) {
        let value = self.get_zero_page_x_value();
        let value = value + 1;
        self.set_zero_page_x(value);
        self.set_flags_nz(value);
        (2, 6)
    }

    /// Function call for INC $xxxx. Absolute
    fn fn_0xee(&mut self) -> (u16, u32) {
        let value = self.get_absolute_value();
        let value = value + 1;
        self.set_absolute(value);
        self.set_flags_nz(value);
        (3, 6)
    }

    /// Function call for INC $xxxx, X. Absolute, X
    fn fn_0xfe(&mut self) -> (u16, u32) {
        let value = self.get_absolute_x_value(true);
        let value = value + 1;
        self.set_absolute_x(value, true);
        self.set_flags_nz(value);
        (3, 7)
    }

    /// Function call for JMP $xxxx. Absolute
    fn fn_0x4c(&mut self) -> (u16, u32) {
        self.program_counter = self.get_absolute_address();
        (0, 3)
    }

    /// Function call for JMP ($xxxx). Indirect
    fn fn_0x6c(&mut self) -> (u16, u32) {
        let mut address = self.get_absolute_address();
        if address & 0xFF == 0xFF {
            // Strange behaviour in nestest.nes where direct jump to re-aligned address where address at end of page
            address = address + 1;
        } else {
            address = self.memory.borrow_mut().read_rom_16(address);
        }
        self.program_counter = address;
        (0, 5)
    }

    /// Function call for JSR $xxxx. Absolute
    fn fn_0x20(&mut self) -> (u16, u32) {
        let program_counter = self.program_counter + 2;
        let high = (program_counter >> 8) as u8;
        let low = (program_counter & 255) as u8;
        self.push(high); // little endian
        self.push(low);
        self.program_counter = self.get_absolute_address();
        (0, 6)
    }

    /// Function call for LDA #$xx. Immediate
    fn fn_0xa9(&mut self) -> (u16, u32) {
        self.accumulator = self.get_immediate();
        self.set_flags_nz(self.accumulator);
        (2, 2)
    }

    /// Function call for LDA $xx. Zero Page
    fn fn_0xa5(&mut self) -> (u16, u32) {
        self.accumulator =self.get_zero_page_value();
        self.set_flags_nz(self.accumulator);
        (2, 3)
    }

    /// Function call for LDA $xx, X. Zero Page, X
    fn fn_0xb5(&mut self) -> (u16, u32) {
        self.accumulator = self.get_zero_page_x_value();
        self.set_flags_nz(self.accumulator);
        (2, 4)
    }

    /// Function call for LDA $xxxx. Absolute
    fn fn_0xad(&mut self) -> (u16, u32) {
        self.accumulator = self.get_absolute_value();
        self.set_flags_nz(self.accumulator);
        (3, 4)
    }

    /// Function call for LDA $xxxx, X. Absolute, X
    fn fn_0xbd(&mut self) -> (u16, u32) {
        self.accumulator = self.get_absolute_x_value(true);
        self.set_flags_nz(self.accumulator);
        (3, 4)
    }

    /// Function call for LDA $xxxx, Y. Absolute, Y
    fn fn_0xb9(&mut self) -> (u16, u32) {
        self.accumulator = self.get_absolute_y_value(true);
        self.set_flags_nz(self.accumulator);
        (3, 4)
    }

    /// Function call for LDA ($xx, X). Indirect, X
    fn fn_0xa1(&mut self) -> (u16, u32) {
        self.accumulator = self.get_indirect_x_value();
        self.set_flags_nz(self.accumulator);
        (2, 6)
    }

    /// Function call for LDA ($xx), Y. Indirect, Y
    fn fn_0xb1(&mut self) -> (u16, u32) {
        self.accumulator = self.get_indirect_y_value(true);
        self.set_flags_nz(self.accumulator);
        (2, 5)
    }

    /// Function call for LDX #$xx. Immediate
    fn fn_0xa2(&mut self) -> (u16, u32) {
        self.x_register = self.get_immediate();
        self.set_flags_nz(self.x_register);
        (2, 2)
    }

    /// Function call for LDX $xx. Zero Page
    fn fn_0xa6(&mut self) -> (u16, u32) {
        self.x_register = self.get_zero_page_value();
        self.set_flags_nz(self.x_register);
        (2, 3)
    }

    /// Function call for LDX $xx, Y. Zero Page, Y
    fn fn_0xb6(&mut self) -> (u16, u32) {
        self.x_register = self.get_zero_page_y_value();
        self.set_flags_nz(self.x_register);
        (2, 4)
    }

    /// Function call for LDX $xxxx. Absolute
    fn fn_0xae(&mut self) -> (u16, u32) {
        self.x_register = self.get_absolute_value();
        self.set_flags_nz(self.x_register);
        (3, 4)
    }

    /// Function call for LDX $xxxx, Y. Absolute, Y
    fn fn_0xbe(&mut self) -> (u16, u32) {
        self.x_register = self.get_absolute_y_value(true);
        self.set_flags_nz(self.x_register);
        (3, 4)
    }

    /// Function call for LDY #$xx. Immediate
    fn fn_0xa0(&mut self) -> (u16, u32) {
        self.y_register = self.get_immediate();
        self.set_flags_nz(self.y_register);
        (2, 2)
    }

    /// Function call for LDY $xx. Zero Page
    fn fn_0xa4(&mut self) -> (u16, u32) {
        self.y_register = self.get_zero_page_value();
        self.set_flags_nz(self.x_register);
        (2, 3)
    }

    /// Function call for LDY $xx, X. Zero Page, X
    fn fn_0xb4(&mut self) -> (u16, u32) {
        self.y_register = self.get_zero_page_x_value();
        self.set_flags_nz(self.y_register);
        (2, 4)
    }

    /// Function call for LDY $xxxx. Absolute(&mut self)
    fn fn_0xac(&mut self) -> (u16, u32) {
        self.y_register =self.get_absolute_value();
        self.set_flags_nz(self.y_register);
        (3, 4)
    }

    /// Function call for LDY $xxxx, X. Absolute, X
    fn fn_0xbc(&mut self) -> (u16, u32) {
        self.y_register = self.get_absolute_x_value(true);
        self.set_flags_nz(self.y_register);
        (3, 4)
    }

    /// Function call for LSR. Accumulator
    fn fn_0x4a(&mut self) -> (u16, u32) {
        self.carry = self.accumulator == 1;
        self.accumulator = self.accumulator >> 1;
        self.set_flags_nz(self.accumulator);
        (1, 2)
    }

    /// Function call for LSR $xx. Zero Page
    fn fn_0x46(&mut self) -> (u16, u32) {
        let value = self.get_zero_page_value();
        self.carry = value == 1;
        let value = value >> 1;
        self.set_zero_page(value);
        self.set_flags_nz(value);
        (2, 5)
    }

    /// Function call for LSR $xx, X. Zero Page, X
    fn fn_0x56(&mut self) -> (u16, u32) {
        let value = self.get_zero_page_x_value();
        self.carry = value == 1;
        let value = value >> 1;
        self.set_zero_page_x(value);
        self.set_flags_nz(value);
        (2, 6)
    }

    /// Function call for LSR $xxxx. Absolute
    fn fn_0x4e(&mut self) -> (u16, u32) {
        let value = self.get_absolute_value();
        self.carry = value == 1;
        let value = value >> 1;
        self.set_absolute(value);
        self.set_flags_nz(value);
        (3, 6)
    }

    /// Function call for LSR $xxxx, X. Absolute, X
    fn fn_0x5e(&mut self) -> (u16, u32) {
        let value = self.get_absolute_x_value(true);
        self.carry = value == 1;
        let value = value >> 1;
        self.set_absolute_x(value, true);
        self.set_flags_nz(value);
        (3, 7)
    }

    /// Function call for LSR $xxxx, X. Absolute, X
    fn fn_0x5e_with_no_additionnal_cycles(&mut self) -> (u16, u32) {
        let value = self.get_absolute_x_value(false);
        self.carry = value == 1;
        let value = value >> 1;
        self.set_absolute_x(value, false);
        self.set_flags_nz(value);
        (3, 7)
    }

    /// Function call for NOP. Implied
    fn fn_0xea(&mut self) -> (u16, u32) {
        (1, 2)
    }

    /// Function call for NOP. Implied
    fn fn_0x1a(&mut self) -> (u16, u32) {
        (1, 2)
    }

    /// Function call for NOP. Implied
    fn fn_0x3a(&mut self) -> (u16, u32) {
        (1, 2)
    }

    /// Function call for NOP. Implied
    fn fn_0x5a(&mut self) -> (u16, u32) {
        (1, 2)
    }

    /// Function call for NOP. Implied
    fn fn_0x7a(&mut self) -> (u16, u32) {
        (1, 2)
    }

    /// Function call for NOP. Implied
    fn fn_0xda(&mut self) -> (u16, u32) {
        (1, 2)
    }

    /// Function call for NOP. Implied
    fn fn_0xfa(&mut self) -> (u16, u32) {
        (1, 2)
    }

    /// Function call for DOP. Implied
    /// Equivalent to NOP NOP (2-byte NOP)
    fn fn_0x04(&mut self) -> (u16, u32) {
        (2, 3)
    }

    /// Function call for DOP. Implied
    /// Equivalent to NOP NOP (2-byte NOP)
    fn fn_0x14(&mut self) -> (u16, u32) {
        (2, 4)
    }

    /// Function call for DOP. Implied
    /// Equivalent to NOP NOP (2-byte NOP)
    fn fn_0x34(&mut self) -> (u16, u32) {
        (2, 4)
    }

    /// Function call for DOP. Implied
    /// Equivalent to NOP NOP (2-byte NOP)
    fn fn_0x44(&mut self) -> (u16, u32) {
        (2, 3)
    }

    /// Function call for DOP. Implied
    /// Equivalent to NOP NOP (2-byte NOP)
    fn fn_0x54(&mut self) -> (u16, u32) {
        (2, 4)
    }

    /// Function call for DOP. Implied
    /// Equivalent to NOP NOP (2-byte NOP)
    fn fn_0x64(&mut self) -> (u16, u32) {
        (2, 3)
    }

    /// Function call for DOP. Implied
    /// Equivalent to NOP NOP (2-byte NOP)
    fn fn_0x74(&mut self) -> (u16, u32) {
        (2, 4)
    }

    /// Function call for DOP. Implied
    /// Equivalent to NOP NOP (2-byte NOP)
    fn fn_0x80(&mut self) -> (u16, u32) {
        (2, 2)
    }

    /// Function call for DOP. Implied
    ///Equivalent to NOP NOP (2-byte NOP)
    fn fn_0x82(&mut self) -> (u16, u32) {
        (2, 2)
    }

    /// Function call for DOP. Implied
    /// Equivalent to NOP NOP (2-byte NOP)
    fn fn_0x89(&mut self) -> (u16, u32) {
        (2, 2)
    }

    /// Function call for DOP. Implied
    /// Equivalent to NOP NOP (2-byte NOP)
    fn fn_0xc2(&mut self) -> (u16, u32) {
        (2, 2)
    }

    /// Function call for DOP. Implied
    ///Equivalent to NOP NOP (2-byte NOP)
    fn fn_0xd4(&mut self) -> (u16, u32) {
        (2, 4)
    }

    /// Function call for DOP. Implied
    /// Equivalent to NOP NOP (2-byte NOP)
    fn fn_0xe2(&mut self) -> (u16, u32) {
        (2, 2)
    }

    /// Function call for DOP. Implied
    /// Equivalent to NOP NOP (2-byte NOP)
    fn fn_0xf4(&mut self) -> (u16, u32) {
        (2, 4)
    }

    /// Function call for TOP. Implied
    ///Equivalent to NOP NOP NOP (3-byte NOP)
    fn fn_0x0c(&mut self) -> (u16, u32) {
        (3, 4)
    }

    /// Function call for TOP. Implied
    /// Equivalent to NOP NOP NOP (3-byte NOP)
    fn fn_0x1c(&mut self) -> (u16, u32) {
        self.get_absolute_x_value(true); // Need extra cycle
        (3, 4)
    }

    /// Function call for TOP. Implied
    ///Equivalent to NOP NOP NOP (3-byte NOP)
    fn fn_0x3c(&mut self) -> (u16, u32) {
        self.get_absolute_x_value(true); // Need extra cycle
        (3, 4)
    }

    /// Function call for TOP. Implied
    ///Equivalent to NOP NOP NOP (3-byte NOP)
    fn fn_0x5c(&mut self) -> (u16, u32) {
        self.get_absolute_x_value(true); // Need extra cycle
        (3, 4)
    }

    /// Function call for TOP. Implied
    ///Equivalent to NOP NOP NOP (3-byte NOP)
    fn fn_0x7c(&mut self) -> (u16, u32) {
        self.get_absolute_x_value(true); // Need extra cycle
        (3, 4)
    }

    /// Function call for TOP. Implied
    /// Equivalent to NOP NOP NOP (3-byte NOP)
    fn fn_0xdc(&mut self) -> (u16, u32) {
        self.get_absolute_x_value(true); // Need extra cycle
        (3, 4)
    }

    /// Function call for TOP. Implied
    ///Equivalent to NOP NOP NOP (3-byte NOP)
    fn fn_0xfc(&mut self) -> (u16, u32) {
        self.get_absolute_x_value(true); // Need extra cycle
        (3, 4)
    }

    /// Function call for ORA #$xx. Immediate
    fn fn_0x09(&mut self) -> (u16, u32) {
        self.accumulator |= self.get_immediate();
        self.set_flags_nz(self.accumulator);
        (2, 2)
    }

    /// Function call for ORA $xx. Zero Page
    fn fn_0x05(&mut self) -> (u16, u32) {
        self.accumulator |= self.get_zero_page_value();
        self.set_flags_nz(self.accumulator);
        (2, 3)
    }

    /// Function call for ORA $xx, X. Zero Page, X
    fn fn_0x15(&mut self) -> (u16, u32) {
        self.accumulator |= self.get_zero_page_x_value();
        self.set_flags_nz(self.accumulator);
        (2, 4)
    }

    /// Function call for ORA $xxxx. Absolute
    fn fn_0x0d(&mut self) -> (u16, u32) {
        self.accumulator |= self.get_absolute_value();
        self.set_flags_nz(self.accumulator);
        (3, 4)
    }

    /// Function call for ORA $xxxx, X. Absolute, X
    fn fn_0x1d(&mut self) -> (u16, u32) {
        self.accumulator |= self.get_absolute_x_value(true);
        self.set_flags_nz(self.accumulator);
        (3, 4)
    }

    /// Function call for ORA $xxxx, X. Absolute, X
    fn fn_0x1d_with_no_additionnal_cycles(&mut self) -> (u16, u32) {
        self.accumulator |= self.get_absolute_x_value(false);
        self.set_flags_nz(self.accumulator);
        (3, 4)
    }

    /// Function call for ORA $xxxx, Y. Absolute, Y
    fn fn_0x19(&mut self) -> (u16, u32) {
        self.accumulator |= self.get_absolute_y_value(true);
        self.set_flags_nz(self.accumulator);
        (3, 4)
    }

    /// Function call for ORA $xxxx, Y. Absolute, Y
    fn fn_0x19_with_no_additionnal_cycles(&mut self) -> (u16, u32) {
        self.accumulator |= self.get_absolute_y_value(false);
        self.set_flags_nz(self.accumulator);
        (3, 4)
    }

    /// Function call for ORA ($xx, X). Indirect, X
    fn fn_0x01(&mut self) -> (u16, u32) {
        self.accumulator |= self.get_indirect_x_value();
        self.set_flags_nz(self.accumulator);
        (2, 6)
    }

    /// Function call for ORA ($xx), Y. Indirect, Y
    fn fn_0x11(&mut self) -> (u16, u32) {
        self.accumulator |= self.get_indirect_y_value(true);
        self.set_flags_nz(self.accumulator);
        (2, 5)
    }

    /// Function call for ORA ($xx), Y. Indirect, Y
    fn fn_0x11_with_no_additionnal_cycles(&mut self) -> (u16, u32) {
        self.accumulator |= self.get_indirect_y_value(false);
        self.set_flags_nz(self.accumulator);
        (2, 5)
    }

    /// Function call for SLO $xx. Zero Page
    /// Equivalent to:
    ///     ASL
    ///     ORA
    fn fn_0x07(&mut self) -> (u16, u32) {
        self.fn_0x06(); // ASL
        self.fn_0x05(); // ORA
        (2, 5)
    }

    /// Function call for SLO $xx, X. Zero Page, X
    /// Equivalent to:
    ///     ASL
    ///     ORA
    fn fn_0x17(&mut self) -> (u16, u32) {
        self.fn_0x16(); // ASL
        self.fn_0x15(); // ORA
        (2, 6)
    }

    /// Function call for SLO $xxxx. Absolute
    /// Equivalent to:
    ///     ASL
    ///     ORA
    fn fn_0x0f(&mut self) -> (u16, u32) {
        self.fn_0x0e(); // ASL
        self.fn_0x0d(); // ORA
        (3, 6)
    }

    /// Function call for SLO $xxxx, X. Absolute, X
    /// Equivalent to:
    ///     ASL
    ///     ORA
    fn fn_0x1f(&mut self) -> (u16, u32) {
        self.fn_0x1e_with_no_additionnal_cycles(); // ASL
        self.fn_0x1d_with_no_additionnal_cycles(); // ORA
        (3, 7)
    }

    /// Function call for SLO $xxxx, Y. Absolute, Y
    /// Equivalent to:
    ///     ASL
    ///     ORA
    fn fn_0x1b(&mut self) -> (u16, u32) {
        value = self.get_absolute_y_value(false);
        self.carry = (value >> 7) != 0;
        value = (value << 1) & 0b11111111;
        self.set_absolute_y(value, false);
        self.fn_0x19_with_no_additionnal_cycles(); // ORA
        (3, 7)
    }

    /// Function call for SLO ($xx, X). Indirect, X
    /// Equivalent to:
    ///     ASL
    ///     ORA
    fn fn_0x03(&mut self) -> (u16, u32) {
        value = self.get_indirect_x_value();
        self.carry = (value >> 7) != 0;
        value = (value << 1) & 0b11111111;
        self.set_indirect_x(value);
        self.fn_0x01(); // ORA
        (2, 8)
    }

    /// Function call for SLO ($xx), Y. Indirect, Y
    /// Equivalent to:
    ///     ASL
    ///     ORA
    fn fn_0x13(&mut self) -> (u16, u32) {
        value = self.get_indirect_y_value(false);
        self.carry = (value >> 7) != 0;
        value = (value << 1) & 0b11111111;
        self.set_indirect_y(value, false);
        self.fn_0x11_with_no_additionnal_cycles(); // ORA
        (2, 8)
    }

    /// Function call for RLA $xx. Zero Page
    /// Equivalent to:
    ///     ROL
    ///     AND
    fn fn_0x27(&mut self) -> (u16, u32) {
        self.fn_0x26(); // ROL
        self.fn_0x25(); // AND
        (2, 5)
    }

    /// Function call for RLA $xx, X. Zero Page, X
    /// Equivalent to:
    ///     ROL
    ///     AND
    fn fn_0x37(&mut self) -> (u16, u32) {
        self.fn_0x36(); // ROL
        self.fn_0x35(); // AND
        (2, 6)
    }

    /// Function call for for RLA $xxxx. Absolute
    /// Equivalent to:
    ///     ROL
    ///     AND
    fn fn_0x2f(&mut self) -> (u16, u32) {
        self.fn_0x2e(); // ROL
        self.fn_0x2d(); // AND
        (3, 6)
    }

    /// Function call for RLA $xxxx, X. Absolute, X
    /// Equivalent to:
    ///     ROL
    ///     AND
    fn fn_0x3f(&mut self) -> (u16, u32) {
        self.fn_0x3e_with_no_additionnal_cycles(); // ROL
        self.fn_0x3d_with_no_additionnal_cycles(); // AND
        (3, 7)
    }

    /// Function call for RLA $xxxx, Y. Absolute, Y
    /// Equivalent to:
    ///     ROL
    ///     AND
    fn fn_0x3b(&mut self) -> (u16, u32) {
        let val = self.get_absolute_y_value(false);
        let val = (val << 1) | (self.carry as u8);
        self.carry = (val >> 8) != 0;
        let val &= 255;
        self.set_absolute_y(val, false);
        self.fn_0x39_with_no_additionnal_cycles(); // AND
        (3, 7)
    }

    /// Function call for RLA ($xx, X). Indirect, X
    /// Equivalent to:
    ///     ROL
    ///     AND
    fn fn_0x23(&mut self) -> (u16, u32) {
        let val = self.get_indirect_x_value();
        let val = (val << 1) | (self.carry as u8);
        self.carry = (val >> 8) != 0;
        let val &= 255;
        self.set_indirect_x(val);
        self.fn_0x21(); // AND
        (2, 8)
    }

    /// Function call for RLA ($xx), Y. Indirect, Y
    /// Equivalent to:
    ///     ROL
    ///     AND
    fn fn_0x33(&mut self) -> (u16, u32)
        let val = self.get_indirect_y_value(false);
        let val = (val << 1) | (self.carry as u8);
        self.carry = (val >> 8) != 0;
        let val &= 255;
        self.set_indirect_y(val, false);
        self.fn_0x31_with_no_additionnal_cycles(); // AND
        (2, 8)
    }

    /// Function call for RRA $xx. Zero Page
    /// Equivalent to:
    ///     ROR
    ///     AND
    fn fn_0x67(&mut self) -> (u16, u32) {
        self.fn_0x66(); // ROR
        self.fn_0x65(); // ADC
        (2, 5)
    }

    /// Function call for RRA $xx, X. Zero Page, X
    /// Equivalent to:
    ///     ROR
    ///     AND
    fn fn_0x77(&mut self) -> (u16, u32) {
        self.fn_0x76(); // ROR
        self.fn_0x75(); // ADC
        (2, 6)
    }

    /// Function call for RRA $xxxx. Absolute
    /// Equivalent to:
    ///     ROR
    ///     AND
    fn fn_0x6f(&mut self) -> (u16, u32) {
        self.fn_0x6e(); // ROR
        self.fn_0x6d(); // ADC
        (3, 6)
    }

    /// Function call for RRA $xxxx, X. Absolute, X
    /// Equivalent to:
    ///     ROR
    ///     AND
    fn fn_0x7f(&mut self) -> (u16, u32) {
        self.fn_0x7e_with_no_additionnal_cycles(); // ROR
        self.fn_0x7d_with_no_additionnal_cycles(); // ADC
        (3, 7)

        /// Function call for RRA $xxxx, Y. Absolute, Y
        /// Equivalent to:
        ///     ROR
        ///     AND
    fn fn_0x7b(&mut self) -> (u16, u32) {
        let val = self.get_absolute_y_value(false);
        let carry = val & 1;
        let val = (val >> 1) | (self.carry as u8 << 7);
        self.carry = carry != 0;
        self.set_absolute_y(val, false);
        self.fn_0x79_with_no_additionnal_cycles(); // ADC
        (3, 7)
    }

    /// Function call for RRA ($xx, X). Indirect, X
    /// Equivalent to:
    ///     ROR
    ///     AND
    fn fn_0x63(&mut self) -> (u16, u32) {
        let val = self.get_indirect_x_value();
        let carry = val & 1;
        let val = (val >> 1) | (self.carry as u8 << 7);
        self.carry = carry != 0;
        self.set_indirect_x(val);
        self.fn_0x61(); // ADC
        (2, 8)
    }

    /// Function call for RRA ($xx), Y. Indirect, Y
    /// Equivalent to:
    ///     ROR
    ///     AND
    fn fn_0x73(&mut self) -> (u16, u32) {
        let val = self.get_indirect_y_value(false);
        let carry = val & 1;
        let val = (val >> 1) | (self.carry as u8 << 7);
        self.carry = carry != 0;
        self.set_indirect_y(val, false);
        self.fn_0x71_with_no_additionnal_cycles(); // ADC
        (2, 8)
    }

    /// Function call for SRE $xx. Zero Page
    /// Equivalent to:
    ///     LSR
    ///     EOR
    fn fn_0x47(&mut self) -> (u16, u32) {
        self.fn_0x46(); // LSR
        self.fn_0x45(); // EOR
        (2, 5)
    }

    /// Function call for SRE $xx, X. Zero Page, X
    /// Equivalent to:
    ///     LSR
    ///     EOR
    fn fn_0x57(&mut self) -> (u16, u32) {
        self.fn_0x56(); // LSR
        self.fn_0x55(); // EOR
        (2, 6)
    }

    /// Function call for SRE $xxxx. Absolute
    /// Equivalent to:
    ///     LSR
    ///     EOR
    fn fn_0x4f(&mut self) -> (u16, u32) {
        self.fn_0x4e(); // LSR
        self.fn_0x4d(); // EOR
        (3, 6)
    }

    /// Function call for SRE $xxxx, X. Absolute, X
    /// Equivalent to:
    ///     LSR
    ///     EOR
    fn fn_0x5f(&mut self) -> (u16, u32) {
        self.fn_0x5e_with_no_additionnal_cycles(); // LSR
        self.fn_0x5d_with_no_additionnal_cycles(); // EOR
        (3, 7)
    }

    /// Function call for SRE $xx. Zero Page
    /// Equivalent to:
    ///     LSR
    ///     EOR
    fn fn_0x5b(&mut self) -> (u16, u32) {
        let val = self.get_absolute_y_value(false);
        self.carry = (val & 1) != 0;
        let val = val >> 1;
        self.set_absolute_y(val, false);
        self.fn_0x59_with_no_additionnal_cycles(); // EOR
        (3, 7)
    }

    /// Function call for SRE ($xx, X). Indirect, X
    /// Equivalent to:
    ///     LSR
    ///     EOR
    fn fn_0x43(&mut self) -> (u16, u32) {
        let val = self.get_indirect_x_value();
        self.carry = (val & 1) != 0;
        let val = val >> 1;
        self.set_indirect_x(val);
        self.fn_0x41(); // EOR
        (2, 8)
    }

    /// Function call for SRE ($xx), Y. Indirect, Y
    /// Equivalent to:
    ///     LSR
    ///     EOR
    fn fn_0x53(&mut self) -> (u16, u32) {
        let val = self.get_indirect_y_value(false);
        self.carry = (val & 1) != 0;
        let val = val >> 1;
        self.set_indirect_y(val, false);
        self.fn_0x51_with_no_additionnal_cycles(); // EOR
        (2, 8)
    }

    /// Function call for TAX. Implied
    fn fn_0xaa(&mut self) -> (u16, u32) {
        self.x_register = self.accumulator;
        self.set_flags_nz(self.x_register);
        (1, 2)
    }

    /// Function call for TXA. Implied
    fn fn_0x8a(&mut self) -> (u16, u32) {
        self.accumulator = self.x_register;
        self.set_flags_nz(self.accumulator);
        (1, 2)
    }

    /// Function call for DEX. Implied
    fn fn_0xca(&mut self) -> (u16, u32) {
        self.x_register = self.x_register - 1;
        self.set_flags_nz(self.x_register);
        (1, 2)
    }

    /// Function call for INX. Implied
    fn fn_0xe8(&mut self) -> (u16, u32) {
        self.x_register = self.x_register + 1;
        self.set_flags_nz(self.x_register);
        (1, 2)
    }

    /// Function call for TAY. Implied
    fn fn_0xa8(&mut self) -> (u16, u32) {
        self.y_register = self.accumulator;
        self.set_flags_nz(self.y_register);
        (1, 2)
    }

    /// Function call for TYA. Implied
    fn fn_0x98(&mut self) -> (u16, u32) {
        self.accumulator = self.y_register;
        self.set_flags_nz(self.accumulator);
        (1, 2)
    }

    /// Function call for DEY. Implied
    fn fn_0x88(&mut self) -> (u16, u32) {
        self.y_register = self.y_register - 1;
        self.set_flags_nz(self.y_register);
        (1, 2)
    }

    /// Function call for INY. Implied
    fn fn_0xc8(&mut self) -> (u16, u32) {
        self.y_register = self.y_register + 1;
        self.set_flags_nz(self.y_register);
        (1, 2)
    }

    /// Function call for ROL A. Accumulator
    fn fn_0x2a(&mut self) -> (u16, u32) {
        self.accumulator = (self.accumulator << 1) | (self.carry as u8);
        self.carry = (self.accumulator >> 8) != 0;
        self.accumulator &= 255;
        self.set_flags_nz(self.accumulator);
        (1, 2)
    }

    /// Function call for ROL $xx. Zero Page
    fn fn_0x26(&mut self) -> (u16, u32) {
        let val = self.get_zero_page_value();
        let val = (val << 1) | (self.carry as u8);
        self.carry = (val >> 8) != 0;;
        let val &= 255;
        self.set_zero_page(val);
        self.set_flags_nz(val);
        (2, 5)
    }

    /// Function call for ROL $xx, X. Zero Page, X
    fn fn_0x36(&mut self) -> (u16, u32) {
        let val = self.get_zero_page_x_value();
        let val = (val << 1) | (self.carry as u8);
        self.carry = (val >> 8) != 0;
        let val &= 255;
        self.set_zero_page_x(val);
        self.set_flags_nz(val);
        (2, 6)
    }

    /// Function call for ROL $xxxx. Absolute
    fn fn_0x2e(&mut self) -> (u16, u32) {
        let val = self.get_absolute_value();
        let val = (val << 1) | (self.carry as u8);
        self.carry = (val >> 8) != 0;
        let val &= 255;
        self.set_absolute(val);
        self.set_flags_nz(val);
        (3, 6)
    }

    /// Function call for ROL $xxxx, X. Absolute, X
    fn fn_0x3e(&mut self) -> (u16, u32) {
        let val = self.get_absolute_x_value(true);
        let val = (val << 1) | (self.carry as u8);
        self.carry = (val >> 8) != 0;
        let val &= 255;
        self.set_absolute_x(val, true);
        self.set_flags_nz(val);
        (3, 7)
    }

    /// Function call for ROL $xxxx, X. Absolute, X
    fn fn_0x3e_with_no_additionnal_cycles(&mut self) -> (u16, u32) {
        let val = self.get_absolute_x_value(false);
        let val = (val << 1) | (self.carry as u8);
        self.carry = (val >> 8) != 0;
        let val &= 255;
        self.set_absolute_x(val, false);
        self.set_flags_nz(val);
        (3, 7)
    }

    /// Function call for ROR A. Accumulator
    fn fn_0x6a(&mut self) -> (u16, u32) {
        let carry = self.accumulator & 1;
        self.accumulator = (self.accumulator >> 1) | (self.carry as u8 << 7);
        self.carry = carry != 0;
        self.set_flags_nz(self.accumulator);
        (1, 2)
    }

    /// Function call for ROR $xx. Zero Page
    fn fn_0x66(&mut self) -> (u16, u32) {
        let val = self.get_zero_page_value();
        let carry = val & 1;
        let val = (val >> 1) | (self.carry as u8 << 7);
        self.carry = carry != 0;
        self.set_zero_page(val);
        self.set_flags_nz(val);
        (2, 5)
    }

    /// Function call for ROR $xx, X. Zero Page, X
    fn fn_0x76(&mut self) -> (u16, u32) {
        let val = self.get_zero_page_x_value();
        let carry = val & 1;
        let val = (val >> 1) | (self.carry as u8 << 7);
        self.carry = carry != 0;
        self.set_zero_page_x(val);
        self.set_flags_nz(val);
        (2, 6)
    }

    /// Function call for ROR $xxxx. Absolute
    fn fn_0x6e(&mut self) -> (u16, u32) {
        let val = self.get_absolute_value();
        let carry = val & 1;
        let val = (val >> 1) | (self.carry as u8 << 7);
        self.carry = carry != 0;
        self.set_absolute(val);
        self.set_flags_nz(val);
        (3, 6)
    }

    /// Function call for ROR$xxxx, X. Absolute, X
    fn fn_0x7e(&mut self) -> (u16, u32) {
        let val = self.get_absolute_x_value(true);
        let carry = val & 1;
        let val = (val >> 1) | (self.carry as u8 << 7);
        self.carry = carry != 0;
        self.set_absolute_x(val, true);
        self.set_flags_nz(val);
        (3, 7)
    }

    /// Function call for ROR$xxxx, X. Absolute, X
    fn fn_0x7e_with_no_additionnal_cycles(&mut self) -> (u16, u32) {
        let val = self.get_absolute_x_value(false);
        let carry = val & 1;
        let val = (val >> 1) | (self.carry as u8 << 7);
        self.carry = carry != 0;
        self.set_absolute_x(val, false);
        self.set_flags_nz(val);
        (3, 7)
    }

    /// Function call for RTI. Implied
    fn fn_0x40(&mut self) -> (u16, u32) {
        self.set_status_register(self.pop());
        let low = self.pop() as u16;
        let high = self.pop() as u16;
        self.program_counter = (high << 8) + low;
        (0, 6)
    }

    /// Function call for RTS. Implied
    fn fn_0x60(&mut self) -> (u16, u32) {
        let low = self.pop() as u16;
        let high = self.pop() as u16;
        self.program_counter = (high << 8) + low + 1; // JSR increment only by two, and RTS add the third
        (0, 6)
    }

    /// General implementation for sbc operation
    ///
    /// SBC is the same as ADC with two's complement on second operand
    fn sbc(&mut self, value: u8) {
        self.adc(255 - value);
    }

    /// Function call for SBC #$xx. Immediate
    fn fn_0xe9(&mut self) -> (u16, u32) {
        self.sbc(self.get_immediate());
        (2, 2)
    }

    /// Function call for SBC #$xx. Immediate
    /// 0xeb alias to 0xe9
    fn fn_0xeb(&mut self) -> (u16, u32) {
        self.fn_0xe9()
    }

    /// Function call for SBC $xx. Zero Page
    fn fn_0xe5(&mut self) -> (u16, u32) {
        self.sbc(self.get_zero_page_value());
        (2, 3)
    }

    /// Function call for SBC $xx, X. Zero Page, X
    fn fn_0xf5(&mut self) -> (u16, u32) {
        self.sbc(self.get_zero_page_x_value());
        (2, 4)
    }

    /// Function call for SBC $xxxx. Absolute
    fn fn_0xed(&mut self) -> (u16, u32) {
        self.sbc(self.get_absolute_value());
        (3, 4)
    }

    /// Function call for SBC $xxxx, X. Absolute, X
    fn fn_0xfd(&mut self) -> (u16, u32) {
        self.sbc(self.get_absolute_x_value());
        (3, 4)
    }

    /// Function call for SBC $xxxx, Y. Absolute, Y
    fn fn_0xf9(&mut self) -> (u16, u32) {
        self.sbc(self.get_absolute_y_value());
        (3, 4)
    }

    /// Function call for SBC ($xx, X). Indirect, X
    fn fn_0xe1(&mut self) -> (u16, u32) {
        self.sbc(self.get_indirect_x_value());
        (2, 6)
    }

    /// Function call for STA $xx. Zero Page
    fn fn_0x85(&mut self) -> (u16, u32) {
        let address = self.get_zero_page_address();
        let extra_cycles = self.memory.borrow_mut().write_rom(address, self.accumulator);
        (2, 3 + extra_cycles)
    }

    /// Function call for STA $xx, X. Zero Page, X
    fn fn_0x95(&mut self) -> (u16, u32) {
        let address = self.get_zero_page_x_address();
        let extra_cycles = self.memory.borrow_mut().write_rom(address, self.accumulator);
        (2, 4 + extra_cycles)
    }

    /// Function call for STA $xxxx. Absolute
    fn fn_0x8d(&mut self) -> (u16, u32) {
        let address = self.get_absolute_address();
        let extra_cycles = self.memory.borrow_mut().write_rom(address, self.accumulator);
        (3, 4 + extra_cycles)
    }

    /// Function call for STA $xxxx, X. Absolute, X
    fn fn_0x9d(&mut self) -> (u16, u32) {
        let address = self.get_absolute_x_address(false); // No additionnal cycles on STA
        let extra_cycles = self.memory.borrow_mut().write_rom(address, self.accumulator);
        (3, 5 + extra_cycles)
    }

    /// Function call for STA $xxxx, Y. Absolute, Y
    fn fn_0x99(&mut self) -> (u16, u32) {
        let address = self.get_absolute_y_address(false); // No additionnal cycles on STA
        let extra_cycles = self.memory.borrow_mut().write_rom(address, self.accumulator);
        (3, 5 + extra_cycles)
    }

    /// Function call for STA ($xx, X). Indirect, X
    fn fn_0x81(&mut self) -> (u16, u32) {
        let address = self.get_indirect_x_address();
        let extra_cycles = self.memory.borrow_mut().write_rom(address, self.accumulator);
        (2, 6 + extra_cycles)
    }

    /// Function call for STA ($xx), Y. Indirect, Y
    fn fn_0x91(&mut self) -> (u16, u32) {
        let address = self.get_indirect_y_address(false); // No additionnal cycles on STA
        let extra_cycles = self.memory.borrow_mut().write_rom(address, self.accumulator);
        (2, 6 + extra_cycles)
    }

    /// Function call for TXS. Implied
    fn fn_0x9a(&mut self) -> (u16, u32) {
        self.stack_pointer = self.x_register;
        (1, 2)
    }

    /// Function call for TSX. Implied
    fn fn_0xba(&mut self) -> (u16, u32) {
        self.x_register = self.stack_pointer;
        self.set_flags_nz(self.x_register);
        (1, 2)
    }

    /// Function call for RHA. Implied
    fn fn_0x48(&mut self) -> (u16, u32) {
        self.push(self.accumulator);
        (1, 3)
    }

    /// Function call for PLA. Implied
    fn fn_0x68(&mut self) -> (u16, u32) {
        self.accumulator = self.pop();
        self.set_flags_nz(self.accumulator);
        (1, 4)
    }

    /// Function call for PHP. Implie
    fn fn_0x08(&mut self) -> (u16, u32) {
        // create status byte
        let status_register = self.get_status_register() | (1 << 4);
        self.push(status_register);
        (1, 3)
    }

    /// Function call for PLP. Implied
    fn fn_0x28(&mut self) -> (u16, u32) {
        let status_register = self.pop();
        self.set_status_register(status_register);
        (1, 4)
    }

    /// Function call for STX $xx. Zero Page
    fn fn_0x86(&mut self) -> (u16, u32) {
        let address = self.get_zero_page_address();
        self.memory.borrow_mut().write_rom(address, self.x_register);
        (2, 3)
    }

    /// Function call for STX $xx, Y. Zero Page, Y
    fn fn_0x96(&mut self) -> (u16, u32) {
        let address = self.get_zero_page_y_address();
        self.memory.borrow_mut().write_rom(address, self.x_register);
        (2, 4)
    }

    /// Function call for STX $xxxx. Absolute
    fn fn_0x8e(&mut self) -> (u16, u32) {
        let address = self.get_absolute_address();
        self.memory.borrow_mut().write_rom(address, self.x_register);
        (3, 4)
    }

    /// Function call for STY $xx. Zero Page
    fn fn_0x84(&mut self) -> (u16, u32) {
        let address = self.get_zero_page_address();
        self.memory.borrow_mut().write_rom(address, self.y_register);
        (2, 3)
    }

    /// Function call for STY $xx, X. Zero Page, X
    fn fn_0x94(&mut self) -> (u16, u32) {
        let address = self.get_zero_page_x_address();
        self.memory.borrow_mut().write_rom(address, self.y_register);
        (2, 4)
    }

    /// Function call for STY $xxxx. Absolute
    fn fn_0x8c(&mut self) -> (u16, u32) {
        let address = self.get_absolute_address();
        self.memory.borrow_mut().write_rom(address, self.y_register);
        (3, 4)
    }

    /// Function call for LAX $xx. Zero Page
    fn fn_0xa7(&mut self) -> (u16, u32) {
        self.accumulator = self.get_zero_page_value();
        self.x_register = self.accumulator;
        self.set_flags_nz(self.accumulator);
        (2, 3)
    }

    /// Function call for LAX $xx, Y. Zero Page, Y
    fn fn_0xb7(&mut self) -> (u16, u32) {
        self.accumulator = self.get_zero_page_y_value();
        self.x_register = self.accumulator;
        self.set_flags_nz(self.accumulator);
        (2, 4)
    }

    /// Function call for LAX $xxxx. Absolute
    fn fn_0xaf(&mut self) -> (u16, u32) {
        self.accumulator = self.get_absolute_value();
        self.x_register = self.accumulator;
        self.set_flags_nz(self.accumulator);
        (3, 4)
    }

    /// Function call for LAX $xxxx, Y. Absolute, Y
    fn fn_0xbf(&mut self) -> (u16, u32) {
        self.accumulator = self.get_absolute_y_value();
        self.x_register = self.accumulator;
        self.set_flags_nz(self.accumulator);
        (3, 4)
    }

    /// Function call for LAX ($xx, X). Indirect, X
    fn fn_0xa3(&mut self) -> (u16, u32) {
        self.accumulator = self.get_indirect_x_value();
        self.x_register = self.accumulator;
        self.set_flags_nz(self.accumulator);
        (2, 6)
    }

    /// Function call for LAX ($xx), Y. Indirect, Y
    fn fn_0xb3(&mut self) -> (u16, u32) {
        self.accumulator = self.get_indirect_y_value();
        self.x_register = self.accumulator;
        self.set_flags_nz(self.accumulator);
        (2, 5)
    }

    /// Function call for SAX $xx. Zero Page
    fn fn_0x87(&mut self) -> (u16, u32) {
        let val = self.accumulator & self.x_register;
        self.set_zero_page(val);
        (2, 3)
    }

    /// Function call for SAX $xx, Y. Zero Page, Y
    fn fn_0x97(&mut self) -> (u16, u32) {
        let val = self.accumulator & self.x_register;
        self.set_zero_page_y(val);
        (2, 4)
    }

    /// Function call for SAX $xxxx. Absolute
    fn fn_0x8f(&mut self) -> (u16, u32) {
        let val = self.accumulator & self.x_register;
        self.set_absolute(val);
        (3, 4)
    }

    /// Function call for SAX ($xx, X). Indirect, X
    fn fn_0x83(&mut self) -> (u16, u32) {
        let val = self.accumulator & self.x_register:
        self.set_indirect_x(val):
        (2, 6)
    }

    /// Return a dictionnary containing the current CPU Status. Usefull for debugging
    pub fn get_status(&self) -> Status {
        Status {
            program_counter : self.program_counter,
            stack_pointer : self.stack_pointer,
            accumulator : self.accumulator,
            x_register : self.x_register,
            y_register : self.y_register,
            status_register : self.get_status_register(),
            total_cycles : self.total_cycles,
        }
    }
}
