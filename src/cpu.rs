struct Cpu {
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
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            accumulator: 0,
            x_register: 0,
            y_register: 0,
            program_counter: 0,
            stack_pointer: 0,
        }
    }

    pub fn start(&self) {

    }

    pub fn next(&self) {

    }
}
