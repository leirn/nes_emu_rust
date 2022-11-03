//! CPU component

// CPU implementation exemple :https://github.com/takahirox/riscv-rust/blob/master/src/cpu.rs

use crate::bus::memory::Bus;
use crate::cartridge::Cartridge;
use std::cell::RefCell;
use std::rc::Rc;

pub mod instructions;
pub mod opcodes;

use instructions::INSTRUCTION_TABLE;

pub struct Status {
    pub program_counter: u16,
    pub stack_pointer: u8,
    pub accumulator: u8,
    pub x_register: u8,
    pub y_register: u8,
    pub status_register: u8,
    pub total_cycles: u32,
}

pub struct Cpu {
    // Access to BUS
    pub bus: Bus,

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

    // Other states
    total_cycles: u32,
    remaining_cycles: u32,
    additionnal_cycles: u32,
    compteur: u32,
}

impl Cpu {
    pub fn new(_sdl_context: Rc<RefCell<sdl2::Sdl>>, _cartridge: Rc<RefCell<Cartridge>>) -> Cpu {
        Cpu {
            bus: Bus::new(_sdl_context, _cartridge),
            accumulator: 0,
            x_register: 0,
            y_register: 0,
            program_counter: 0,
            stack_pointer: 0,
            negative: false,
            overflow: false,
            break_flag: false,
            decimal: false,
            interrupt: true,
            zero: false,
            carry: false,
            total_cycles: 0,
            remaining_cycles: 0,
            additionnal_cycles: 0,
            compteur: 0,
        }
    }

    /// CPU initialisation function
    pub fn start(&mut self, entry_point: Option<u16>) {
        //!Execute 6502 Start sequence

        // Default is equivalent to JMP ($FFFC)
        if entry_point == None {
            self.program_counter = entry_point.unwrap_or_else(|| self.bus.read_rom_16(0xfffc));
        } else {
            self.program_counter = entry_point.unwrap();
        }

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
            return;
        }

        let opcode = self.bus.read_rom(self.program_counter) as usize;
        let cpu_instruction = &INSTRUCTION_TABLE[opcode];

        let instruction_result = (cpu_instruction.operation)(self);

        //let (step, remaining_cycles) = cpu_instruction(self);

        self.remaining_cycles = instruction_result.remaining_cycles + self.additionnal_cycles;
        self.total_cycles += self.remaining_cycles;
        self.remaining_cycles -= 1; // Do not count current cycle twice
        self.additionnal_cycles = 0;
        self.program_counter += instruction_result.step;
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
        self.push((self.program_counter >> 8) as u8);
        self.push((self.program_counter & 255) as u8);
        self.push(self.get_status_register() & 0b11101111); // NMI and IRQ set break flag to 0

        self.interrupt = false;

