pub struct Interrupt {
    is_nmi: bool,
    is_irq: bool,
    is_frame_updated: bool,
}

impl Interrupt {
    // Create new interrupt object
    pub fn new() -> Interrupt {
        Interrupt {
            is_nmi: false,
            is_irq: false,
            is_frame_updated: false,
        }
    }

    /// Raises an NMI interrupt
    pub fn raise_nmi(&mut self) {
        self.is_nmi = true;
    }

    /// Checked and clear NMI interrupt
    pub fn check_and_clear_nmi(&mut self) -> bool {
        let ret = self.is_nmi;
        self.is_nmi = false;
        ret
    }

    /// Raises an IRQ interrupt
    pub fn _raise_irq(&mut self) {
        self.is_irq = true;
    }

    /// Checked and clear NMI interrupt
    pub fn check_and_clear_irq(&mut self) -> bool {
        let ret = self.is_irq;
        self.is_irq = false;
        ret
    }

    /// Set is_frame_updated to true
    pub fn set_frame_updated(&mut self) {
        self.is_frame_updated = true;
    }

    /// Set is_frame_updated to true
    pub fn check_and_clear_frame_updated(&mut self) -> bool {
        let ret = self.is_frame_updated;
        self.is_frame_updated = false;
        ret
    }
}
