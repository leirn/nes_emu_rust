//! CPU component
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;
use crate::memory::Memory;


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
    instructions: HashMap<u8, fn(&mut Cpu) -> (u16, u32) >,

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
        
    }

    /// Dummy function to temporarly load the instruction array
    fn dummy(&mut self) -> (u16, u32) {
        panic!("Function is not implemented yet at PC = {}", self.program_counter);
        //(0, 0)
    }
    /// CPU initialisation function
    pub fn start(&mut self, entry_point: Option<u16>) {
        //!Execute 6502 Start sequence

        self.populate_instructions_vector();

        // Default is equivalent to JMP ($FFFC)
        self.program_counter = entry_point.unwrap_or(self.memory.borrow_mut().read_rom_16(0xfffc));

        println!("Entry point is {}", self.program_counter);

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

        let opcode:u8  = self.memory.borrow_mut().read_rom(self.program_counter);

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
        self.carry =        (status_register & 1) != 0;
        self.zero =         ((status_register >> 1) & 1) != 0;
        self.interrupt =    ((status_register >> 2) & 1) != 0;
        self.decimal =      ((status_register >> 3) & 1) != 0;
        //self.flagB =      (status_register >> 4) & 1;
        self.overflow =     ((status_register >> 6) & 1) != 0;
        self.negative =     ((status_register >> 7) & 1) != 0;
    }

    /// Push value into stack
    fn push(&mut self, value:u8) {
        self.memory.borrow_mut().write_rom(0x0100 | (self.stack_pointer as u16), value);
        self.stack_pointer = self.stack_pointer - 1; // Will eventually overflow on purpose
    }

    /// Pop/Pull value from stack
    fn pull(&mut self) -> u8 {
        self.stack_pointer = self.stack_pointer + 1; // Will eventually overflow on purpose
        self.memory.borrow_mut().read_rom(0x0100 | (self.stack_pointer as u16))
    }

    /// Get 8 bit immediate value on PC + 1
    fn get_immediate(&mut self) -> u8 {
        self.memory.borrow_mut().read_rom(self.program_counter+1)
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
        ((self.memory.borrow_mut().read_rom(self.program_counter+1) + self.x_register) & 255) as u16
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
          ((self.memory.borrow_mut().read_rom(self.program_counter+1) + self.y_register) & 255) as u16
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
         self.memory.borrow_mut().read_rom_16(self.program_counter+1)
    }

    /// Get val from MEMORY. Address is given as opcode 2-byte argument
    fn get_absolute_value(&mut self) -> u8 {
        let address = self.get_absolute_address();
        self.memory.borrow_mut().read_rom(address)
    }

    /// Write val into MEMORY. Address is given as opcode 2-byte argument and X register
    /// additionnal is boolean to fnine if this instruction will require extra cycles on page crossing
    fn set_absolute_x(&mut self, value: u8, is_additionnal: Option<bool>) {
        let additionnal = is_additionnal.unwrap_or(true);
        let address = self.get_absolute_x_address(Some(additionnal));
        self.memory.borrow_mut().write_rom(address, value);
    }

    /// Get address given as opcode 2-byte argument and X register
    fn get_absolute_x_address(&mut self, is_additionnal: Option<bool>) -> u16 {
        let additionnal = is_additionnal.unwrap_or(true);
        let address = self.memory.borrow_mut().read_rom_16(self.program_counter+1);
        let target_address = address + self.x_register as u16;
        if  additionnal && address & 0xFF00 != target_address & 0xff00 {
            self.additionnal_cycles += 1;
        }
        target_address
    }

    /// Get val from MEMORY. Address is given as opcode 2-byte argument and X register
    fn get_absolute_x_value(&mut self, is_additionnal: Option<bool>) -> u8 {
        let additionnal = is_additionnal.unwrap_or(true);
        let address = self.get_absolute_x_address(Some(additionnal));
        self.memory.borrow_mut().read_rom(address)
    }

    /// Write val into MEMORY. Address is given as opcode 2-byte argument and Y register
    fn set_absolute_y(&mut self, value: u8, is_additionnal: Option<bool>) {
        let additionnal = is_additionnal.unwrap_or(true);
        let address = self.get_absolute_y_address(Some(additionnal));
        self.memory.borrow_mut().write_rom(address, value);
    }

    /// Get address given as opcode 2-byte argument and Y register
    fn get_absolute_y_address(&mut self, is_additionnal: Option<bool>)-> u16 {
        let additionnal = is_additionnal.unwrap_or(true);
        let address = self.memory.borrow_mut().read_rom_16(self.program_counter+1);
        let target_address = address + self.y_register as u16;
        if additionnal && address & 0xff00 != target_address & 0xff00 {
            self.additionnal_cycles += 1;
        }
        target_address
    }

    /// Get val from MEMORY. Address is given as opcode 2-byte argument and Y register
    fn get_absolute_y_value(&mut self, is_additionnal: Option<bool>)-> u8 {
        let additionnal = is_additionnal.unwrap_or(true);
        let address = self.get_absolute_y_address(is_additionnal);
        self.memory.borrow_mut().read_rom(address)
    }

    /// Get indirect address given as opcode 2-byte argument and X register
    fn get_indirect_x_address(&mut self) -> u16 {
        let address = self.get_zero_page_x_address();
        self.memory.borrow_mut().read_rom_16_no_crossing_page(address)
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
    fn get_indirect_y_address(&mut self, is_additionnal: Option<bool>) -> u16 {
        let additionnal = is_additionnal.unwrap_or(true);
        let address = self.get_zero_page_address();
        let address = self.memory.borrow_mut().read_rom_16_no_crossing_page(address);
        let target_address = address + self.y_register as u16;
        if additionnal && address & 0xff00 != target_address & 0xff00 {
            self.additionnal_cycles += 1;
        }
        target_address
    }

    /// Get val from MEMORY. Indirect address is given as opcode 2-byte argument and Y register
    fn get_indirect_y_value(&mut self, is_additionnal: Option<bool>) -> u8 {
        let additionnal = is_additionnal.unwrap_or(true);
        let address = self.get_indirect_y_address(Some(additionnal));
        self.memory.borrow_mut().read_rom(address)
    }

    /// Write val into MEMORY. Indirect address is given as opcode 2-byte argument and Y register
    fn set_indirect_y(&mut self, value: u8, is_additionnal: Option<bool>) {
        let additionnal = is_additionnal.unwrap_or(true);
        let address = self.get_indirect_y_address(Some(additionnal));
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
        let adc:u16 = (value as u16) + (self.accumulator as u16) + (self.carry as u16);
        self.carry = ((adc >> 8) & 1) != 0;
        let result:u8 = (0xff & adc) as u8;

        self.overflow = (!! ((self.accumulator ^ result) & (value ^ result) & 0x80)) != 0;
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
        let absolute = self.get_absolute_x_value(Some(true));
        self.adc(absolute);
        (3, 4)
    }

    /// Function call for ADC $xxxx, X. Absolute, X
    fn fn_0x7d_with_no_additionnal_cycles(&mut self) -> (u16, u32) {
        let absolute = self.get_absolute_x_value(Some(false));
        self.adc(absolute);
        (3, 4)
    }

    /// Function call for ADC $xxxx, Y. Absolute, Y
    fn fn_0x79(&mut self) -> (u16, u32) {
        let absolute = self.get_absolute_y_value(Some(true));
        self.adc(absolute);
        (3, 4)
    }

    /// Function call for ADC $xxxx, Y. Absolute, Y
    fn fn_0x79_with_no_additionnal_cycles(&mut self) -> (u16, u32) {
        let absolute = self.get_absolute_y_value(Some(false));
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
        let indirect = self.get_indirect_y_value(Some(true));
        self.adc(indirect);
        (2, 5)
    }

    /// Function call for ADC ($xx), Y. Indirect, Y
    fn fn_0x71_with_no_additionnal_cycles(&mut self) -> (u16, u32) {
        let indirect = self.get_indirect_y_value(Some(false));
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
        self.accumulator &= self.get_absolute_x_value(Some(true));
        self.set_flags_nz(self.accumulator);
        (3, 4)
    }

    /// Function call for AND $xxxx, X. Absolute, X
    fn fn_0x3d_with_no_additionnal_cycles(&mut self) -> (u16, u32) {
        self.accumulator &= self.get_absolute_x_value(Some(false));
        self.set_flags_nz(self.accumulator);
        (3, 4)
    }

    /// Function call for AND $xxxx, Y. Absolute, Y
    fn fn_0x39(&mut self) -> (u16, u32) {
        
        self.accumulator &= self.get_absolute_y_value(Some(true));
        self.set_flags_nz(self.accumulator);
        (3, 4)
    }

    /// Function call for AND $xxxx, Y. Absolute, Y
    fn fn_0x39_with_no_additionnal_cycles(&mut self) -> (u16, u32) {
        self.accumulator &= self.get_absolute_y_value(Some(false));
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
        self.accumulator &= self.get_indirect_y_value(Some(true));
        self.set_flags_nz(self.accumulator);
        (2, 5)
    }

    /// Function call for AND ($xx), Y. Indirect, Y
    fn fn_0x31_with_no_additionnal_cycles(&mut self) -> (u16, u32) {
        self.accumulator &= self.get_indirect_y_value(Some(false));
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
        let value = self.get_absolute_x_value(Some(true));
        self.carry = (value >> 7) != 0;
        let value = (value << 1) & 0b11111111;
        self.set_absolute_x(value, Some(true));
        self.set_flags_nz(value);
        (3, 7)
    }

    /// Function call for ASL $xxxx, X. Absolute, X/// 
    fn fn_0x1e_with_no_additionnal_cycles(&mut self) -> (u16, u32) {
        let value = self.get_absolute_x_value(Some(true));
        self.carry = (value >> 7) != 0;
        let value = (value << 1) & 0b11111111;
        self.set_absolute_x(value, Some(true));
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
        if ! self.negative {
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
        if ! self.overflow {
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
            if self.program_counter & 0xff00 != old_pc & 0xff00{
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
            if (self.program_counter + 2) & 0xff00 != old_pc & 0xff00 { // PC+2 to take into account current instruction size
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
}
