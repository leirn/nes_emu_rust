use super::Cpu;
use std::fmt;

pub struct Instruction {
    pub opcode: u8,
    pub name: InstructionCode,
    pub mode: InstructionMode,
    pub operation: fn(cpu: &mut Cpu) -> InstructionResult,
}

impl Instruction {
    pub fn get_syntax(&self, cpu: &mut Cpu) -> String {
        let instruction_name = self.name.to_string();
        let argument = match self.mode {
            InstructionMode::Implied => String::new(),
            InstructionMode::Immediate => format!("#${:02x}", cpu.get_immediate()).to_uppercase(),
            InstructionMode::Accumulator => String::from("A"),
            InstructionMode::Absolute => String::from(format!(
                "${:04x} = {:02x}",
                cpu.get_absolute_address(),
                cpu.get_absolute_value()
            ))
            .to_uppercase(),
            InstructionMode::AbsoluteX => String::from(format!(
                "${:04x} = {:02x}",
                cpu.get_absolute_x_address(false),
                cpu.get_absolute_x_value(false)
            ))
            .to_uppercase(),
            InstructionMode::AbsoluteY => String::from(format!(
                "${:04x} = {:02x}",
                cpu.get_absolute_y_address(false),
                cpu.get_absolute_y_value(false)
            ))
            .to_uppercase(),
            InstructionMode::ZeroPage => String::from(format!(
                "${:02x} = {:02x}",
                cpu.get_immediate(),
                cpu.get_zero_page_value()
            ))
            .to_uppercase(),
            InstructionMode::ZeroPageX => String::from(format!(
                "${:04x} = {:02x}",
                cpu.get_zero_page_x_address(),
                cpu.get_zero_page_x_value()
            ))
            .to_uppercase(),
            InstructionMode::ZeroPageY => String::from(format!(
                "${:04x} = {:02x}",
                cpu.get_zero_page_y_address(),
                cpu.get_zero_page_y_value()
            ))
            .to_uppercase(),

            InstructionMode::IndirectX => String::from(format!(
                "(${:02x},X) @ 80 = {:04x} = {:02x}",
                cpu.get_immediate(),
                cpu.get_indirect_x_address(),
                cpu.get_indirect_x_value(),
            ))
            .to_uppercase(),
            _ => String::new(),
        };
        format!("{} {}", instruction_name, argument)
    }
}

pub struct InstructionResult {
    pub step: u16,
    pub remaining_cycles: u32,
}

#[derive(PartialEq)]
pub enum InstructionMode {
    Immediate,
    Implied,
    Accumulator,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Relative,
    Indirect,
    IndirectX,
    IndirectY,
    Undefined,
}

#[derive(PartialEq, Debug)]
pub enum InstructionCode {
    BRK,
    ADC,
    AND,
    ASL,
    BIT,
    BCC,
    BCS,
    BMI,
    BNE,
    BPL,
    BVC,
    BVS,
    CMP,
    CPX,
    CPY,
    DCP,
    EOR,
    ISC,
    JMP,
    JSR,
    SLO, // Equivalent to ASL + ORA
    NOP,
    DOP,
    TOP,
    RLA, // Equivalent to ROL + AND
    RRA, // Equivalent to ROR + AND
    ORA,
    ROL,
    ROR,
    SRE, // Equivalent to LSR + EOR
    RTI,
    RTS,
    SBC,
    LDA,
    LDX,
    LDY,
    LAX,
    SAX,
    STA,
    STX,
    STY,
    TSX,
    TXS,
    PHA,
    PLA,
    PHP,
    PLP,
    INC,
    DEC,
    INX,
    DEX,
    INY,
    DEY,
    TAX,
    TXA,
    TAY,
    TYA,
    CLC,
    SEC,
    CLI,
    SEI,
    CLV,
    CLD,
    SED,
    LSR,
    Unknown,
}