        self.program_counter = self.bus.read_rom_16(address);
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
        self.break_flag = false;
        self.overflow = ((status_register >> 6) & 1) != 0;
        self.negative = ((status_register >> 7) & 1) != 0;
    }

    /// Push value into stack
    fn push(&mut self, value: u8) {
        self.bus
            .write_rom(0x0100 | (self.stack_pointer as u16), value);
        self.stack_pointer -= 1; // Will eventually overflow on purpose
    }

    /// Pop/Pull value from stack
    fn pull(&mut self) -> u8 {
        self.stack_pointer += 1; // Will eventually overflow on purpose
        self.bus.read_rom(0x0100 | (self.stack_pointer as u16))
    }

    /// Get 8 bit immediate value on PC + 1
    fn get_immediate(&mut self) -> u8 {
        self.bus.read_rom(self.program_counter + 1)
    }

    /// Write val into Zero Page memory. Address is given as opcode 1-byte argument
    fn set_zero_page(&mut self, value: u8) {
        let address = self.get_zero_page_address();
        self.bus.write_rom(address, value);
    }

    /// Get ZeroPage address to be used for current opcode. Alias to get_immediate
    fn get_zero_page_address(&mut self) -> u16 {
        self.get_immediate() as u16
    }

    /// Get val from Zero Page MEMORY. Address is given as opcode 1-byte argument
    fn get_zero_page_value(&mut self) -> u8 {
        let address = self.get_immediate() as u16;
        self.bus.read_rom(address)
    }

    /// Write val into Zero Page MEMORY. Address is given as opcode 1-byte argument and X register
    fn set_zero_page_x(&mut self, value: u8) {
        let address = self.get_zero_page_x_address();
        self.bus.write_rom(address, value);
    }

    /// Get ZeroPage address to be used for current opcode and X register
    fn get_zero_page_x_address(&mut self) -> u16 {
        (self.bus.read_rom(self.program_counter + 1) + self.x_register) as u16
    }

    /// Get value at ZeroPage address to be used for current opcode and X register
    fn get_zero_page_x_value(&mut self) -> u8 {
        let address = self.get_zero_page_x_address();
        self.bus.read_rom(address)
    }

    /// Write val into Zero Page MEMORY. Address is given as opcode 1-byte argument and Y register
    fn set_zero_page_y(&mut self, value: u8) {
        let address = self.get_zero_page_y_address();
        self.bus.write_rom(address, value);
    }

    /// Get ZeroPage address to be used for current opcode and Y register
    fn get_zero_page_y_address(&mut self) -> u16 {
        (self.bus.read_rom(self.program_counter + 1) + self.y_register) as u16
    }

    /// Get value at ZeroPage address to be used for current opcode and Y register
    fn get_zero_page_y_value(&mut self) -> u8 {
        let address = self.get_zero_page_y_address();
        self.bus.read_rom(address)
    }

    /// Write val into MEMORY. Address is given as opcode 2-byte argument
    fn set_absolute(&mut self, value: u8) {
        let address = self.get_absolute_address();
        self.bus.write_rom(address, value);
    }

    /// Get address given as opcode 2-byte argument
    fn get_absolute_address(&mut self) -> u16 {
        self.bus.read_rom_16(self.program_counter + 1)
    }

    /// Get val from MEMORY. Address is given as opcode 2-byte argument
    fn get_absolute_value(&mut self) -> u8 {
        let address = self.get_absolute_address();
        self.bus.read_rom(address)
    }

    /// Write val into MEMORY. Address is given as opcode 2-byte argument and X register
    /// additionnal is boolean to fnine if this instruction will require extra cycles on page crossing
    fn set_absolute_x(&mut self, value: u8, is_additionnal: bool) {
        let address = self.get_absolute_x_address(is_additionnal);
        self.bus.write_rom(address, value);
    }

    /// Get address given as opcode 2-byte argument and X register
    fn get_absolute_x_address(&mut self, is_additionnal: bool) -> u16 {
        let address = self.bus.read_rom_16(self.program_counter + 1);
        let target_address = address + self.x_register as u16;
        if is_additionnal && address & 0xFF00 != target_address & 0xff00 {
            self.additionnal_cycles += 1;
        }
        target_address
    }

    /// Get val from MEMORY. Address is given as opcode 2-byte argument and X register
    fn get_absolute_x_value(&mut self, is_additionnal: bool) -> u8 {
        let address = self.get_absolute_x_address(is_additionnal);
        self.bus.read_rom(address)
    }

    /// Write val into MEMORY. Address is given as opcode 2-byte argument and Y register
    fn set_absolute_y(&mut self, value: u8, is_additionnal: bool) {
        let address = self.get_absolute_y_address(is_additionnal);
        self.bus.write_rom(address, value);
    }

    /// Get address given as opcode 2-byte argument and Y register
    fn get_absolute_y_address(&mut self, is_additionnal: bool) -> u16 {
        let address = self.bus.read_rom_16(self.program_counter + 1);
        let target_address = address + self.y_register as u16;
        if is_additionnal && address & 0xff00 != target_address & 0xff00 {
            self.additionnal_cycles += 1;
        }
        target_address
    }

    /// Get val from MEMORY. Address is given as opcode 2-byte argument and Y register
    fn get_absolute_y_value(&mut self, is_additionnal: bool) -> u8 {
        let address = self.get_absolute_y_address(is_additionnal);
        self.bus.read_rom(address)
    }

    /// Get indirect address given as opcode 2-byte argument and X register
    fn get_indirect_x_address(&mut self) -> u16 {
        let address = self.get_zero_page_x_address();
        self.bus.read_rom_16_no_crossing_page(address)
    }

    /// Get val from MEMORY. Indirect address is given as opcode 2-byte argument and X register
    fn get_indirect_x_value(&mut self) -> u8 {
        let address = self.get_indirect_x_address();
        self.bus.read_rom(address)
    }

    /// Write val into MEMORY. Indirect address is given as opcode 2-byte argument and X register///
    fn set_indirect_x(&mut self, value: u8) {
        let address = self.get_indirect_x_address();
        self.bus.write_rom(address, value);
    }

    /// Get indirect address given as opcode 2-byte argument and Y register
    fn get_indirect_y_address(&mut self, is_additionnal: bool) -> u16 {
        let address = self.get_zero_page_address();
        let address = self.bus.read_rom_16_no_crossing_page(address);
        let target_address = address + self.y_register as u16;
        if is_additionnal && address & 0xff00 != target_address & 0xff00 {
            self.additionnal_cycles += 1;
        }
        target_address
    }

    /// Get val from MEMORY. Indirect address is given as opcode 2-byte argument and Y register
    fn get_indirect_y_value(&mut self, is_additionnal: bool) -> u8 {
        let address = self.get_indirect_y_address(is_additionnal);
        self.bus.read_rom(address)
    }

    /// Write val into MEMORY. Indirect address is given as opcode 2-byte argument and Y register
    fn set_indirect_y(&mut self, value: u8, is_additionnal: bool) {
        let address = self.get_indirect_y_address(is_additionnal);
        self.bus.write_rom(address, value);
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

    /// General implementation for sbc operation
    /// SBC is the same as ADC with two's complement on second operand
    fn sbc(&mut self, value: u8) {
        self.adc(255 - value);
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
        } else if op2 - op1 >= 0x80 {
            self.carry = false;
            self.negative = false;
            self.zero = false;
        } else {
            self.carry = false;
            self.negative = true;
            self.zero = false;
        }
    }

    /// Function call for ADC $xxxx, X. Absolute, X
    fn fn_0x7d_with_no_additionnal_cycles(&mut self) -> (u16, u32) {
        let absolute = self.get_absolute_x_value(false);
        self.adc(absolute);
        (3, 4)
    }

    /// Function call for ADC $xxxx, Y. Absolute, Y
    fn fn_0x79_with_no_additionnal_cycles(&mut self) -> (u16, u32) {
        let absolute = self.get_absolute_y_value(false);
        self.adc(absolute);
        (3, 4)
    }

    /// Function call for ADC ($xx), Y. Indirect, Y
    fn fn_0x71_with_no_additionnal_cycles(&mut self) -> (u16, u32) {
        let indirect = self.get_indirect_y_value(false);
        self.adc(indirect);
        (2, 5)
    }

    /// Function call for AND $xxxx, X. Absolute, X
    fn fn_0x3d_with_no_additionnal_cycles(&mut self) -> (u16, u32) {
        self.accumulator &= self.get_absolute_x_value(false);
        self.set_flags_nz(self.accumulator);
        (3, 4)
    }

    /// Function call for AND $xxxx, Y. Absolute, Y
    fn fn_0x39_with_no_additionnal_cycles(&mut self) -> (u16, u32) {
        self.accumulator &= self.get_absolute_y_value(false);
        self.set_flags_nz(self.accumulator);
        (3, 4)
    }

    /// Function call for AND ($xx), Y. Indirect, Y
    fn fn_0x31_with_no_additionnal_cycles(&mut self) -> (u16, u32) {
        self.accumulator &= self.get_indirect_y_value(false);
        self.set_flags_nz(self.accumulator);
        (2, 5)
    }

    /// Function call for ASL $xxxx, X. Absolute, X///
    fn fn_0x1e_with_no_additionnal_cycles(&mut self) -> (u16, u32) {
        let mut value = self.get_absolute_x_value(false);
        self.carry = (value >> 7) != 0;
        value <<= 1;
        self.set_absolute_x(value, false);
        self.set_flags_nz(value);
        (3, 7)
    }

    /// Function call for EOR $xxxx, X. Absolute, X
    fn fn_0x5d_with_no_additionnal_cycles(&mut self) -> (u16, u32) {
        self.accumulator ^= self.get_absolute_x_value(false);
        self.set_flags_nz(self.accumulator);
        (3, 4)
    }

    /// Function call for EOR $xxxx, Y. Absolute, Y
    fn fn_0x59_with_no_additionnal_cycles(&mut self) -> (u16, u32) {
        self.accumulator ^= self.get_absolute_y_value(false);
        self.set_flags_nz(self.accumulator);
        (3, 4)
    }

    /// Function call for EOR ($xx), Y. Indirect, Y
    fn fn_0x51_with_no_additionnal_cycles(&mut self) -> (u16, u32) {
        self.accumulator ^= self.get_indirect_y_value(false);
        self.set_flags_nz(self.accumulator);
        (2, 5)
    }

    /// Function call for LSR $xxxx, X. Absolute, X
    fn fn_0x5e_with_no_additionnal_cycles(&mut self) -> (u16, u32) {
        let mut value = self.get_absolute_x_value(false);
        self.carry = (value & 1) == 1;
        value >>= 1;
        self.set_absolute_x(value, false);
        self.set_flags_nz(value);
        (3, 7)
    }

    /// Function call for ORA $xxxx, X. Absolute, X
    fn fn_0x1d_with_no_additionnal_cycles(&mut self) -> (u16, u32) {
        self.accumulator |= self.get_absolute_x_value(false);
        self.set_flags_nz(self.accumulator);
        (3, 4)
    }

    /// Function call for ORA $xxxx, Y. Absolute, Y
    fn fn_0x19_with_no_additionnal_cycles(&mut self) -> (u16, u32) {
        self.accumulator |= self.get_absolute_y_value(false);
        self.set_flags_nz(self.accumulator);
        (3, 4)
    }

    /// Function call for ORA ($xx), Y. Indirect, Y
    fn fn_0x11_with_no_additionnal_cycles(&mut self) -> (u16, u32) {
        self.accumulator |= self.get_indirect_y_value(false);
        self.set_flags_nz(self.accumulator);
        (2, 5)
    }

    /// Function call for ROL $xxxx, X. Absolute, X
    fn fn_0x3e_with_no_additionnal_cycles(&mut self) -> (u16, u32) {
        let val = self.get_absolute_x_value(false);
        let carry = self.carry as u8;
        self.carry = (val >> 7) != 0;
        let val = (val << 1) | (carry);
        self.set_absolute_x(val, false);
        self.set_flags_nz(val);
        (3, 7)
    }

    /// Function call for ROR$xxxx, X. Absolute, X
    fn fn_0x7e_with_no_additionnal_cycles(&mut self) -> (u16, u32) {
        let val = self.get_absolute_x_value(false);
        let carry = val & 1;
        let val = (val >> 1) | ((self.carry as u8) << 7);
        self.carry = carry != 0;
        self.set_absolute_x(val, false);
        self.set_flags_nz(val);
        (3, 7)
    }

    /// Return a dictionnary containing the current CPU Status. Usefull for debugging
    pub fn get_status(&self) -> Status {
        Status {
            program_counter: self.program_counter,
            stack_pointer: self.stack_pointer,
            accumulator: self.accumulator,
            x_register: self.x_register,
            y_register: self.y_register,
            status_register: self.get_status_register(),
            total_cycles: self.total_cycles,
        }
    }

    pub fn get_remaining_cycles(&self) -> u32 {
        self.remaining_cycles
    }

    pub fn _get_total_cycles(&self) -> u32 {
        self.total_cycles
    }
}
