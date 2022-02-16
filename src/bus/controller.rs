pub struct Controller {
    status: u8,
}

impl Controller {
    /// Instanciate new controller
    pub fn new() -> Controller {
        Controller {
            status: 0,
        }
    }

    /// Get controller status
    pub fn get_status(&self) -> u8 {
        self.status
    }

    /// Set when A button is press
    pub fn set_a(&mut self) {
        self.status |= 1;
    }

    /// Clear when A button is released
    pub fn clear_a(&mut self) {
        self.status &= 0b11111110;
    }

    /// Set when B button is press
    pub fn set_b(&mut self) {
        self.status |= 0b10;
    }

    /// Clear when B button is released
    pub fn clear_b(&mut self) {
        self.status &= 0b11111101;
    }

    /// Set when Select button is press
    pub fn set_select(&mut self) {
        self.status |= 0b100;
    }

    /// Clear when Select button is released
    pub fn clear_select(&mut self) {
        self.status &= 0b11111011;
    }

    /// Set when Start button is press
    pub fn set_start(&mut self) {
        self.status |= 0b1000;
    }

    /// Clear when Start button is released
    pub fn clear_start(&mut self) {
        self.status &= 0b11110111;
    }

    /// Set when Up button is press
    pub fn set_up(&mut self) {
        self.status |= 0b10000;
    }

    /// Clear when Up button is released
    pub fn clear_up(&mut self) {
        self.status &= 0b11101111;
    }

    /// Set when Down button is press
    pub fn set_down(&mut self) {

        self.status |= 0b100000;
    }

    /// Clear when Down button is released
    pub fn clear_down(&mut self) {
        self.status &= 0b11011111;
    }

    /// Set when Left button is press
    pub fn set_left(&mut self) {
        self.status |= 0b1000000;
    }

    /// Clear when Left button is released
    pub fn clear_left(&mut self) {
        self.status &= 0b10111111;
    }

    /// Set when Right button is press
    pub fn set_right(&mut self) {
        self.status |= 0b10000000;
    }

    /// Clear when Right button is released
    pub fn clear_right(&mut self) {
        self.status &= 0b01111111;
    }
}
