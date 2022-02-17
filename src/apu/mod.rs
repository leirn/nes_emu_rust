//! APU Component
pub struct Apu {
    pulse_1: Pulse,
    pulse_2: Pulse,
    noise: Noise,
    triangle: Triangle,
    dmc: Dmc,

    enable_dmc: bool,
    enable_noise: bool,
    enable_triangle: bool,
    enable_pulse_1: bool,
    enable_pulse_2: bool,
}

impl Apu {
    /// Instantiate APU component
    pub fn new() -> Apu {
        Apu {
            pulse_1: Pulse{
                byte_0: 0,
                byte_1: 0,
                byte_2: 0,
                byte_3: 0,
                duty: 0,
                enveloppe_loop_length_counter_halt: false,
                constant_volume: false,
                volume_envelope: 0,
                sweep_unit_enabled: false,
                sweep_unit_period: 0,
                sweep_unit_negate: false,
                sweep_unit_shift: 0,
                timer: 0,
                length_counter_load: 0,
            },
            pulse_2: Pulse{
                byte_0: 0,
                byte_1: 0,
                byte_2: 0,
                byte_3: 0,
                duty: 0,
                enveloppe_loop_length_counter_halt: false,
                constant_volume: false,
                volume_envelope: 0,
                sweep_unit_enabled: false,
                sweep_unit_period: 0,
                sweep_unit_negate: false,
                sweep_unit_shift: 0,
                timer: 0,
                length_counter_load: 0,
            },
            triangle: Triangle {
                byte_0: 0,
                byte_2: 0,
                byte_3: 0,
                lenght_counter_halt_linear_counter_control: false,
                linear_counter_load: 0,
                length_counter_load: 0,
                timer: 0,
            },
            noise: Noise {
                byte_0: 0,
                byte_2: 0,
                byte_3: 0,
                envelope_loop_length_counter_halt: false,
                constant_volume: false,
                volume_envelope: 0,
                loop_noise: false,
                noise_period: 0,
                length_counter_load: 0,
            },
            dmc: Dmc {
                byte_0: 0,
                irq_enabled: false,
                loop_sample: false,
                frequency: 0,
                load_counter: 0,
                sample_address: 0,
                sample_length: 0,
            },

            enable_dmc: false,
            enable_noise: false,
            enable_triangle: false,
            enable_pulse_1: false,
            enable_pulse_2: false,
        }
    }

    /// Next APU cycle
    pub fn next(&self) {

    }

    /// Read APU registers
    pub fn read_registers(&mut self, address: u16) -> u8 {
        match address {
            0x4000 => self.pulse_1.get_byte_0(),
            0x4001 => self.pulse_1.get_byte_1(),
            0x4002 => self.pulse_1.get_byte_2(),
            0x4003 => self.pulse_1.get_byte_3(),
            0x4004 => self.pulse_2.get_byte_0(),
            0x4005 => self.pulse_2.get_byte_1(),
            0x4006 => self.pulse_2.get_byte_2(),
            0x4007 => self.pulse_2.get_byte_3(),
            0x4008 => self.triangle.get_byte_0(),
            0x4009 => 0, // Unused
            0x400a => self.triangle.get_byte_2(),
            0x400b => self.triangle.get_byte_3(),
            0x400c => self.noise.get_byte_0(),
            0x400d => 0, // Unused
            0x400e => self.noise.get_byte_2(),
            0x400f => self.noise.get_byte_3(),
            0x4010 => self.dmc.get_byte_0(),
            0x4011 => self.dmc.get_byte_1(),
            0x4012 => self.dmc.get_byte_2(),
            0x4013 => self.dmc.get_byte_3(),
            0x4015 => self.get_status(),
            0x4017 => self.get_frame_counter(),
            _ => (),
        }
    }

    /// Write APU registers
    pub fn write_registers(&mut self, address: u16, value: u8) {
        match address {
            0x4000 => self.pulse_1.set_byte_0(value),
            0x4001 => self.pulse_1.set_byte_1(value),
            0x4002 => self.pulse_1.set_byte_2(value),
            0x4003 => self.pulse_1.set_byte_3(value),
            0x4004 => self.pulse_2.set_byte_0(value),
            0x4005 => self.pulse_2.set_byte_1(value),
            0x4006 => self.pulse_2.set_byte_2(value),
            0x4007 => self.pulse_2.set_byte_3(value),
            0x4008 => self.triangle.set_byte_0(value),
            0x4009 => (), // Unused
            0x400a => self.triangle.set_byte_2(value),
            0x400b => self.triangle.set_byte_3(value),
            0x400c => self.noise.set_byte_0(value),
            0x400d => (), // Unused
            0x400e => self.noise.set_byte_2(value),
            0x400f => self.noise.set_byte_3(value),
            0x4010 => self.dmc.set_byte_0(value),
            0x4011 => self.dmc.set_byte_1(value),
            0x4012 => self.dmc.set_byte_2(value),
            0x4013 => self.dmc.set_byte_3(value),
            0x4015 => self.set_status(value),
            0x4017 => self.set_frame_counter(value),
            _ => (),
        };
    }

    fn get_status(&self) -> u8 {
        0
    }

    fn set_status(&mut self, value: u8) {

    }

    fn get_frame_counter(&self) -> u8 {
        0
    }

    fn set_frame_counter(&mut self, value: u8) {

    }
}


