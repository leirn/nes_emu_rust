use crate::components::{MEMORY};
use std::collections::HashMap;

pub struct Cpu {
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
        
    }
    fn dummy(&mut self) -> (u16, u32) {
        (0, 0)
    }
    /// CPU initialisation function
    pub fn start(&mut self, entry_point: Option<u16>) {
        //!Execute 6502 Start sequence

        self.populate_instructions_vector();

        // Fnault is equivalent to JMP ($FFFC)
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
        MEMORY.lock().unwrap().write_rom(0x0100 | (self.stack_pointer as u16), value);
        self.stack_pointer = self.stack_pointer - 1; // Will eventually overflow on purpose
    }

    /// Pop/Pull value from stack
    fn pull(&mut self) -> u8 {
        self.stack_pointer = self.stack_pointer + 1; // Will eventually overflow on purpose
        MEMORY.lock().unwrap().read_rom(0x0100 | (self.stack_pointer as u16))
    }

    /// Get 8 bit immediate value on PC + 1
    fn get_immediate(&mut self) -> u8 {
        MEMORY.lock().unwrap().read_rom(self.program_counter+1)
    }

    /// Write val into Zero Page memory. Address is given as opcode 1-byte argument
    fn set_zero_page(&mut self, value: u8) {
        MEMORY.lock().unwrap().write_rom(self.get_zero_page_address(), value)
    }

    /// Get ZeroPage address to be used for current opcode. Alias to get_immediate
    fn get_zero_page_address(&mut self) -> u16 {
         self.get_immediate() as u16
    }

    /// Get val from Zero Page MEMORY. Address is given as opcode 1-byte argument
    fn get_zero_page_value(&mut self) -> u8 {
        let address = self.get_immediate() as u16;
        MEMORY.lock().unwrap().read_rom(address)
    }

    /// Write val into Zero Page MEMORY. Address is given as opcode 1-byte argument and X register
    fn set_zero_page_x(&mut self, value: u8) {
        MEMORY.lock().unwrap().write_rom(self.get_zero_page_x_address(), value);
    }

    /// Get ZeroPage address to be used for current opcode and X register
    fn get_zero_page_x_address(&mut self) -> u16 {
        ((MEMORY.lock().unwrap().read_rom(self.program_counter+1) + self.x_register) & 255) as u16
    }

    /// Get value at ZeroPage address to be used for current opcode and X register
    fn get_zero_page_x_value(&mut self) -> u8 {
        let address = self.get_zero_page_x_address();
        MEMORY.lock().unwrap().read_rom(address)
    }

    /// Write val into Zero Page MEMORY. Address is given as opcode 1-byte argument and Y register
    fn set_zero_page_y(&mut self, value: u8) {
        MEMORY.lock().unwrap().write_rom(self.get_zero_page_y_address(), value)
    }

    /// Get ZeroPage address to be used for current opcode and Y register
    fn get_zero_page_y_address(&mut self) -> u16 {
          ((MEMORY.lock().unwrap().read_rom(self.program_counter+1) + self.y_register) & 255) as u16
    }

    /// Get value at ZeroPage address to be used for current opcode and Y register
    fn get_zero_page_y_value(&mut self) -> u8 {
        let address = self.get_zero_page_y_address();
        MEMORY.lock().unwrap().read_rom(address)
    }

    /// Write val into MEMORY. Address is given as opcode 2-byte argument
    fn set_absolute(&mut self, value: u8) {
        MEMORY.lock().unwrap().write_rom(self.get_absolute_address(), value)
    }

    /// Get address given as opcode 2-byte argument
    fn get_absolute_address(&mut self) -> u16 {
         MEMORY.lock().unwrap().read_rom_16(self.program_counter+1)
    }

    /// Get val from MEMORY. Address is given as opcode 2-byte argument
    fn get_absolute_value(&mut self) -> u8 {
        let address = self.get_absolute_address();
        MEMORY.lock().unwrap().read_rom(address)
    }

    /// Write val into MEMORY. Address is given as opcode 2-byte argument and X register
    /// additionnal is boolean to fnine if this instruction will require extra cycles on page crossing
    fn set_absolute_x(&mut self, value: u8, is_additionnal: Option<bool>) {
        let additionnal = is_additionnal.unwrap_or(true);
        MEMORY.lock().unwrap().write_rom(self.get_absolute_x_address(Some(additionnal)), value);
    }

    /// Get address given as opcode 2-byte argument and X register
    fn get_absolute_x_address(&mut self, is_additionnal: Option<bool>) -> u16 {
        let additionnal = is_additionnal.unwrap_or(true);
        let address = MEMORY.lock().unwrap().read_rom_16(self.program_counter+1);
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
        MEMORY.lock().unwrap().read_rom(address)
    }

    /// Write val into MEMORY. Address is given as opcode 2-byte argument and Y register
    fn set_absolute_y(&mut self, value: u8, is_additionnal: Option<bool>) {
        let additionnal = is_additionnal.unwrap_or(true);
        MEMORY.lock().unwrap().write_rom(self.get_absolute_y_address(Some(additionnal)), value);
    }

    /// Get address given as opcode 2-byte argument and Y register
    fn get_absolute_y_address(&mut self, is_additionnal: Option<bool>)-> u16 {
        let additionnal = is_additionnal.unwrap_or(true);
        let address = MEMORY.lock().unwrap().read_rom_16(self.program_counter+1);
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
        MEMORY.lock().unwrap().read_rom(address)
    }

    /// Get indirect address given as opcode 2-byte argument and X register
    fn get_indirect_x_address(&mut self) -> u16 {
        let address = self.get_zero_page_x_address();
        MEMORY.lock().unwrap().read_rom_16_no_crossing_page(address)
    }

    /// Get val from MEMORY. Indirect address is given as opcode 2-byte argument and X register
    fn get_indirect_x_value(&mut self) -> u8 {
        let address = self.get_indirect_x_address();
        MEMORY.lock().unwrap().read_rom(address)
    }

    /// Write val into MEMORY. Indirect address is given as opcode 2-byte argument and X register/// 
    fn set_indirect_x(&mut self, value: u8) {
        MEMORY.lock().unwrap().write_rom(self.get_indirect_x_address(), value);
    }

    /// Get indirect address given as opcode 2-byte argument and Y register
    fn get_indirect_y_address(&mut self, is_additionnal: Option<bool>) -> u16 {
        let additionnal = is_additionnal.unwrap_or(true);
        let address = self.get_zero_page_address();
        let address = MEMORY.lock().unwrap().read_rom_16_no_crossing_page(address);
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
        MEMORY.lock().unwrap().read_rom(address)
    }

    /// Write val into MEMORY. Indirect address is given as opcode 2-byte argument and Y register
    fn set_indirect_y(&mut self, value: u8, is_additionnal: Option<bool>) {
        let additionnal = is_additionnal.unwrap_or(true);
        MEMORY.lock().unwrap().write_rom(self.get_indirect_y_address(Some(additionnal)), value);
    }

    /// Sets flags N and Z according to value
    fn set_flags_nz(&mut self, value: u8) {
        self.set_negative(value);
        self.set_zero(value);
    }

    ///  Set Negative Flag according to value
    fn set_negative(&mut self, value: u8) {
        if value < 0 {
            self.negative = false;
        }
        else {
            self.negative = (value >> 7) != 0;
        }
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

    /// Function call for AND ($xx), Y. Indirect, Y/// 
    fn fn_0x31_with_no_additionnal_cycles(&mut self) -> (u16, u32) {
        self.accumulator &= self.get_indirect_y_value(Some(false));
        self.set_flags_nz(self.accumulator);
        (2, 5)
    }
}