impl fmt::Display for InstructionCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub const INSTRUCTION_TABLE: [Instruction; 0x100] = [
    Instruction {
        opcode: 0x00,
        name: InstructionCode::BRK,
        mode: InstructionMode::Implied,
        operation: |cpu| {
            cpu.program_counter += 1;
            cpu.push((cpu.program_counter >> 8) as u8);
            cpu.push((cpu.program_counter & 0xff) as u8);
            cpu.push(cpu.get_status_register() | (1 << 4));
            cpu.program_counter = cpu.bus.read_rom_16(0xfffe);
            InstructionResult {
                step: 0,
                remaining_cycles: 7,
            }
        },
    },
    Instruction {
        opcode: 0x01,
        name: InstructionCode::ORA,
        mode: InstructionMode::IndirectX,
        operation: |cpu| {
            cpu.accumulator |= cpu.get_indirect_x_value();
            cpu.set_flags_nz(cpu.accumulator);
            InstructionResult {
                step: 2,
                remaining_cycles: 6,
            }
        },
    },
    Instruction {
        opcode: 0x02,
        name: InstructionCode::Unknown,
        mode: InstructionMode::Undefined,
        operation: |_cpu| panic!("Unknow instruction 0x02"),
    },
    Instruction {
        opcode: 0x03,
        name: InstructionCode::SLO,
        mode: InstructionMode::IndirectX,
        operation: |cpu| {
            let mut value = cpu.get_indirect_x_value();
            cpu.carry = (value >> 7) != 0;
            value <<= 1;
            cpu.set_indirect_x(value);
            (INSTRUCTION_TABLE[0x01].operation)(cpu); // ORA
            InstructionResult {
                step: 2,
                remaining_cycles: 8,
            }
        },
    },
    Instruction {
        opcode: 0x04,
        name: InstructionCode::DOP,
        mode: InstructionMode::Implied,
        operation: |_cpu| InstructionResult {
            step: 2,
            remaining_cycles: 3,
        },
    },
    Instruction {
        opcode: 0x05,
        name: InstructionCode::ORA,
        mode: InstructionMode::ZeroPage,
        operation: |cpu| {
            cpu.accumulator |= cpu.get_zero_page_value();
            cpu.set_flags_nz(cpu.accumulator);
            InstructionResult {
                step: 2,
                remaining_cycles: 3,
            }
        },
    },
    Instruction {
        opcode: 0x06,
        name: InstructionCode::ASL,
        mode: InstructionMode::ZeroPage,
        operation: |cpu| {
            let mut value = cpu.get_zero_page_value();
            cpu.carry = (value >> 7) != 0;
            value <<= 1;
            cpu.set_zero_page(value);
            cpu.set_flags_nz(value);
            InstructionResult {
                step: 2,
                remaining_cycles: 5,
            }
        },
    },
    Instruction {
        opcode: 0x07,
        name: InstructionCode::SLO,
        mode: InstructionMode::ZeroPage,
        operation: |cpu| {
            (INSTRUCTION_TABLE[0x06].operation)(cpu);
            (INSTRUCTION_TABLE[0x05].operation)(cpu);
            InstructionResult {
                step: 2,
                remaining_cycles: 5,
            }
        },
    },
    Instruction {
        opcode: 0x08,
        name: InstructionCode::PHP,
        mode: InstructionMode::Implied,
        operation: |cpu| {
            // create status byte
            let status_register = cpu.get_status_register() | (1 << 4);
            cpu.push(status_register);
            InstructionResult {
                step: 1,
                remaining_cycles: 3,
            }
        },
    },
    Instruction {
        opcode: 0x09,
        name: InstructionCode::ORA,
        mode: InstructionMode::Immediate,
        operation: |cpu| {
            cpu.accumulator |= cpu.get_immediate();
            cpu.set_flags_nz(cpu.accumulator);
            InstructionResult {
                step: 2,
                remaining_cycles: 2,
            }
        },
    },
    Instruction {
        opcode: 0x0a,
        name: InstructionCode::ASL,
        mode: InstructionMode::Accumulator,
        operation: |cpu| {
            cpu.carry = (cpu.accumulator >> 7) != 0;
            cpu.accumulator <<= 1;
            cpu.set_flags_nz(cpu.accumulator);
            InstructionResult {
                step: 1,
                remaining_cycles: 2,
            }
        },
    },
    Instruction {
        opcode: 0x0b,
        name: InstructionCode::Unknown,
        mode: InstructionMode::Undefined,
        operation: |_cpu| panic!("Unknow instruction 0x0b"),
    },
    Instruction {
        opcode: 0x0c,
        name: InstructionCode::TOP,
        mode: InstructionMode::Implied,
        operation: |_cpu| InstructionResult {
            step: 3,
            remaining_cycles: 4,
        },
    },
    Instruction {
        opcode: 0x0d,
        name: InstructionCode::ORA,
        mode: InstructionMode::Absolute,
        operation: |cpu| {
            cpu.accumulator |= cpu.get_absolute_value();
            cpu.set_flags_nz(cpu.accumulator);
            InstructionResult {
                step: 3,
                remaining_cycles: 4,
            }
        },
    },
    Instruction {
        opcode: 0x0e,
        name: InstructionCode::ASL,
        mode: InstructionMode::Absolute,
        operation: |cpu| {
            let mut value = cpu.get_absolute_value();
            cpu.carry = (value >> 7) != 0;
            value <<= 1;
            cpu.set_absolute(value);
            cpu.set_flags_nz(value);
            InstructionResult {
                step: 3,
                remaining_cycles: 6,
            }
        },
    },
    Instruction {
        opcode: 0x0f,
        name: InstructionCode::SLO,
        mode: InstructionMode::Absolute,
        operation: |cpu| {
            (INSTRUCTION_TABLE[0x0e].operation)(cpu);
            (INSTRUCTION_TABLE[0x0d].operation)(cpu);
            InstructionResult {
                step: 3,
                remaining_cycles: 6,
            }
        },
    },
    Instruction {
        opcode: 0x10,
        name: InstructionCode::BPL,
        mode: InstructionMode::Relative,
        operation: |cpu| {
            let old_pc = cpu.program_counter + 2;
            let signed: i8 = cpu.get_immediate() as i8;
            if !cpu.negative {
                cpu.program_counter = cpu.program_counter.wrapping_add(signed as u16);
                cpu.additionnal_cycles += 1;
                if cpu.program_counter & 0xff00 != old_pc & 0xff00 {
                    cpu.additionnal_cycles += 1;
                }
            }
            InstructionResult {
                step: 2,
                remaining_cycles: 2,
            }
        },
    },
    Instruction {
        opcode: 0x11,
        name: InstructionCode::ORA,
        mode: InstructionMode::IndirectY,
        operation: |cpu| {
            cpu.accumulator |= cpu.get_indirect_y_value(true);
            cpu.set_flags_nz(cpu.accumulator);
            InstructionResult {
                step: 2,
                remaining_cycles: 5,
            }
        },
    },
    Instruction {
        opcode: 0x12,
        name: InstructionCode::Unknown,
        mode: InstructionMode::Undefined,
        operation: |_cpu| panic!("Unknow instruction 0x12"),
    },
    Instruction {
        opcode: 0x13,
        name: InstructionCode::SLO,
        mode: InstructionMode::IndirectY,
        operation: |cpu| {
            let mut value = cpu.get_indirect_y_value(false);
            cpu.carry = (value >> 7) != 0;
            value <<= 1;
            cpu.set_indirect_y(value, false);
            cpu.fn_0x11_with_no_additionnal_cycles(); // ORA
            InstructionResult {
                step: 2,
                remaining_cycles: 8,
            }
        },
    },
    Instruction {
        opcode: 0x14,
        name: InstructionCode::DOP,
        mode: InstructionMode::Implied,
        operation: |_cpu| InstructionResult {
            step: 2,
            remaining_cycles: 4,
        },
    },
    Instruction {
        opcode: 0x15,
        name: InstructionCode::ORA,
        mode: InstructionMode::ZeroPageX,
        operation: |cpu| {
            cpu.accumulator |= cpu.get_zero_page_x_value();
            cpu.set_flags_nz(cpu.accumulator);
            InstructionResult {
                step: 2,
                remaining_cycles: 4,
            }
        },
    },
    Instruction {
        opcode: 0x16,
        name: InstructionCode::ASL,
        mode: InstructionMode::ZeroPageX,
        operation: |cpu| {
            let mut value = cpu.get_zero_page_x_value();
            cpu.carry = (value >> 7) != 0;
            value <<= 1;
            cpu.set_zero_page_x(value);
            cpu.set_flags_nz(value);
            InstructionResult {
                step: 2,
                remaining_cycles: 6,
            }
        },
    },
    Instruction {
        opcode: 0x17,
        name: InstructionCode::SLO,
        mode: InstructionMode::ZeroPageX,
        operation: |cpu| {
            (INSTRUCTION_TABLE[0x16].operation)(cpu);
            (INSTRUCTION_TABLE[0x15].operation)(cpu);
            InstructionResult {
                step: 2,
                remaining_cycles: 6,
            }
        },
    },
    Instruction {
        opcode: 0x18,
        name: InstructionCode::CLC,
        mode: InstructionMode::Implied,
        operation: |cpu| {
            cpu.carry = false;
            InstructionResult {
                step: 1,
                remaining_cycles: 2,
            }
        },
    },
    Instruction {
        opcode: 0x19,
        name: InstructionCode::ORA,
        mode: InstructionMode::AbsoluteY,
        operation: |cpu| {
            cpu.accumulator |= cpu.get_absolute_y_value(true);
            cpu.set_flags_nz(cpu.accumulator);
            InstructionResult {
                step: 3,
                remaining_cycles: 4,
            }
        },
    },
    Instruction {
        opcode: 0x1a,
        name: InstructionCode::NOP,
        mode: InstructionMode::Implied,
        operation: |_cpu| InstructionResult {
            step: 1,
            remaining_cycles: 2,
        },
    },
    Instruction {
        opcode: 0x1b,
        name: InstructionCode::SLO,
        mode: InstructionMode::AbsoluteY,
        operation: |cpu| {
            let mut value = cpu.get_absolute_y_value(false);
            cpu.carry = (value >> 7) != 0;
            value <<= 1;
            cpu.set_absolute_y(value, false);
            cpu.fn_0x19_with_no_additionnal_cycles(); // ORA
            InstructionResult {
                step: 3,
                remaining_cycles: 7,
            }
        },
    },
    Instruction {
        opcode: 0x1c,
        name: InstructionCode::TOP,
        mode: InstructionMode::Implied,
        operation: |cpu| {
            cpu.get_absolute_x_value(true); // Need extra cycle
            InstructionResult {
                step: 3,
                remaining_cycles: 4,
            }
        },
    },
    Instruction {
        opcode: 0x1d,
        name: InstructionCode::ORA,
        mode: InstructionMode::AbsoluteX,
        operation: |cpu| {
            cpu.accumulator |= cpu.get_absolute_x_value(true);
            cpu.set_flags_nz(cpu.accumulator);
            InstructionResult {
                step: 3,
                remaining_cycles: 4,
            }
        },
    },
    Instruction {
        opcode: 0x1e,
        name: InstructionCode::ASL,
        mode: InstructionMode::AbsoluteX,
        operation: |cpu| {
            let mut value = cpu.get_absolute_x_value(true);
            cpu.carry = (value >> 7) != 0;
            value <<= 1;
            cpu.set_absolute_x(value, true);
            cpu.set_flags_nz(value);
            InstructionResult {
                step: 3,
                remaining_cycles: 7,
            }
        },
    },
    Instruction {
        opcode: 0x1f,
        name: InstructionCode::SLO,
        mode: InstructionMode::AbsoluteX,
        operation: |cpu| {
            cpu.fn_0x1e_with_no_additionnal_cycles(); // ASL
            cpu.fn_0x1d_with_no_additionnal_cycles(); // ORA
            InstructionResult {
                step: 3,
                remaining_cycles: 7,
            }
        },
    },
    Instruction {
        opcode: 0x20,
        name: InstructionCode::JSR,
        mode: InstructionMode::Absolute,
        operation: |cpu| {
            let program_counter = cpu.program_counter + 2;
            let high = (program_counter >> 8) as u8;
            let low = (program_counter & 255) as u8;
            cpu.push(high); // little endian
            cpu.push(low);
            cpu.program_counter = cpu.get_absolute_address();
            InstructionResult {
                step: 0,
                remaining_cycles: 6,
            }
        },
    },
    Instruction {
        opcode: 0x21,
        name: InstructionCode::AND,
        mode: InstructionMode::IndirectX,
        operation: |cpu| {
            cpu.accumulator &= cpu.get_indirect_x_value();
            cpu.set_flags_nz(cpu.accumulator);
            InstructionResult {
                step: 2,
                remaining_cycles: 6,
            }
        },
    },
    Instruction {
        opcode: 0x22,
        name: InstructionCode::Unknown,
        mode: InstructionMode::Undefined,
        operation: |_cpu| panic!("Unknow instruction 0x22"),
    },
    Instruction {
        opcode: 0x23,
        name: InstructionCode::RLA,
        mode: InstructionMode::IndirectX,
        operation: |cpu| {
            let val = cpu.get_indirect_x_value();
            let carry = cpu.carry as u8;
            cpu.carry = (val >> 7) != 0;
            let val = (val << 1) | (carry);
            cpu.set_indirect_x(val);
            (INSTRUCTION_TABLE[0x21].operation)(cpu);
            InstructionResult {
                step: 2,
                remaining_cycles: 8,
            }
        },
    },
    Instruction {
        opcode: 0x24,
        name: InstructionCode::BIT,
        mode: InstructionMode::ZeroPage,
        operation: |cpu| {
            let tocomp = cpu.get_zero_page_value();
            let value = tocomp & cpu.accumulator;
            cpu.set_zero(value);
            cpu.set_negative(tocomp);
            cpu.overflow = ((tocomp >> 6) & 1) != 0;
            InstructionResult {
                step: 2,
                remaining_cycles: 3,
            }
        },
    },
    Instruction {
        opcode: 0x25,
        name: InstructionCode::AND,
        mode: InstructionMode::ZeroPage,
        operation: |cpu| {
            cpu.accumulator &= cpu.get_zero_page_value();
            cpu.set_flags_nz(cpu.accumulator);
            InstructionResult {
                step: 2,
                remaining_cycles: 3,
            }
        },
    },
    Instruction {
        opcode: 0x26,
        name: InstructionCode::ROL,
        mode: InstructionMode::ZeroPage,
        operation: |cpu| {
            let val = cpu.get_zero_page_value();
            let carry = cpu.carry as u8;
            cpu.carry = (val >> 7) != 0;
            let val = (val << 1) | (carry);
            cpu.set_zero_page(val);
            cpu.set_flags_nz(val);
            InstructionResult {
                step: 2,
                remaining_cycles: 5,
            }
        },
    },
    Instruction {
        opcode: 0x27,
        name: InstructionCode::RLA,
        mode: InstructionMode::ZeroPage,
        operation: |cpu| {
            (INSTRUCTION_TABLE[0x26].operation)(cpu);
            (INSTRUCTION_TABLE[0x25].operation)(cpu);
            InstructionResult {
                step: 2,
                remaining_cycles: 5,
            }
        },
    },
    Instruction {
        opcode: 0x28,
        name: InstructionCode::PLP,
        mode: InstructionMode::Implied,
        operation: |cpu| {
            let status_register = cpu.pull();
            cpu.set_status_register(status_register);
            InstructionResult {
                step: 1,
                remaining_cycles: 4,
            }
        },
    },
    Instruction {
        opcode: 0x29,
        name: InstructionCode::AND,
        mode: InstructionMode::Immediate,
        operation: |cpu| {
            cpu.accumulator &= cpu.get_immediate();
            cpu.set_flags_nz(cpu.accumulator);
            InstructionResult {
                step: 2,
                remaining_cycles: 2,
            }
        },
    },
    Instruction {
        opcode: 0x2a,
        name: InstructionCode::ROL,
        mode: InstructionMode::Accumulator,
        operation: |cpu| {
            let carry = cpu.carry as u8;
            cpu.carry = (cpu.accumulator >> 7) != 0;
            cpu.accumulator = (cpu.accumulator << 1) | (carry);
            cpu.set_flags_nz(cpu.accumulator);
            InstructionResult {
                step: 1,
                remaining_cycles: 2,
            }
        },
    },
    Instruction {
        opcode: 0x2b,
        name: InstructionCode::Unknown,
        mode: InstructionMode::Undefined,
        operation: |_cpu| panic!("Unknow instruction 0x2b"),
    },
    Instruction {
        opcode: 0x2c,
        name: InstructionCode::BIT,
        mode: InstructionMode::Absolute,
        operation: |cpu| {
            let tocomp = cpu.get_absolute_value();
            let value = tocomp & cpu.accumulator;
            cpu.set_zero(value);
            cpu.set_negative(tocomp);
            cpu.overflow = ((tocomp >> 6) & 1) != 0;
            InstructionResult {
                step: 3,
                remaining_cycles: 4,
            }
        },
    },
    Instruction {
        opcode: 0x2d,
        name: InstructionCode::AND,
        mode: InstructionMode::Absolute,
        operation: |cpu| {
            cpu.accumulator &= cpu.get_absolute_value();
            cpu.set_flags_nz(cpu.accumulator);
            InstructionResult {
                step: 3,
                remaining_cycles: 4,
            }
        },
    },
    Instruction {
        opcode: 0x2e,
        name: InstructionCode::ROL,
        mode: InstructionMode::Absolute,
        operation: |cpu| {
            let val = cpu.get_absolute_value();
            let carry = cpu.carry as u8;
            cpu.carry = (val >> 7) != 0;
            let val = (val << 1) | (carry);
            cpu.set_absolute(val);
            cpu.set_flags_nz(val);
            InstructionResult {
                step: 3,
                remaining_cycles: 6,
            }
        },
    },
    Instruction {
        opcode: 0x2f,
        name: InstructionCode::RLA,
        mode: InstructionMode::Absolute,
        operation: |cpu| {
            (INSTRUCTION_TABLE[0x2e].operation)(cpu);
            (INSTRUCTION_TABLE[0x2d].operation)(cpu);
            InstructionResult {
                step: 3,
                remaining_cycles: 6,
            }
        },
    },
    Instruction {
        opcode: 0x30,
        name: InstructionCode::BMI,
        mode: InstructionMode::Relative,
        operation: |cpu| {
            let old_pc = cpu.program_counter + 2;
            let signed: i8 = cpu.get_immediate() as i8;
            if cpu.negative {
                cpu.program_counter = cpu.program_counter.wrapping_add(signed as u16);
                cpu.additionnal_cycles += 1;
                if cpu.program_counter & 0xff00 != old_pc & 0xff00 {
                    cpu.additionnal_cycles += 1
                }
            }
            InstructionResult {
                step: 2,
                remaining_cycles: 2,
            }
        },
    },
    Instruction {
        opcode: 0x31,
        name: InstructionCode::AND,
        mode: InstructionMode::IndirectY,
        operation: |cpu| {
            cpu.accumulator &= cpu.get_indirect_y_value(true);
            cpu.set_flags_nz(cpu.accumulator);
            InstructionResult {
                step: 2,
                remaining_cycles: 5,
            }
        },
    },
    Instruction {
        opcode: 0x32,
        name: InstructionCode::Unknown,
        mode: InstructionMode::Undefined,
        operation: |_cpu| panic!("Unknow instruction 0x32"),
    },
    Instruction {
        opcode: 0x33,
        name: InstructionCode::RLA,
        mode: InstructionMode::IndirectY,
        operation: |cpu| {
            let val = cpu.get_indirect_y_value(false);
            let carry = cpu.carry as u8;
            cpu.carry = (val >> 7) != 0;
            let val = (val << 1) | (carry);
            cpu.set_indirect_y(val, false);
            cpu.fn_0x31_with_no_additionnal_cycles(); // AND
            InstructionResult {
                step: 2,
                remaining_cycles: 8,
            }
        },
    },
    Instruction {
        opcode: 0x34,
        name: InstructionCode::DOP,
        mode: InstructionMode::Implied,
        operation: |_cpu| InstructionResult {
            step: 2,
            remaining_cycles: 4,
        },
    },
    Instruction {
        opcode: 0x35,
        name: InstructionCode::AND,
        mode: InstructionMode::ZeroPageX,
        operation: |cpu| {
            cpu.accumulator &= cpu.get_zero_page_x_value();
            cpu.set_flags_nz(cpu.accumulator);
            InstructionResult {
                step: 2,
                remaining_cycles: 4,
            }
        },
    },
    Instruction {
        opcode: 0x36,
        name: InstructionCode::ROL,
        mode: InstructionMode::ZeroPageX,
        operation: |cpu| {
            let val = cpu.get_zero_page_x_value();
            let carry = cpu.carry as u8;
            cpu.carry = (val >> 7) != 0;
            let val = (val << 1) | (carry);
            cpu.set_zero_page_x(val);
            cpu.set_flags_nz(val);
            InstructionResult {
                step: 2,
                remaining_cycles: 6,
            }
        },
    },
    Instruction {
        opcode: 0x37,
        name: InstructionCode::RLA,
        mode: InstructionMode::ZeroPageX,
        operation: |cpu| {
            (INSTRUCTION_TABLE[0x36].operation)(cpu);
            (INSTRUCTION_TABLE[0x35].operation)(cpu);
            InstructionResult {
                step: 2,
                remaining_cycles: 6,
            }
        },
    },
    Instruction {
        opcode: 0x38,
        name: InstructionCode::SEC,
        mode: InstructionMode::Implied,
        operation: |cpu| {
            cpu.carry = true;
            InstructionResult {
                step: 1,
                remaining_cycles: 2,
            }
        },
    },
    Instruction {
        opcode: 0x39,
        name: InstructionCode::AND,
        mode: InstructionMode::AbsoluteY,
        operation: |cpu| {
            cpu.accumulator &= cpu.get_absolute_y_value(true);
            cpu.set_flags_nz(cpu.accumulator);
            InstructionResult {
                step: 3,
                remaining_cycles: 4,
            }
        },
    },
    Instruction {
        opcode: 0x3a,
        name: InstructionCode::NOP,
        mode: InstructionMode::Implied,
        operation: |_cpu| InstructionResult {
            step: 1,
            remaining_cycles: 2,
        },
    },
    Instruction {
        opcode: 0x3b,
        name: InstructionCode::RLA,
        mode: InstructionMode::AbsoluteY,
        operation: |cpu| {
            let val = cpu.get_absolute_y_value(false);
            let carry = cpu.carry as u8;
            cpu.carry = (val >> 7) != 0;
            let val = (val << 1) | (carry);
            cpu.set_absolute_y(val, false);
            cpu.fn_0x39_with_no_additionnal_cycles(); // AND
            InstructionResult {
                step: 3,
                remaining_cycles: 7,
            }
        },
    },
    Instruction {
        opcode: 0x3c,
        name: InstructionCode::TOP,
        mode: InstructionMode::Implied,
        operation: |cpu| {
            cpu.get_absolute_x_value(true); // Need extra cycle
            InstructionResult {
                step: 3,
                remaining_cycles: 4,
            }
        },
    },
    Instruction {
        opcode: 0x3d,
        name: InstructionCode::AND,
        mode: InstructionMode::AbsoluteX,
        operation: |cpu| {
            cpu.accumulator &= cpu.get_absolute_x_value(true);
            cpu.set_flags_nz(cpu.accumulator);
            InstructionResult {
                step: 3,
                remaining_cycles: 4,
            }
        },
    },
    Instruction {
        opcode: 0x3e,
        name: InstructionCode::ROL,
        mode: InstructionMode::AbsoluteX,
        operation: |cpu| {
            let val = cpu.get_absolute_x_value(true);
            let carry = cpu.carry as u8;
            cpu.carry = (val >> 7) != 0;
            let val = (val << 1) | (carry);
            cpu.set_absolute_x(val, true);
            cpu.set_flags_nz(val);
            InstructionResult {
                step: 3,
                remaining_cycles: 7,
            }
        },
    },
    Instruction {
        opcode: 0x3f,
        name: InstructionCode::RLA,
        mode: InstructionMode::AbsoluteX,
        operation: |cpu| {
            cpu.fn_0x3e_with_no_additionnal_cycles(); // ROL
            cpu.fn_0x3d_with_no_additionnal_cycles(); // AND
            InstructionResult {
                step: 3,
                remaining_cycles: 7,
            }
        },
    },
    Instruction {
        opcode: 0x40,
        name: InstructionCode::RTI,
        mode: InstructionMode::Implied,
        operation: |cpu| {
            let stack_value = cpu.pull();
            cpu.set_status_register(stack_value);
            let low = cpu.pull() as u16;
            let high = cpu.pull() as u16;
            cpu.program_counter = (high << 8) + low;
            InstructionResult {
                step: 0,
                remaining_cycles: 6,
            }
        },
    },
    Instruction {
        opcode: 0x41,
        name: InstructionCode::EOR,
        mode: InstructionMode::IndirectX,
        operation: |cpu| {
            cpu.accumulator ^= cpu.get_indirect_x_value();
            cpu.set_flags_nz(cpu.accumulator);
            InstructionResult {
                step: 2,
                remaining_cycles: 6,
            }
        },
    },
    Instruction {
        opcode: 0x42,
        name: InstructionCode::Unknown,
        mode: InstructionMode::Undefined,
        operation: |_cpu| panic!("Unknow instruction 0x42"),
    },
    Instruction {
        opcode: 0x43,
        name: InstructionCode::SRE,
        mode: InstructionMode::IndirectX,
        operation: |cpu| {
            let val = cpu.get_indirect_x_value();
            cpu.carry = (val & 1) != 0;
            let val = val >> 1;
            cpu.set_indirect_x(val);
            (INSTRUCTION_TABLE[0x41].operation)(cpu); // EOR
            InstructionResult {
                step: 2,
                remaining_cycles: 8,
            }
        },
    },
    Instruction {
        opcode: 0x44,
        name: InstructionCode::DOP,
        mode: InstructionMode::Implied,
        operation: |_cpu| InstructionResult {
            step: 2,
            remaining_cycles: 3,
        },
    },
    Instruction {
        opcode: 0x45,
        name: InstructionCode::EOR,
        mode: InstructionMode::ZeroPage,
        operation: |cpu| {
            cpu.accumulator ^= cpu.get_zero_page_value();
            cpu.set_flags_nz(cpu.accumulator);
            InstructionResult {
                step: 2,
                remaining_cycles: 3,
            }
        },
    },
    Instruction {
        opcode: 0x46,
        name: InstructionCode::LSR,
        mode: InstructionMode::ZeroPage,
        operation: |cpu| {
            let mut value = cpu.get_zero_page_value();
            cpu.carry = (value & 1) == 1;
            value >>= 1;
            cpu.set_zero_page(value);
            cpu.set_flags_nz(value);
            InstructionResult {
                step: 2,
                remaining_cycles: 5,
            }
        },
    },
    Instruction {
        opcode: 0x47,
        name: InstructionCode::SRE,
        mode: InstructionMode::ZeroPage,
        operation: |cpu| {
            (INSTRUCTION_TABLE[0x46].operation)(cpu); // LSR
            (INSTRUCTION_TABLE[0x45].operation)(cpu); // EOR
            InstructionResult {
                step: 2,
                remaining_cycles: 5,
            }
        },
    },
    Instruction {
        opcode: 0x48,
        name: InstructionCode::PHA,
        mode: InstructionMode::Implied,
        operation: |cpu| {
            cpu.push(cpu.accumulator);
            InstructionResult {
                step: 1,
                remaining_cycles: 3,
            }
        },
    },
    Instruction {
        opcode: 0x49,
        name: InstructionCode::EOR,
        mode: InstructionMode::Immediate,
        operation: |cpu| {
            cpu.accumulator ^= cpu.get_immediate();
            cpu.set_flags_nz(cpu.accumulator);
            InstructionResult {
                step: 2,
                remaining_cycles: 2,
            }
        },
    },
    Instruction {
        opcode: 0x4a,
        name: InstructionCode::LSR,
        mode: InstructionMode::Accumulator,
        operation: |cpu| {
            cpu.carry = (cpu.accumulator & 1) == 1;
            cpu.accumulator >>= 1;
            cpu.set_flags_nz(cpu.accumulator);
            InstructionResult {
                step: 1,
                remaining_cycles: 2,
            }
        },
    },
    Instruction {
        opcode: 0x4b,
        name: InstructionCode::Unknown,
        mode: InstructionMode::Undefined,
        operation: |_cpu| panic!("Unknow instruction 0x4b"),
    },
    Instruction {
        opcode: 0x4c,
        name: InstructionCode::JMP,
        mode: InstructionMode::Absolute,
        operation: |cpu| {
            cpu.program_counter = cpu.get_absolute_address();
            InstructionResult {
                step: 0,
                remaining_cycles: 3,
            }
        },
    },
    Instruction {
        opcode: 0x4d,
        name: InstructionCode::EOR,
        mode: InstructionMode::Absolute,
        operation: |cpu| {
            cpu.accumulator ^= cpu.get_absolute_value();
            cpu.set_flags_nz(cpu.accumulator);
            InstructionResult {
                step: 3,
                remaining_cycles: 4,
            }
        },
    },
    Instruction {
        opcode: 0x4e,
        name: InstructionCode::LSR,
        mode: InstructionMode::Absolute,
        operation: |cpu| {
            let mut value = cpu.get_absolute_value();
            cpu.carry = (value & 1) == 1;
            value >>= 1;
            cpu.set_absolute(value);
            cpu.set_flags_nz(value);
            InstructionResult {
                step: 3,
                remaining_cycles: 6,
            }
        },
    },
    Instruction {
        opcode: 0x4f,
        name: InstructionCode::SRE,
        mode: InstructionMode::Absolute,
        operation: |cpu| {
            (INSTRUCTION_TABLE[0x4e].operation)(cpu); // LSR
            (INSTRUCTION_TABLE[0x4d].operation)(cpu); // EOR
            InstructionResult {
                step: 3,
                remaining_cycles: 6,
            }
        },
    },
    Instruction {
        opcode: 0x50,
        name: InstructionCode::BVC,
        mode: InstructionMode::Relative,
        operation: |cpu| {
            let signed: i8 = cpu.get_immediate() as i8;
            if !cpu.overflow {
                cpu.program_counter = cpu.program_counter.wrapping_add(signed as u16);
                cpu.additionnal_cycles += 1;
            }
            InstructionResult {
                step: 2,
                remaining_cycles: 2,
            }
        },
    },
    Instruction {
        opcode: 0x51,
        name: InstructionCode::EOR,
        mode: InstructionMode::IndirectY,
        operation: |cpu| {
            cpu.accumulator ^= cpu.get_indirect_y_value(true);
            cpu.set_flags_nz(cpu.accumulator);
            InstructionResult {
                step: 2,
                remaining_cycles: 5,
            }
        },
    },
    Instruction {
        opcode: 0x52,
        name: InstructionCode::Unknown,
        mode: InstructionMode::Undefined,
        operation: |_cpu| panic!("Unknow instruction 0x52"),
    },
    Instruction {
        opcode: 0x53,
        name: InstructionCode::SRE,
        mode: InstructionMode::IndirectY,
        operation: |cpu| {
            let val = cpu.get_indirect_y_value(false);
            cpu.carry = (val & 1) != 0;
            let val = val >> 1;
            cpu.set_indirect_y(val, false);
            cpu.fn_0x51_with_no_additionnal_cycles(); // EOR
            InstructionResult {
                step: 2,
                remaining_cycles: 8,
            }
        },
    },
    Instruction {
        opcode: 0x54,
        name: InstructionCode::DOP,
        mode: InstructionMode::Implied,
        operation: |_cpu| InstructionResult {
            step: 2,
            remaining_cycles: 4,
        },
    },
    Instruction {
        opcode: 0x55,
        name: InstructionCode::EOR,
        mode: InstructionMode::ZeroPageX,
        operation: |cpu| {
            cpu.accumulator ^= cpu.get_zero_page_x_value();
            cpu.set_flags_nz(cpu.accumulator);
            InstructionResult {
                step: 2,
                remaining_cycles: 4,
            }
        },
    },
    Instruction {
        opcode: 0x56,
        name: InstructionCode::LSR,
        mode: InstructionMode::ZeroPageX,
        operation: |cpu| {
            let mut value = cpu.get_zero_page_x_value();
            cpu.carry = (value & 1) == 1;
            value >>= 1;
            cpu.set_zero_page_x(value);
            cpu.set_flags_nz(value);
            InstructionResult {
                step: 2,
                remaining_cycles: 6,
            }
        },
    },
    Instruction {
        opcode: 0x57,
        name: InstructionCode::SRE,
        mode: InstructionMode::ZeroPageX,
        operation: |cpu| {
            (INSTRUCTION_TABLE[0x56].operation)(cpu); // LSR
            (INSTRUCTION_TABLE[0x55].operation)(cpu); // EOR
            InstructionResult {
                step: 2,
                remaining_cycles: 6,
            }
        },
    },
    Instruction {
        opcode: 0x58,
        name: InstructionCode::CLI,
        mode: InstructionMode::Implied,
        operation: |cpu| {
            cpu.interrupt = false;
            InstructionResult {
                step: 1,
                remaining_cycles: 2,
            }
        },
    },
    Instruction {
        opcode: 0x59,
        name: InstructionCode::EOR,
        mode: InstructionMode::AbsoluteY,
        operation: |cpu| {
            cpu.accumulator ^= cpu.get_absolute_y_value(true);
            cpu.set_flags_nz(cpu.accumulator);
            InstructionResult {
                step: 3,
                remaining_cycles: 4,
            }
        },
    },
    Instruction {
        opcode: 0x5a,
        name: InstructionCode::NOP,
        mode: InstructionMode::Implied,
        operation: |_cpu| InstructionResult {
            step: 1,
            remaining_cycles: 2,
        },
    },
    Instruction {
        opcode: 0x5b,
        name: InstructionCode::SRE,
        mode: InstructionMode::ZeroPage,
        operation: |cpu| {
            let val = cpu.get_absolute_y_value(false);
            cpu.carry = (val & 1) != 0;
            let val = val >> 1;
            cpu.set_absolute_y(val, false);
            cpu.fn_0x59_with_no_additionnal_cycles(); // EOR
            InstructionResult {
                step: 3,
                remaining_cycles: 7,
            }
        },
    },
    Instruction {
        opcode: 0x5c,
        name: InstructionCode::TOP,
        mode: InstructionMode::Implied,
        operation: |cpu| {
            cpu.get_absolute_x_value(true); // Need extra cycle
            InstructionResult {
                step: 3,
                remaining_cycles: 4,
            }
        },
    },
    Instruction {
        opcode: 0x5d,
        name: InstructionCode::EOR,
        mode: InstructionMode::AbsoluteX,
        operation: |cpu| {
            cpu.accumulator ^= cpu.get_absolute_x_value(true);
            cpu.set_flags_nz(cpu.accumulator);
            InstructionResult {
                step: 3,
                remaining_cycles: 4,
            }
        },
    },
    Instruction {
        opcode: 0x5e,
        name: InstructionCode::LSR,
        mode: InstructionMode::AbsoluteX,
        operation: |cpu| {
            let mut value = cpu.get_absolute_x_value(true);
            cpu.carry = (value & 1) == 1;
            value >>= 1;
            cpu.set_absolute_x(value, true);
            cpu.set_flags_nz(value);
            InstructionResult {
                step: 3,
                remaining_cycles: 7,
            }
        },
    },
    Instruction {
        opcode: 0x5f,
        name: InstructionCode::SRE,
        mode: InstructionMode::AbsoluteX,
        operation: |cpu| {
            cpu.fn_0x5e_with_no_additionnal_cycles(); // LSR
            cpu.fn_0x5d_with_no_additionnal_cycles(); // EOR
            InstructionResult {
                step: 3,
                remaining_cycles: 7,
            }
        },
    },
    Instruction {
        opcode: 0x60,
        name: InstructionCode::RTS,
        mode: InstructionMode::Implied,
        operation: |cpu| {
            let low = cpu.pull() as u16;
            let high = cpu.pull() as u16;
            cpu.program_counter = (high << 8) + low + 1; // JSR increment only by two, and RTS add the third
            InstructionResult {
                step: 0,
                remaining_cycles: 6,
            }
        },
    },
    Instruction {
        opcode: 0x61,
        name: InstructionCode::ADC,
        mode: InstructionMode::IndirectX,
        operation: |cpu| {
            let indirect = cpu.get_indirect_x_value();
            cpu.adc(indirect);
            InstructionResult {
                step: 2,
                remaining_cycles: 6,
            }
        },
    },
    Instruction {
        opcode: 0x62,
        name: InstructionCode::Unknown,
        mode: InstructionMode::Undefined,
        operation: |_cpu| panic!("Unknow instruction 0x62"),
    },
    Instruction {
        opcode: 0x63,
        name: InstructionCode::RRA,
        mode: InstructionMode::IndirectX,
        operation: |cpu| {
            let val = cpu.get_indirect_x_value();
            let carry = val & 1;
            let val = (val >> 1) | ((cpu.carry as u8) << 7);
            cpu.carry = carry != 0;
            cpu.set_indirect_x(val);
            (INSTRUCTION_TABLE[0x61].operation)(cpu);
            InstructionResult {
                step: 2,
                remaining_cycles: 8,
            }
        },
    },
    Instruction {
        opcode: 0x64,
        name: InstructionCode::DOP,
        mode: InstructionMode::Implied,
        operation: |_cpu| InstructionResult {
            step: 2,
            remaining_cycles: 3,
        },
    },
    Instruction {
        opcode: 0x65,
        name: InstructionCode::ADC,
        mode: InstructionMode::ZeroPage,
        operation: |cpu| {
            let zeropage = cpu.get_zero_page_value();
            cpu.adc(zeropage);
            InstructionResult {
                step: 2,
                remaining_cycles: 3,
            }
        },
    },
    Instruction {
        opcode: 0x66,
        name: InstructionCode::ROR,
        mode: InstructionMode::ZeroPage,
        operation: |cpu| {
            let val = cpu.get_zero_page_value();
            let carry = val & 1;
            let val = (val >> 1) | ((cpu.carry as u8) << 7);
            cpu.carry = carry != 0;
            cpu.set_zero_page(val);
            cpu.set_flags_nz(val);
            InstructionResult {
                step: 2,
                remaining_cycles: 5,
            }
        },
    },
    Instruction {
        opcode: 0x67,
        name: InstructionCode::RRA,
        mode: InstructionMode::ZeroPage,
        operation: |cpu| {
            (INSTRUCTION_TABLE[0x66].operation)(cpu);
            (INSTRUCTION_TABLE[0x65].operation)(cpu);
            InstructionResult {
                step: 2,
                remaining_cycles: 5,
            }
        },
    },
    Instruction {
        opcode: 0x68,
        name: InstructionCode::PLA,
        mode: InstructionMode::Implied,
        operation: |cpu| {
            cpu.accumulator = cpu.pull();
            cpu.set_flags_nz(cpu.accumulator);
            InstructionResult {
                step: 1,
                remaining_cycles: 4,
            }
        },
    },
    Instruction {
        opcode: 0x69,
        name: InstructionCode::ADC,
        mode: InstructionMode::Immediate,
        operation: |cpu| {
            let immediate = cpu.get_immediate();
            cpu.adc(immediate);
            InstructionResult {
                step: 2,
                remaining_cycles: 2,
            }
        },
    },
    Instruction {
        opcode: 0x6a,
        name: InstructionCode::ROR,
        mode: InstructionMode::Accumulator,
        operation: |cpu| {
            let carry = cpu.accumulator & 1;
            cpu.accumulator = (cpu.accumulator >> 1) | ((cpu.carry as u8) << 7);
            cpu.carry = carry != 0;
            cpu.set_flags_nz(cpu.accumulator);
            InstructionResult {
                step: 1,
                remaining_cycles: 2,
            }
        },
    },
    Instruction {
        opcode: 0x6b,
        name: InstructionCode::Unknown,
        mode: InstructionMode::Undefined,
        operation: |_cpu| panic!("Unknow instruction 0x6b"),
    },
    Instruction {
        opcode: 0x6c,
        name: InstructionCode::JMP,
        mode: InstructionMode::Indirect,
        operation: |cpu| {
            let mut address = cpu.get_absolute_address();
            if address & 0xFF == 0xFF {
                // Strange behaviour in nestest.nes where direct jump to re-aligned address where address at end of page
                address += 1;
            } else {
                address = cpu.bus.read_rom_16(address);
            }
            cpu.program_counter = address;
            InstructionResult {
                step: 0,
                remaining_cycles: 5,
            }
        },
    },
    Instruction {
        opcode: 0x6d,
        name: InstructionCode::ADC,
        mode: InstructionMode::Absolute,
        operation: |cpu| {
            let absolute = cpu.get_absolute_value();
            cpu.adc(absolute);
            InstructionResult {
                step: 3,
                remaining_cycles: 4,
            }
        },
    },
    Instruction {
        opcode: 0x6e,
        name: InstructionCode::ROR,
        mode: InstructionMode::Absolute,
        operation: |cpu| {
            let val = cpu.get_absolute_value();
            let carry = val & 1;
            let val = (val >> 1) | ((cpu.carry as u8) << 7);
            cpu.carry = carry != 0;
            cpu.set_absolute(val);
            cpu.set_flags_nz(val);
            InstructionResult {
                step: 3,
                remaining_cycles: 6,
            }
        },
    },
    Instruction {
        opcode: 0x6f,
        name: InstructionCode::RRA,
        mode: InstructionMode::Absolute,
        operation: |cpu| {
            (INSTRUCTION_TABLE[0x6e].operation)(cpu);
            (INSTRUCTION_TABLE[0x6d].operation)(cpu);
            InstructionResult {
                step: 3,
                remaining_cycles: 6,
            }
        },
    },
    Instruction {
        opcode: 0x70,
        name: InstructionCode::BVS,
        mode: InstructionMode::Relative,
        operation: |cpu| {
            let old_pc = cpu.program_counter + 2;
            let signed: i8 = cpu.get_immediate() as i8;
            if cpu.overflow {
                cpu.program_counter = cpu.program_counter.wrapping_add(signed as u16);
                cpu.additionnal_cycles += 1;
                if cpu.program_counter & 0xff00 != old_pc & 0xff00 {
                    cpu.additionnal_cycles += 1
                }
            }
            InstructionResult {
                step: 2,
                remaining_cycles: 2,
            }
        },
    },
    Instruction {
        opcode: 0x71,
        name: InstructionCode::ADC,
        mode: InstructionMode::IndirectY,
        operation: |cpu| {
            let indirect = cpu.get_indirect_y_value(true);
            cpu.adc(indirect);
            InstructionResult {
                step: 2,
                remaining_cycles: 5,
            }
        },
    },
    Instruction {
        opcode: 0x72,
        name: InstructionCode::Unknown,
        mode: InstructionMode::Undefined,
        operation: |_cpu| panic!("Unknow instruction 0x72"),
    },
    Instruction {
        opcode: 0x73,
        name: InstructionCode::RRA,
        mode: InstructionMode::IndirectY,
        operation: |cpu| {
            let val = cpu.get_indirect_y_value(false);
            let carry = val & 1;
            let val = (val >> 1) | ((cpu.carry as u8) << 7);
            cpu.carry = carry != 0;
            cpu.set_indirect_y(val, false);
            cpu.fn_0x71_with_no_additionnal_cycles(); // ADC
            InstructionResult {
                step: 2,
                remaining_cycles: 8,
            }
        },
    },
    Instruction {
        opcode: 0x74,
        name: InstructionCode::DOP,
        mode: InstructionMode::Implied,
        operation: |_cpu| InstructionResult {
            step: 2,
            remaining_cycles: 4,
        },
    },
    Instruction {
        opcode: 0x75,
        name: InstructionCode::ADC,
        mode: InstructionMode::ZeroPageX,
        operation: |cpu| {
            let zeropage = cpu.get_zero_page_x_value();
            cpu.adc(zeropage);
            InstructionResult {
                step: 2,
                remaining_cycles: 4,
            }
        },
    },
    Instruction {
        opcode: 0x76,
        name: InstructionCode::ROR,
        mode: InstructionMode::ZeroPageX,
        operation: |cpu| {
            let val = cpu.get_zero_page_x_value();
            let carry = val & 1;
            let val = (val >> 1) | ((cpu.carry as u8) << 7);
            cpu.carry = carry != 0;
            cpu.set_zero_page_x(val);
            cpu.set_flags_nz(val);
            InstructionResult {
                step: 2,
                remaining_cycles: 6,
            }
        },
    },
    Instruction {
        opcode: 0x77,
        name: InstructionCode::RRA,
        mode: InstructionMode::ZeroPageX,
        operation: |cpu| {
            (INSTRUCTION_TABLE[0x76].operation)(cpu);
            (INSTRUCTION_TABLE[0x75].operation)(cpu);
            InstructionResult {
                step: 2,
                remaining_cycles: 6,
            }
        },
    },
    Instruction {
        opcode: 0x78,
        name: InstructionCode::SEI,
        mode: InstructionMode::Implied,
        operation: |cpu| {
            cpu.interrupt = true;
            InstructionResult {
                step: 1,
                remaining_cycles: 2,
            }
        },
    },
    Instruction {
        opcode: 0x79,
        name: InstructionCode::ADC,
        mode: InstructionMode::AbsoluteY,
        operation: |cpu| {
            let absolute = cpu.get_absolute_y_value(true);
            cpu.adc(absolute);
            InstructionResult {
                step: 3,
                remaining_cycles: 4,
            }
        },
    },
    Instruction {
        opcode: 0x7a,
        name: InstructionCode::NOP,
        mode: InstructionMode::Implied,
        operation: |_cpu| InstructionResult {
            step: 1,
            remaining_cycles: 2,
        },
    },
    Instruction {
        opcode: 0x7b,
        name: InstructionCode::RRA,
        mode: InstructionMode::AbsoluteY,
        operation: |cpu| {
            let val = cpu.get_absolute_y_value(false);
            let carry = val & 1;
            let val = (val >> 1) | ((cpu.carry as u8) << 7);
            cpu.carry = carry != 0;
            cpu.set_absolute_y(val, false);
            cpu.fn_0x79_with_no_additionnal_cycles(); // ADC
            InstructionResult {
                step: 3,
                remaining_cycles: 7,
            }
        },
    },
    Instruction {
        opcode: 0x7c,
        name: InstructionCode::TOP,
        mode: InstructionMode::Implied,
        operation: |cpu| {
            cpu.get_absolute_x_value(true); // Need extra cycle
            InstructionResult {
                step: 3,
                remaining_cycles: 4,
            }
        },
    },
    Instruction {
        opcode: 0x7d,
        name: InstructionCode::ADC,
        mode: InstructionMode::AbsoluteX,
        operation: |cpu| {
            let absolute = cpu.get_absolute_x_value(true);
            cpu.adc(absolute);
            InstructionResult {
                step: 3,
                remaining_cycles: 4,
            }
        },
    },
    Instruction {
        opcode: 0x7e,
        name: InstructionCode::ROR,
        mode: InstructionMode::AbsoluteX,
        operation: |cpu| {
            let val = cpu.get_absolute_x_value(true);
            let carry = val & 1;
            let val = (val >> 1) | ((cpu.carry as u8) << 7);
            cpu.carry = carry != 0;
            cpu.set_absolute_x(val, true);
            cpu.set_flags_nz(val);
            InstructionResult {
                step: 3,
                remaining_cycles: 7,
            }
        },
    },
    Instruction {
        opcode: 0x7f,
        name: InstructionCode::RRA,
        mode: InstructionMode::AbsoluteX,
        operation: |cpu| {
            cpu.fn_0x7e_with_no_additionnal_cycles(); // ROR
            cpu.fn_0x7d_with_no_additionnal_cycles(); // ADC
            InstructionResult {
                step: 3,
                remaining_cycles: 7,
            }
        },
    },
    Instruction {
        opcode: 0x80,
        name: InstructionCode::DOP,
        mode: InstructionMode::Implied,
        operation: |_cpu| InstructionResult {
            step: 2,
            remaining_cycles: 2,
        },
    },
    Instruction {
        opcode: 0x81,
        name: InstructionCode::STA,
        mode: InstructionMode::IndirectX,
        operation: |cpu| {
            let address = cpu.get_indirect_x_address();
            let extra_cycles = cpu.bus.write_rom(address, cpu.accumulator);
            InstructionResult {
                step: 2,
                remaining_cycles: 6 + extra_cycles,
            }
        },
    },
    Instruction {
        opcode: 0x82,
        name: InstructionCode::DOP,
        mode: InstructionMode::Implied,
        operation: |_cpu| InstructionResult {
            step: 2,
            remaining_cycles: 2,
        },
    },
    Instruction {
        opcode: 0x83,
        name: InstructionCode::SAX,
        mode: InstructionMode::Absolute,
        operation: |cpu| {
            let val = cpu.accumulator & cpu.x_register;
            cpu.set_indirect_x(val);
            InstructionResult {
                step: 2,
                remaining_cycles: 6,
            }
        },
    },
    Instruction {
        opcode: 0x84,
        name: InstructionCode::STY,
        mode: InstructionMode::ZeroPage,
        operation: |cpu| {
            let address = cpu.get_zero_page_address();
            cpu.bus.write_rom(address, cpu.y_register);
            InstructionResult {
                step: 2,
                remaining_cycles: 3,
            }
        },
    },
    Instruction {
        opcode: 0x85,
        name: InstructionCode::STA,
        mode: InstructionMode::ZeroPage,
        operation: |cpu| {
            let address = cpu.get_zero_page_address();
            let extra_cycles = cpu.bus.write_rom(address, cpu.accumulator);
            InstructionResult {
                step: 2,
                remaining_cycles: 3 + extra_cycles,
            }
        },
    },
    Instruction {
        opcode: 0x86,
        name: InstructionCode::STX,
        mode: InstructionMode::ZeroPage,
        operation: |cpu| {
            let address = cpu.get_zero_page_address();
            cpu.bus.write_rom(address, cpu.x_register);
            InstructionResult {
                step: 2,
                remaining_cycles: 3,
            }
        },
    },
    Instruction {
        opcode: 0x87,
        name: InstructionCode::SAX,
        mode: InstructionMode::ZeroPage,
        operation: |cpu| {
            let val = cpu.accumulator & cpu.x_register;
            cpu.set_zero_page(val);
            InstructionResult {
                step: 2,
                remaining_cycles: 3,
            }
        },
    },
    Instruction {
        opcode: 0x88,
        name: InstructionCode::DEY,
        mode: InstructionMode::Implied,
        operation: |cpu| {
            cpu.y_register -= 1;
            cpu.set_flags_nz(cpu.y_register);
            InstructionResult {
                step: 1,
                remaining_cycles: 2,
            }
        },
    },
    Instruction {
        opcode: 0x89,
        name: InstructionCode::DOP,
        mode: InstructionMode::Implied,
        operation: |_cpu| InstructionResult {
            step: 2,
            remaining_cycles: 2,
        },
    },
    Instruction {
        opcode: 0x8a,
        name: InstructionCode::TXA,
        mode: InstructionMode::Implied,
        operation: |cpu| {
            cpu.accumulator = cpu.x_register;
            cpu.set_flags_nz(cpu.accumulator);
            InstructionResult {
                step: 1,
                remaining_cycles: 2,
            }
        },
    },
    Instruction {
        opcode: 0x8b,
        name: InstructionCode::Unknown,
        mode: InstructionMode::Undefined,
        operation: |_cpu| panic!("Unknow instruction 0x8b"),
    },
    Instruction {
        opcode: 0x8c,
        name: InstructionCode::STY,
        mode: InstructionMode::Absolute,
        operation: |cpu| {
            let address = cpu.get_absolute_address();
            cpu.bus.write_rom(address, cpu.y_register);
            InstructionResult {
                step: 3,
                remaining_cycles: 4,
            }
        },
    },
    Instruction {
        opcode: 0x8d,
        name: InstructionCode::STA,
        mode: InstructionMode::Absolute,
        operation: |cpu| {
            let address = cpu.get_absolute_address();
            let extra_cycles = cpu.bus.write_rom(address, cpu.accumulator);
            InstructionResult {
                step: 3,
                remaining_cycles: 4 + extra_cycles,
            }
        },
    },
    Instruction {
        opcode: 0x8e,
        name: InstructionCode::STX,
        mode: InstructionMode::Absolute,
        operation: |cpu| {
            let address = cpu.get_absolute_address();
            cpu.bus.write_rom(address, cpu.x_register);
            InstructionResult {
                step: 3,
                remaining_cycles: 4,
            }
        },
    },
    Instruction {
        opcode: 0x8f,
        name: InstructionCode::SAX,
        mode: InstructionMode::Absolute,
        operation: |cpu| {
            let val = cpu.accumulator & cpu.x_register;
            cpu.set_absolute(val);
            InstructionResult {
                step: 3,
                remaining_cycles: 4,
            }
        },
    },
    Instruction {
        opcode: 0x90,
        name: InstructionCode::BCC,
        mode: InstructionMode::Relative,
        operation: |cpu| {
            let old_pc = cpu.program_counter + 2;
            let signed: i8 = cpu.get_immediate() as i8;
            if !cpu.carry {
                cpu.program_counter = cpu.program_counter.wrapping_add(signed as u16);
                cpu.additionnal_cycles += 1;
                if cpu.program_counter & 0xff00 != old_pc & 0xff00 {
                    cpu.additionnal_cycles += 1
                }
            }
            InstructionResult {
                step: 2,
                remaining_cycles: 2,
            }
        },
    },
    Instruction {
        opcode: 0x91,
        name: InstructionCode::STA,
        mode: InstructionMode::IndirectY,
        operation: |cpu| {
            let address = cpu.get_indirect_y_address(false); // No additionnal cycles on STA
            let extra_cycles = cpu.bus.write_rom(address, cpu.accumulator);
            InstructionResult {
                step: 2,
                remaining_cycles: 6 + extra_cycles,
            }
        },
    },
    Instruction {
        opcode: 0x92,
        name: InstructionCode::Unknown,
        mode: InstructionMode::Undefined,
        operation: |_cpu| panic!("Unknow instruction 0x92"),
    },
    Instruction {
        opcode: 0x93,
        name: InstructionCode::Unknown,
        mode: InstructionMode::Undefined,
        operation: |_cpu| panic!("Unknow instruction 0x93"),
    },
    Instruction {
        opcode: 0x94,
        name: InstructionCode::STY,
        mode: InstructionMode::ZeroPageX,
        operation: |cpu| {
            let address = cpu.get_zero_page_x_address();
            cpu.bus.write_rom(address, cpu.y_register);
            InstructionResult {
                step: 2,
                remaining_cycles: 4,
            }
        },
    },
    Instruction {
        opcode: 0x95,
        name: InstructionCode::STA,
        mode: InstructionMode::ZeroPageX,
        operation: |cpu| {
            let address = cpu.get_zero_page_x_address();
            let extra_cycles = cpu.bus.write_rom(address, cpu.accumulator);
            InstructionResult {
                step: 2,
                remaining_cycles: 4 + extra_cycles,
            }
        },
    },
    Instruction {
        opcode: 0x96,
        name: InstructionCode::STX,
        mode: InstructionMode::ZeroPageY,
        operation: |cpu| {
            let address = cpu.get_zero_page_y_address();
            cpu.bus.write_rom(address, cpu.x_register);
            InstructionResult {
                step: 2,
                remaining_cycles: 4,
            }
        },
    },
    Instruction {
        opcode: 0x97,
        name: InstructionCode::SAX,
        mode: InstructionMode::ZeroPageY,
        operation: |cpu| {
            let val = cpu.accumulator & cpu.x_register;
            cpu.set_zero_page_y(val);
            InstructionResult {
                step: 2,
                remaining_cycles: 4,
            }
        },
    },
    Instruction {
        opcode: 0x98,
        name: InstructionCode::TYA,
        mode: InstructionMode::Implied,
        operation: |cpu| {
            cpu.accumulator = cpu.y_register;
            cpu.set_flags_nz(cpu.accumulator);
            InstructionResult {
                step: 1,
                remaining_cycles: 2,
            }
        },
    },
    Instruction {
        opcode: 0x99,
        name: InstructionCode::STA,
        mode: InstructionMode::AbsoluteY,
        operation: |cpu| {
            let address = cpu.get_absolute_y_address(false); // No additionnal cycles on STA
            let extra_cycles = cpu.bus.write_rom(address, cpu.accumulator);
            InstructionResult {
                step: 3,
                remaining_cycles: 5 + extra_cycles,
            }
        },
    },
    Instruction {
        opcode: 0x9a,
        name: InstructionCode::TXS,
        mode: InstructionMode::Implied,
        operation: |cpu| {
            cpu.stack_pointer = cpu.x_register;
            InstructionResult {
                step: 1,
                remaining_cycles: 2,
            }
        },
    },
    Instruction {
        opcode: 0x9b,
        name: InstructionCode::Unknown,
        mode: InstructionMode::Undefined,
        operation: |_cpu| panic!("Unknow instruction 0x9b"),
    },
    Instruction {
        opcode: 0x9c,
        name: InstructionCode::DOP,
        mode: InstructionMode::Implied,
        operation: |_cpu| panic!("Unknow instruction 0x9c"),
    },
    Instruction {
        opcode: 0x9d,
        name: InstructionCode::STA,
        mode: InstructionMode::AbsoluteX,
        operation: |cpu| {
            let address = cpu.get_absolute_x_address(false); // No additionnal cycles on STA
            let extra_cycles = cpu.bus.write_rom(address, cpu.accumulator);
            InstructionResult {
                step: 3,
                remaining_cycles: 5 + extra_cycles,
            }
        },
    },
    Instruction {
        opcode: 0x9e,
        name: InstructionCode::Unknown,
        mode: InstructionMode::Undefined,
        operation: |_cpu| panic!("Unknow instruction 0x9e"),
    },
    Instruction {
        opcode: 0x9f,
        name: InstructionCode::Unknown,
        mode: InstructionMode::Undefined,
        operation: |_cpu| panic!("Unknow instruction 0x9f"),
    },
    Instruction {
        opcode: 0xa0,
        name: InstructionCode::LDY,
        mode: InstructionMode::Immediate,
        operation: |cpu| {
            cpu.y_register = cpu.get_immediate();
            cpu.set_flags_nz(cpu.y_register);
            InstructionResult {
                step: 2,
                remaining_cycles: 2,
            }
        },
    },
    Instruction {
        opcode: 0xa1,
        name: InstructionCode::LDA,
        mode: InstructionMode::IndirectX,
        operation: |cpu| {
            cpu.accumulator = cpu.get_indirect_x_value();
            cpu.set_flags_nz(cpu.accumulator);
            InstructionResult {
                step: 2,
                remaining_cycles: 6,
            }
        },
    },
    Instruction {
        opcode: 0xa2,
        name: InstructionCode::LDX,
        mode: InstructionMode::Immediate,
        operation: |cpu| {
            cpu.x_register = cpu.get_immediate();
            cpu.set_flags_nz(cpu.x_register);
            InstructionResult {
                step: 2,
                remaining_cycles: 2,
            }
        },
    },
    Instruction {
        opcode: 0xa3,
        name: InstructionCode::LAX,
        mode: InstructionMode::IndirectX,
        operation: |cpu| {
            cpu.accumulator = cpu.get_indirect_x_value();
            cpu.x_register = cpu.accumulator;
            cpu.set_flags_nz(cpu.accumulator);
            InstructionResult {
                step: 2,
                remaining_cycles: 6,
            }
        },
    },
    Instruction {
        opcode: 0xa4,
        name: InstructionCode::LDY,
        mode: InstructionMode::ZeroPage,
        operation: |cpu| {
            cpu.y_register = cpu.get_zero_page_value();
            cpu.set_flags_nz(cpu.x_register);
            InstructionResult {
                step: 2,
                remaining_cycles: 3,
            }
        },
    },
    Instruction {
        opcode: 0xa5,
        name: InstructionCode::LDA,
        mode: InstructionMode::ZeroPage,
        operation: |cpu| {
            cpu.accumulator = cpu.get_zero_page_value();
            cpu.set_flags_nz(cpu.accumulator);
            InstructionResult {
                step: 2,
                remaining_cycles: 3,
            }
        },
    },
    Instruction {
        opcode: 0xa6,
        name: InstructionCode::LDX,
        mode: InstructionMode::ZeroPage,
        operation: |cpu| {
            cpu.x_register = cpu.get_zero_page_value();
            cpu.set_flags_nz(cpu.x_register);
            InstructionResult {
                step: 2,
                remaining_cycles: 3,
            }
        },
    },
    Instruction {
        opcode: 0xa7,
        name: InstructionCode::LAX,
        mode: InstructionMode::ZeroPage,
        operation: |cpu| {
            cpu.accumulator = cpu.get_zero_page_value();
            cpu.x_register = cpu.accumulator;
            cpu.set_flags_nz(cpu.accumulator);
            InstructionResult {
                step: 2,
                remaining_cycles: 3,
            }
        },
    },
    Instruction {
        opcode: 0xa8,
        name: InstructionCode::TAY,
        mode: InstructionMode::Implied,
        operation: |cpu| {
            cpu.y_register = cpu.accumulator;
            cpu.set_flags_nz(cpu.y_register);
            InstructionResult {
                step: 1,
                remaining_cycles: 2,
            }
        },
    },
    Instruction {
        opcode: 0xa9,
        name: InstructionCode::LDA,
        mode: InstructionMode::Immediate,
        operation: |cpu| {
            cpu.accumulator = cpu.get_immediate();
            cpu.set_flags_nz(cpu.accumulator);
            InstructionResult {
                step: 2,
                remaining_cycles: 2,
            }
        },
    },
    Instruction {
        opcode: 0xaa,
        name: InstructionCode::TAX,
        mode: InstructionMode::Implied,
        operation: |cpu| {
            cpu.x_register = cpu.accumulator;
            cpu.set_flags_nz(cpu.x_register);
            InstructionResult {
                step: 1,
                remaining_cycles: 2,
            }
        },
    },
    Instruction {
        opcode: 0xab,
        name: InstructionCode::Unknown,
        mode: InstructionMode::Undefined,
        operation: |_cpu| panic!("Unknow instruction 0xab"),
    },
    Instruction {
        opcode: 0xac,
        name: InstructionCode::LDY,
        mode: InstructionMode::Absolute,
        operation: |cpu| {
            cpu.y_register = cpu.get_absolute_value();
            cpu.set_flags_nz(cpu.y_register);
            InstructionResult {
                step: 3,
                remaining_cycles: 4,
            }
        },
    },
    Instruction {
        opcode: 0xad,
        name: InstructionCode::LDA,
        mode: InstructionMode::Absolute,
        operation: |cpu| {
            cpu.accumulator = cpu.get_absolute_value();
            cpu.set_flags_nz(cpu.accumulator);
            InstructionResult {
                step: 3,
                remaining_cycles: 4,
            }
        },
    },
    Instruction {
        opcode: 0xae,
        name: InstructionCode::LDX,
        mode: InstructionMode::Absolute,
        operation: |cpu| {
            cpu.x_register = cpu.get_absolute_value();
            cpu.set_flags_nz(cpu.x_register);
            InstructionResult {
                step: 3,
                remaining_cycles: 4,
            }
        },
    },
    Instruction {
        opcode: 0xaf,
        name: InstructionCode::LAX,
        mode: InstructionMode::Absolute,
        operation: |cpu| {
            cpu.accumulator = cpu.get_absolute_value();
            cpu.x_register = cpu.accumulator;
            cpu.set_flags_nz(cpu.accumulator);
            InstructionResult {
                step: 3,
                remaining_cycles: 4,
            }
        },
    },
    Instruction {
        opcode: 0xb0,
        name: InstructionCode::BCS,
        mode: InstructionMode::Relative,
        operation: |cpu| {
            let old_pc = cpu.program_counter + 2;
            let signed: i8 = cpu.get_immediate() as i8;
            if cpu.carry {
                cpu.program_counter = cpu.program_counter.wrapping_add(signed as u16);
                cpu.additionnal_cycles += 1;
                if cpu.program_counter & 0xff00 != old_pc & 0xff00 {
                    cpu.additionnal_cycles += 1
                }
            }
            InstructionResult {
                step: 2,
                remaining_cycles: 2,
            }
        },
    },
    Instruction {
        opcode: 0xb1,
        name: InstructionCode::LDA,
        mode: InstructionMode::IndirectY,
        operation: |cpu| {
            cpu.accumulator = cpu.get_indirect_y_value(true);
            cpu.set_flags_nz(cpu.accumulator);
            InstructionResult {
                step: 2,
                remaining_cycles: 5,
            }
        },
    },
    Instruction {
        opcode: 0xb2,
        name: InstructionCode::Unknown,
        mode: InstructionMode::Undefined,
        operation: |_cpu| panic!("Unknow instruction 0xb2"),
    },
    Instruction {
        opcode: 0xb3,
        name: InstructionCode::LAX,
        mode: InstructionMode::IndirectY,
        operation: |cpu| {
            cpu.accumulator = cpu.get_indirect_y_value(true);
            cpu.x_register = cpu.accumulator;
            cpu.set_flags_nz(cpu.accumulator);
            InstructionResult {
                step: 2,
                remaining_cycles: 5,
            }
        },
    },
    Instruction {
        opcode: 0xb4,
        name: InstructionCode::LDY,
        mode: InstructionMode::ZeroPageX,
        operation: |cpu| {
            cpu.y_register = cpu.get_zero_page_x_value();
            cpu.set_flags_nz(cpu.y_register);
            InstructionResult {
                step: 2,
                remaining_cycles: 4,
            }
        },
    },
    Instruction {
        opcode: 0xb5,
        name: InstructionCode::LDA,
        mode: InstructionMode::ZeroPageX,
        operation: |cpu| {
            cpu.accumulator = cpu.get_zero_page_x_value();
            cpu.set_flags_nz(cpu.accumulator);
            InstructionResult {
                step: 2,
                remaining_cycles: 4,
            }
        },
    },
    Instruction {
        opcode: 0xb6,
        name: InstructionCode::LDX,
        mode: InstructionMode::ZeroPageY,
        operation: |cpu| {
            cpu.x_register = cpu.get_zero_page_y_value();
            cpu.set_flags_nz(cpu.x_register);
            InstructionResult {
                step: 2,
                remaining_cycles: 4,
            }
        },
    },
    Instruction {
        opcode: 0xb7,
        name: InstructionCode::LAX,
        mode: InstructionMode::ZeroPageY,
        operation: |cpu| {
            cpu.accumulator = cpu.get_zero_page_y_value();
            cpu.x_register = cpu.accumulator;
            cpu.set_flags_nz(cpu.accumulator);
            InstructionResult {
                step: 2,
                remaining_cycles: 4,
            }
        },
    },
    Instruction {
        opcode: 0xb8,
        name: InstructionCode::CLV,
        mode: InstructionMode::Implied,
        operation: |cpu| {
            cpu.overflow = false;
            InstructionResult {
                step: 1,
                remaining_cycles: 2,
            }
        },
    },
    Instruction {
        opcode: 0xb9,
        name: InstructionCode::LDA,
        mode: InstructionMode::AbsoluteY,
        operation: |cpu| {
            cpu.accumulator = cpu.get_absolute_y_value(true);
            cpu.set_flags_nz(cpu.accumulator);
            InstructionResult {
                step: 3,
                remaining_cycles: 4,
            }
        },
    },
    Instruction {
        opcode: 0xba,
        name: InstructionCode::TSX,
        mode: InstructionMode::Implied,
        operation: |cpu| {
            cpu.x_register = cpu.stack_pointer;
            cpu.set_flags_nz(cpu.x_register);
            InstructionResult {
                step: 1,
                remaining_cycles: 2,
            }
        },
    },
    Instruction {
        opcode: 0xbb,
        name: InstructionCode::Unknown,
        mode: InstructionMode::Undefined,
        operation: |_cpu| panic!("Unknow instruction 0xbb"),
    },
    Instruction {
        opcode: 0xbc,
        name: InstructionCode::LDY,
        mode: InstructionMode::AbsoluteX,
        operation: |cpu| {
            cpu.y_register = cpu.get_absolute_x_value(true);
            cpu.set_flags_nz(cpu.y_register);
            InstructionResult {
                step: 3,
                remaining_cycles: 4,
            }
        },
    },
    Instruction {
        opcode: 0xbd,
        name: InstructionCode::LDA,
        mode: InstructionMode::AbsoluteX,
        operation: |cpu| {
            cpu.accumulator = cpu.get_absolute_x_value(true);
            cpu.set_flags_nz(cpu.accumulator);
            InstructionResult {
                step: 3,
                remaining_cycles: 4,
            }
        },
    },
    Instruction {
        opcode: 0xbe,
        name: InstructionCode::LDX,
        mode: InstructionMode::AbsoluteY,
        operation: |cpu| {
            cpu.x_register = cpu.get_absolute_y_value(true);
            cpu.set_flags_nz(cpu.x_register);
            InstructionResult {
                step: 3,
                remaining_cycles: 4,
            }
        },
    },
    Instruction {
        opcode: 0xbf,
        name: InstructionCode::LAX,
        mode: InstructionMode::AbsoluteY,
        operation: |cpu| {
            cpu.accumulator = cpu.get_absolute_y_value(true);
            cpu.x_register = cpu.accumulator;
            cpu.set_flags_nz(cpu.accumulator);
            InstructionResult {
                step: 3,
                remaining_cycles: 4,
            }
        },
    },
    Instruction {
        opcode: 0xc0,
        name: InstructionCode::CPY,
        mode: InstructionMode::Immediate,
        operation: |cpu| {
            let immediate = cpu.get_immediate();
            cpu.cmp(cpu.y_register, immediate);
            InstructionResult {
                step: 2,
                remaining_cycles: 2,
            }
        },
    },
    Instruction {
        opcode: 0xc1,
        name: InstructionCode::CMP,
        mode: InstructionMode::IndirectX,
        operation: |cpu| {
            let indirect_x = cpu.get_indirect_x_value();
            cpu.cmp(cpu.accumulator, indirect_x);
            InstructionResult {
                step: 2,
                remaining_cycles: 6,
            }
        },
    },
    Instruction {
        opcode: 0xc2,
        name: InstructionCode::DOP,
        mode: InstructionMode::Implied,
        operation: |_cpu| InstructionResult {
            step: 2,
            remaining_cycles: 2,
        },
    },
    Instruction {
        opcode: 0xc3,
        name: InstructionCode::DCP,
        mode: InstructionMode::IndirectX,
        operation: |cpu| {
            let value = cpu.get_indirect_x_value();
            let value = value - 1;
            cpu.set_indirect_x(value);
            cpu.cmp(cpu.accumulator, value);
            InstructionResult {
                step: 2,
                remaining_cycles: 8,
            }
        },
    },
    Instruction {
        opcode: 0xc4,
        name: InstructionCode::CPY,
        mode: InstructionMode::ZeroPage,
        operation: |cpu| {
            let zero_page = cpu.get_zero_page_value();
            cpu.cmp(cpu.y_register, zero_page);
            InstructionResult {
                step: 2,
                remaining_cycles: 3,
            }
        },
    },
    Instruction {
        opcode: 0xc5,
        name: InstructionCode::CMP,
        mode: InstructionMode::ZeroPage,
        operation: |cpu| {
            let zero_page = cpu.get_zero_page_value();
            cpu.cmp(cpu.accumulator, zero_page);
            InstructionResult {
                step: 2,
                remaining_cycles: 3,
            }
        },
    },
    Instruction {
        opcode: 0xc6,
        name: InstructionCode::DEC,
        mode: InstructionMode::ZeroPage,
        operation: |cpu| {
            let value = cpu.get_zero_page_value();
            let value = value - 1;
            cpu.set_zero_page(value);
            cpu.set_flags_nz(value);
            InstructionResult {
                step: 2,
                remaining_cycles: 5,
            }
        },
    },
    Instruction {
        opcode: 0xc7,
        name: InstructionCode::DCP,
        mode: InstructionMode::ZeroPage,
        operation: |cpu| {
            let value = cpu.get_zero_page_value();
            let value = value - 1;
            cpu.set_zero_page(value);
            cpu.cmp(cpu.accumulator, value);
            InstructionResult {
                step: 2,
                remaining_cycles: 5,
            }
        },
    },
    Instruction {
        opcode: 0xc8,
        name: InstructionCode::INY,
        mode: InstructionMode::Implied,
        operation: |cpu| {
            cpu.y_register += 1;
            cpu.set_flags_nz(cpu.y_register);
            InstructionResult {
                step: 1,
                remaining_cycles: 2,
            }
        },
    },
    Instruction {
        opcode: 0xc9,
        name: InstructionCode::CMP,
        mode: InstructionMode::Immediate,
        operation: |cpu| {
            let immediate = cpu.get_immediate();
            cpu.cmp(cpu.accumulator, immediate);
            InstructionResult {
                step: 2,
                remaining_cycles: 2,
            }
        },
    },
    Instruction {
        opcode: 0xca,
        name: InstructionCode::DEX,
        mode: InstructionMode::Implied,
        operation: |cpu| {
            cpu.x_register -= 1;
            cpu.set_flags_nz(cpu.x_register);
            InstructionResult {
                step: 1,
                remaining_cycles: 2,
            }
        },
    },
    Instruction {
        opcode: 0xcb,
        name: InstructionCode::Unknown,
        mode: InstructionMode::Undefined,
        operation: |_cpu| panic!("Unknow instruction 0xcb"),
    },
    Instruction {
        opcode: 0xcc,
        name: InstructionCode::CPY,
        mode: InstructionMode::Absolute,
        operation: |cpu| {
            let absolute = cpu.get_absolute_value();
            cpu.cmp(cpu.y_register, absolute);
            InstructionResult {
                step: 3,
                remaining_cycles: 4,
            }
        },
    },
    Instruction {
        opcode: 0xcd,
        name: InstructionCode::CMP,
        mode: InstructionMode::Absolute,
        operation: |cpu| {
            let absolute = cpu.get_absolute_value();
            cpu.cmp(cpu.accumulator, absolute);
            InstructionResult {
                step: 3,
                remaining_cycles: 4,
            }
        },
    },
    Instruction {
        opcode: 0xce,
        name: InstructionCode::DEC,
        mode: InstructionMode::Absolute,
        operation: |cpu| {
            let value = cpu.get_absolute_value();
            let value = value - 1;
            cpu.set_absolute(value);
            cpu.set_flags_nz(value);
            InstructionResult {
                step: 3,
                remaining_cycles: 6,
            }
        },
    },
    Instruction {
        opcode: 0xcf,
        name: InstructionCode::DCP,
        mode: InstructionMode::Absolute,
        operation: |cpu| {
            let value = cpu.get_absolute_value();
            let value = value - 1;
            cpu.set_absolute(value);
            cpu.cmp(cpu.accumulator, value);
            InstructionResult {
                step: 3,
                remaining_cycles: 6,
            }
        },
    },
    Instruction {
        opcode: 0xd0,
        name: InstructionCode::BNE,
        mode: InstructionMode::Relative,
        operation: |cpu| {
            let old_pc = cpu.program_counter + 2;
            let signed: i8 = cpu.get_immediate() as i8;
            if !cpu.zero {
                cpu.program_counter = cpu.program_counter.wrapping_add(signed as u16);
                cpu.additionnal_cycles += 1;
                if cpu.program_counter & 0xff00 != old_pc & 0xff00 {
                    cpu.additionnal_cycles += 1;
                }
            }
            InstructionResult {
                step: 2,
                remaining_cycles: 2,
            }
        },
    },
    Instruction {
        opcode: 0xd1,
        name: InstructionCode::CMP,
        mode: InstructionMode::IndirectY,
        operation: |cpu| {
            let indirect_y = cpu.get_indirect_y_value(true);
            cpu.cmp(cpu.accumulator, indirect_y);
            InstructionResult {
                step: 2,
                remaining_cycles: 5,
            }
        },
    },
    Instruction {
        opcode: 0xd2,
        name: InstructionCode::Unknown,
        mode: InstructionMode::Undefined,
        operation: |_cpu| panic!("Unknow instruction 0xd2"),
    },
    Instruction {
        opcode: 0xd3,
        name: InstructionCode::DCP,
        mode: InstructionMode::IndirectY,
        operation: |cpu| {
            let value = cpu.get_indirect_y_value(false);
            let value = value - 1;
            cpu.set_indirect_y(value, false);
            cpu.cmp(cpu.accumulator, value);
            InstructionResult {
                step: 2,
                remaining_cycles: 8,
            }
        },
    },
    Instruction {
        opcode: 0xd4,
        name: InstructionCode::DOP,
        mode: InstructionMode::Implied,
        operation: |_cpu| InstructionResult {
            step: 2,
            remaining_cycles: 4,
        },
    },
    Instruction {
        opcode: 0xd5,
        name: InstructionCode::CMP,
        mode: InstructionMode::ZeroPageX,
        operation: |cpu| {
            let zero_page_x = cpu.get_zero_page_x_value();
            cpu.cmp(cpu.accumulator, zero_page_x);
            InstructionResult {
                step: 2,
                remaining_cycles: 4,
            }
        },
    },
    Instruction {
        opcode: 0xd6,
        name: InstructionCode::DEC,
        mode: InstructionMode::ZeroPageX,
        operation: |cpu| {
            let value = cpu.get_zero_page_x_value();
            let value = value - 1;
            cpu.set_zero_page_x(value);
            cpu.set_flags_nz(value);
            InstructionResult {
                step: 2,
                remaining_cycles: 6,
            }
        },
    },
    Instruction {
        opcode: 0xd7,
        name: InstructionCode::DCP,
        mode: InstructionMode::ZeroPageX,
        operation: |cpu| {
            let value = cpu.get_zero_page_x_value();
            let value = value - 1;
            cpu.set_zero_page_x(value);
            cpu.cmp(cpu.accumulator, value);
            InstructionResult {
                step: 2,
                remaining_cycles: 6,
            }
        },
    },
    Instruction {
        opcode: 0xd8,
        name: InstructionCode::CLD,
        mode: InstructionMode::Implied,
        operation: |cpu| {
            cpu.decimal = false;
            InstructionResult {
                step: 1,
                remaining_cycles: 2,
            }
        },
    },
    Instruction {
        opcode: 0xd9,
        name: InstructionCode::CMP,
        mode: InstructionMode::AbsoluteY,
        operation: |cpu| {
            let absolute_y = cpu.get_absolute_y_value(true);
            cpu.cmp(cpu.accumulator, absolute_y);
            InstructionResult {
                step: 3,
                remaining_cycles: 4,
            }
        },
    },
    Instruction {
        opcode: 0xda,
        name: InstructionCode::NOP,
        mode: InstructionMode::Implied,
        operation: |_cpu| InstructionResult {
            step: 1,
            remaining_cycles: 2,
        },
    },
    Instruction {
        opcode: 0xdb,
        name: InstructionCode::DCP,
        mode: InstructionMode::AbsoluteY,
        operation: |cpu| {
            let value = cpu.get_absolute_y_value(false);
            let value = value - 1;
            cpu.set_absolute_y(value, false);
            cpu.cmp(cpu.accumulator, value);
            InstructionResult {
                step: 3,
                remaining_cycles: 7,
            }
        },
    },
    Instruction {
        opcode: 0xdc,
        name: InstructionCode::TOP,
        mode: InstructionMode::Implied,
        operation: |cpu| {
            cpu.get_absolute_x_value(true); // Need extra cycle
            InstructionResult {
                step: 3,
                remaining_cycles: 4,
            }
        },
    },
    Instruction {
        opcode: 0xdd,
        name: InstructionCode::CMP,
        mode: InstructionMode::AbsoluteX,
        operation: |cpu| {
            let absolute_x = cpu.get_absolute_x_value(true);
            cpu.cmp(cpu.accumulator, absolute_x);
            InstructionResult {
                step: 3,
                remaining_cycles: 4,
            }
        },
    },
    Instruction {
        opcode: 0xde,
        name: InstructionCode::DEC,
        mode: InstructionMode::AbsoluteX,
        operation: |cpu| {
            let value = cpu.get_absolute_x_value(true);
            let value = value - 1;
            cpu.set_absolute_x(value, true);
            cpu.set_flags_nz(value);
            InstructionResult {
                step: 3,
                remaining_cycles: 7,
            }
        },
    },
    Instruction {
        opcode: 0xdf,
        name: InstructionCode::DCP,
        mode: InstructionMode::AbsoluteX,
        operation: |cpu| {
            let value = cpu.get_absolute_x_value(false);
            let value = value - 1;
            cpu.set_absolute_x(value, false);
            cpu.cmp(cpu.accumulator, value);
            InstructionResult {
                step: 3,
                remaining_cycles: 7,
            }
        },
    },
    Instruction {
        opcode: 0xe0,
        name: InstructionCode::CPX,
        mode: InstructionMode::Immediate,
        operation: |cpu| {
            let immediate = cpu.get_immediate();
            cpu.cmp(cpu.x_register, immediate);
            InstructionResult {
                step: 2,
                remaining_cycles: 2,
            }
        },
    },
    Instruction {
        opcode: 0xe1,
        name: InstructionCode::SBC,
        mode: InstructionMode::IndirectX,
        operation: |cpu| {
            let value = cpu.get_indirect_x_value();
            cpu.sbc(value);
            InstructionResult {
                step: 2,
                remaining_cycles: 6,
            }
        },
    },
    Instruction {
        opcode: 0xe2,
        name: InstructionCode::DOP,
        mode: InstructionMode::Implied,
        operation: |_cpu| InstructionResult {
            step: 2,
            remaining_cycles: 2,
        },
    },
    Instruction {
        opcode: 0xe3,
        name: InstructionCode::ISC,
        mode: InstructionMode::IndirectX,
        operation: |cpu| {
            let mut value = cpu.get_indirect_x_value();
            value += 1;
            cpu.set_indirect_x(value);
            cpu.sbc(value);
            InstructionResult {
                step: 2,
                remaining_cycles: 8,
            }
        },
    },
    Instruction {
        opcode: 0xe4,
        name: InstructionCode::CPX,
        mode: InstructionMode::ZeroPage,
        operation: |cpu| {
            let zero_page = cpu.get_zero_page_value();
            cpu.cmp(cpu.x_register, zero_page);
            InstructionResult {
                step: 2,
                remaining_cycles: 3,
            }
        },
    },
    Instruction {
        opcode: 0xe5,
        name: InstructionCode::SBC,
        mode: InstructionMode::ZeroPage,
        operation: |cpu| {
            let value = cpu.get_zero_page_value();
            cpu.sbc(value);
            InstructionResult {
                step: 2,
                remaining_cycles: 3,
            }
        },
    },
    Instruction {
        opcode: 0xe6,
        name: InstructionCode::INC,
        mode: InstructionMode::ZeroPage,
        operation: |cpu| {
            let mut value = cpu.get_zero_page_value();
            value += 1;
            cpu.set_zero_page(value);
            cpu.set_flags_nz(value);
            InstructionResult {
                step: 2,
                remaining_cycles: 5,
            }
        },
    },
    Instruction {
        opcode: 0xe7,
        name: InstructionCode::ISC,
        mode: InstructionMode::ZeroPage,
        operation: |cpu| {
            let mut value = cpu.get_zero_page_value();
            value += 1;
            cpu.set_zero_page(value);
            cpu.sbc(value);
            InstructionResult {
                step: 2,
                remaining_cycles: 5,
            }
        },
    },
    Instruction {
        opcode: 0xe8,
        name: InstructionCode::INX,
        mode: InstructionMode::Implied,
        operation: |cpu| {
            cpu.x_register += 1;
            cpu.set_flags_nz(cpu.x_register);
            InstructionResult {
                step: 1,
                remaining_cycles: 2,
            }
        },
    },
    Instruction {
        opcode: 0xe9,
        name: InstructionCode::SBC,
        mode: InstructionMode::Immediate,
        operation: |cpu| {
            let value = cpu.get_immediate();
            cpu.sbc(value);
            InstructionResult {
                step: 2,
                remaining_cycles: 2,
            }
        },
    },
    Instruction {
        opcode: 0xea,
        name: InstructionCode::NOP,
        mode: InstructionMode::Implied,
        operation: |_cpu| InstructionResult {
            step: 1,
            remaining_cycles: 2,
        },
    },
    Instruction {
        opcode: 0xeb, // Alias to 0xe9
        name: InstructionCode::SBC,
        mode: InstructionMode::Immediate,
        operation: |cpu| (INSTRUCTION_TABLE[0xe9].operation)(cpu),
    },
    Instruction {
        opcode: 0xec,
        name: InstructionCode::CPX,
        mode: InstructionMode::Absolute,
        operation: |cpu| {
            let absolute = cpu.get_absolute_value();
            cpu.cmp(cpu.x_register, absolute);
            InstructionResult {
                step: 3,
                remaining_cycles: 4,
            }
        },
    },
    Instruction {
        opcode: 0xed,
        name: InstructionCode::SBC,
        mode: InstructionMode::Absolute,
        operation: |cpu| {
            let value = cpu.get_absolute_value();
            cpu.sbc(value);
            InstructionResult {
                step: 3,
                remaining_cycles: 4,
            }
        },
    },
    Instruction {
        opcode: 0xee,
        name: InstructionCode::INC,
        mode: InstructionMode::Absolute,
        operation: |cpu| {
            let mut value = cpu.get_absolute_value();
            value += 1;
            cpu.set_absolute(value);
            cpu.set_flags_nz(value);
            InstructionResult {
                step: 3,
                remaining_cycles: 6,
            }
        },
    },
    Instruction {
        opcode: 0xef,
        name: InstructionCode::ISC,
        mode: InstructionMode::Absolute,
        operation: |cpu| {
            let mut value = cpu.get_absolute_value();
            value += 1;
            cpu.set_absolute(value);
            cpu.sbc(value);
            InstructionResult {
                step: 3,
                remaining_cycles: 6,
            }
        },
    },
    Instruction {
        opcode: 0xf0,
        name: InstructionCode::BNE,
        mode: InstructionMode::Relative,
        operation: |cpu| {
            let old_pc = cpu.program_counter + 2;
            let signed: i8 = cpu.get_immediate() as i8;
            if cpu.zero {
                cpu.program_counter = cpu.program_counter.wrapping_add(signed as u16);
                cpu.additionnal_cycles += 1;
                if (cpu.program_counter + 2) & 0xff00 != old_pc & 0xff00 {
                    // PC+2 to take into account current instruction size
                    cpu.additionnal_cycles += 1;
                }
            }
            InstructionResult {
                step: 2,
                remaining_cycles: 2,
            }
        },
    },
    Instruction {
        opcode: 0xf1,
        name: InstructionCode::SBC,
        mode: InstructionMode::IndirectY,
        operation: |cpu| {
            let value = cpu.get_indirect_y_value(true);
            cpu.sbc(value);
            InstructionResult {
                step: 2,
                remaining_cycles: 5,
            }
        },
    },
    Instruction {
        opcode: 0xf2,
        name: InstructionCode::Unknown,
        mode: InstructionMode::Undefined,
        operation: |_cpu| panic!("Unknow instruction 0xf2"),
    },
    Instruction {
        opcode: 0xf3,
        name: InstructionCode::ISC,
        mode: InstructionMode::IndirectY,
        operation: |cpu| {
            let mut value = cpu.get_indirect_y_value(true);
            value += 1;
            cpu.set_indirect_y(value, true);
            cpu.sbc(value);
            InstructionResult {
                step: 2,
                remaining_cycles: 6,
            }
        },
    },
    Instruction {
        opcode: 0xf4,
        name: InstructionCode::DOP,
        mode: InstructionMode::Implied,
        operation: |_cpu| InstructionResult {
            step: 2,
            remaining_cycles: 4,
        },
    },
    Instruction {
        opcode: 0xf5,
        name: InstructionCode::SBC,
        mode: InstructionMode::ZeroPageX,
        operation: |cpu| {
            let value = cpu.get_zero_page_x_value();
            cpu.sbc(value);
            InstructionResult {
                step: 2,
                remaining_cycles: 4,
            }
        },
    },
    Instruction {
        opcode: 0xf6,
        name: InstructionCode::INC,
        mode: InstructionMode::ZeroPageX,
        operation: |cpu| {
            let mut value = cpu.get_zero_page_x_value();
            value += 1;
            cpu.set_zero_page_x(value);
            cpu.set_flags_nz(value);
            InstructionResult {
                step: 2,
                remaining_cycles: 6,
            }
        },
    },
    Instruction {
        opcode: 0xf7,
        name: InstructionCode::ISC,
        mode: InstructionMode::ZeroPageX,
        operation: |cpu| {
            let mut value = cpu.get_zero_page_x_value();
            value += 1;
            cpu.set_zero_page_x(value);
            cpu.sbc(value);
            InstructionResult {
                step: 2,
                remaining_cycles: 6,
            }
        },
    },
    Instruction {
        opcode: 0xf8,
        name: InstructionCode::SED,
        mode: InstructionMode::Implied,
        operation: |cpu| {
            cpu.decimal = true;
            InstructionResult {
                step: 1,
                remaining_cycles: 2,
            }
        },
    },
    Instruction {
        opcode: 0xf9,
        name: InstructionCode::SBC,
        mode: InstructionMode::AbsoluteY,
        operation: |cpu| {
            let value = cpu.get_absolute_y_value(true);
            cpu.sbc(value);
            InstructionResult {
                step: 3,
                remaining_cycles: 4,
            }
        },
    },
    Instruction {
        opcode: 0xfa,
        name: InstructionCode::NOP,
        mode: InstructionMode::Implied,
        operation: |_cpu| InstructionResult {
            step: 1,
            remaining_cycles: 2,
        },
    },
    Instruction {
        opcode: 0xfb,
        name: InstructionCode::ISC,
        mode: InstructionMode::AbsoluteY,
        operation: |cpu| {
            let mut value = cpu.get_absolute_y_value(false);
            value += 1;
            cpu.set_absolute_y(value, false);
            cpu.sbc(value);
            InstructionResult {
                step: 3,
                remaining_cycles: 7,
            }
        },
    },
    Instruction {
        opcode: 0xfc,
        name: InstructionCode::TOP,
        mode: InstructionMode::Implied,
        operation: |cpu| {
            cpu.get_absolute_x_value(true); // Need extra cycle
            InstructionResult {
                step: 3,
                remaining_cycles: 4,
            }
        },
    },
    Instruction {
        opcode: 0xfd,
        name: InstructionCode::SBC,
        mode: InstructionMode::AbsoluteX,
        operation: |cpu| {
            let value = cpu.get_absolute_x_value(true);
            cpu.sbc(value);
            InstructionResult {
                step: 3,
                remaining_cycles: 4,
            }
        },
    },
    Instruction {
        opcode: 0xfe,
        name: InstructionCode::INC,
        mode: InstructionMode::AbsoluteX,
        operation: |cpu| {
            let mut value = cpu.get_absolute_x_value(true);
            value += 1;
            cpu.set_absolute_x(value, true);
            cpu.set_flags_nz(value);
            InstructionResult {
                step: 3,
                remaining_cycles: 7,
            }
        },
    },
    Instruction {
        opcode: 0xff,
        name: InstructionCode::ISC,
        mode: InstructionMode::AbsoluteX,
        operation: |cpu| {
            let mut value = cpu.get_absolute_x_value(false);
            value += 1;
            cpu.set_absolute_x(value, false);
            cpu.sbc(value);
            InstructionResult {
                step: 3,
                remaining_cycles: 7,
            }
        },
    },
];

#[cfg(test)]
mod tests {
    use crate::cpu::instructions::{InstructionMode, INSTRUCTION_TABLE};

    use super::InstructionCode;

    #[test]
    fn are_opcodes_aligned() {
        for i in 0..=255 {
            assert_eq!(INSTRUCTION_TABLE[i].opcode, i as u8);
        }
    }

    #[test]
    fn are_undefined_only_unknown() {
        for i in 0..=255 {
            if INSTRUCTION_TABLE[i].name == InstructionCode::Unknown {
                assert!(INSTRUCTION_TABLE[i].mode == InstructionMode::Undefined);
            } else {
                assert!(INSTRUCTION_TABLE[i].mode != InstructionMode::Undefined);
            }
        }
    }
}