/// Pulse register
struct Pulse {
    byte_0: u8,
    byte_1: u8,
    byte_2: u8,
    byte_3: u8,
    duty: u8,
    enveloppe_loop_length_counter_halt: bool,
    constant_volume: bool,
    volume_envelope: u8,
    sweep_unit_enabled: bool,
    sweep_unit_period: u8,
    sweep_unit_negate: bool,
    sweep_unit_shift: u8,
    timer: u16,
    length_counter_load: u8,
}

impl Pulse {
    pub fn set_byte_0(&mut self, value: u8) {
        self.byte_0 = value;
        self.duty = value >> 6;
        self.envelope_loop_length_counter_halt = (value & 0b100000) != 0;
        self.constant_volume = (value & 0b10000) != 0;
        self.volume_envelope = value & 0b1111
    }

    pub fn get_byte_0(&self) -> u8 {
        self.byte_0
    }

    pub fn set_byte_1(&mut self, value: u8) {
        self.byte_1 = value;
        self.sweep_unit_enabled = (value & 0b10000000) != 0;
        self.sweep_unit_period = (value >> 4) & 0b111;
        self.sweep_unit_negate = (value & 0b1000) != 0;
        self.sweep_unit_shift = value & 0b111;
    }

    pub fn get_byte_1(&self) -> u8 {
        self.byte_1
    }

    pub fn set_byte_2(&mut self, value: u8) {
        self.byte_2 = value;
        self.timer = (self.timer & 0xff00) | (value as u16);
    }

    pub fn get_byte_2(&self) -> u8 {
        self.byte_2
    }

    pub fn set_byte_3(&mut self, value: u8) {
        self.byte_3 = value;
        self.length_counter_load = value >> 3;
        self.timer = (self.timer & 0xff) | ((value as u16 & 0b111) << 8);
    }

    pub fn get_byte_3(&self) -> u8 {
        self.byte_3
    }
}

/// Triangle register
struct Triangle {
    byte_0: u8,
    byte_2: u8,
    byte_3: u8,
    lenght_counter_halt_linear_counter_control: bool,
    linear_counter_load: u8,
    length_counter_load: u8,
    timer: u16,
}

impl Triangle {
    pub fn set_byte_0(&mut self, value: u8) {
        self.byte_0 = value;
        self.lenght_counter_halt_linear_counter_control = (value & 0b10000000) != 0;
        self.linear_counter_load = value & 0x7f;
    }

    pub fn get_byte_0(&self) -> u8 {
        self.byte_0
    }

    pub fn set_byte_2(&mut self, value: u8) {
        self.byte_2 = value;
        self.timer = (self.timer & 0xff00) | (value as u16);
    }

    pub fn get_byte_2(&self) -> u8 {
        self.byte_2
    }

    pub fn set_byte_3(&mut self, value: u8) {
        self.byte_3 = value;
        self.length_counter_load = value >> 3;
        self.timer = (self.timer & 0xff) | ((value as u16 & 0b111) << 8);
    }

    pub fn get_byte_3(&self) -> u8 {
        self.byte_3
    }
}

/// Noise register
struct Noise {
    byte_0: u8,
    byte_2: u8,
    byte_3: u8,
    envelope_loop_length_counter_halt: bool,
    constant_volume: bool,
    volume_envelope: u8,
    loop_noise: bool,
    noise_period: u8,
    length_counter_load: u8,
}

impl Noise {
    pub fn set_byte_0(&mut self, value: u8) {
        self.byte_0 = value;
        self.envelope_loop_length_counter_halt = (value & 0b100000) != 0;
        self.constant_volume = (value & 0b10000) != 0;
        self.volume_envelope = value & 0b1111;
    }

    pub fn get_byte_0(&self) -> u8 {
        self.byte_0
    }

    pub fn set_byte_2(&mut self, value: u8) {
        self.byte_2 = value;
        self.loop_noise = (value & 0x80) != 0;
        self.noise_period = value & 0xf;
    }

    pub fn get_byte_2(&self) -> u8 {
        self.byte_2
    }

    pub fn set_byte_3(&mut self, value: u8) {
        self.byte_3 = value;
        self.length_counter_load = value >> 3;
    }

    pub fn get_byte_3(&self) -> u8 {
        self.byte_3
    }
}

/// DMC Register
struct Dmc {
    byte_0: u8,
    irq_enabled: bool,
    loop_sample: bool,
    frequency: u8,
    load_counter: u8,
    sample_address: u8,
    sample_length: u8,
}

impl Dmc {
    pub fn set_byte_0(&mut self, value: u8) {
        self.byte_0 = value;
        self.irq_enabled = (value & 0x80) != 0;
        self.loop_sample = (value & 0x40) != 0;
        self.frequency = value & 0xf;
    }

    pub fn get_byte_0(&self) -> u8 {
        self.byte_0
    }

    pub fn set_byte_1(&mut self, value: u8) {
        self.load_counter = value & 0x7f;
    }

    pub fn get_byte_1(&self) -> u8 {
        self.load_counter
    }

    pub fn set_byte_2(&mut self, value: u8) {
        self.sample_address = value;
    }

    pub fn get_byte_2(&self) -> u8 {
        self.sample_address
    }

    pub fn set_byte_3(&mut self, value: u8) {
        self.sample_length = value;
    }

    pub fn get_byte_3(&self) -> u8 {
        self.sample_length
    }
}
