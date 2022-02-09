//! Emulator main engine
mod clock;
use std::cell::RefCell;
use std::rc::Rc;

pub struct NesEmulator {
    is_nmi: bool,
    is_irq: bool,
    pause: bool,
    is_frame_updated: bool,
    pub sdl_context: Rc<RefCell<sdl2::Sdl>>,
    clock: clock::Clock,
    cartridge: Rc<RefCell<crate::cartridge::Cartridge>>,
    memory: Rc<RefCell<crate::memory::Memory>>,
    apu: Rc<RefCell<crate::apu::Apu>>,
    ppu: Rc<RefCell<crate::ppu::Ppu>>,
    cpu: Rc<RefCell<crate::cpu::Cpu>>,
}

unsafe impl Sync for NesEmulator {}
unsafe impl Send for NesEmulator {}

impl NesEmulator {
    /// Instantiate the Emulator
    pub fn new(rom_file: String) -> NesEmulator {
        let _sdl_context = Rc::new(RefCell::new(sdl2::init().unwrap()));
        println!("SDL Context initialized");

        
        let cartridge = Rc::new(RefCell::new(crate::cartridge::Cartridge::new(rom_file)));
        let apu = Rc::new(RefCell::new(crate::apu::Apu::new()));
        let ppu = Rc::new(RefCell::new(crate::ppu::Ppu::new(cartridge.clone(), _sdl_context.clone())));
        let memory = Rc::new(RefCell::new(crate::memory::Memory::new(Rc::clone(&cartridge), Rc::clone(&ppu), Rc::clone(&apu))));
        let cpu = Rc::new(RefCell::new(crate::cpu::Cpu::new(Rc::clone(&memory))));

        NesEmulator{
            is_nmi: false,
            is_irq: false,
            pause: false,
            is_frame_updated: false,
            sdl_context: _sdl_context,
            clock: clock::Clock::new(),
            cartridge: cartridge,
            memory: memory,
            apu: apu,
            ppu: ppu,
            cpu: cpu,
        }
    }

    /// Starts and runs the Emulator execution 
    pub fn start(&mut self) {
        self.ppu.borrow_mut().start();
        self.cpu.borrow_mut().start(None);
        self.ppu.borrow_mut().next();
        self.ppu.borrow_mut().next();
        self.ppu.borrow_mut().next();

        let mut continuer:bool = true;

        while continuer {
            if !self.pause {
                if self.is_nmi {
                    self.is_nmi = false;
                    self.cpu.borrow_mut().nmi();
                }
                if self.is_irq && self.cpu.borrow_mut().get_interrupt_flag() {
                    self.is_irq = false;
                    self.cpu.borrow_mut().irq();
                }
                self.cpu.borrow_mut().next();
                self.ppu.borrow_mut().next();
                self.ppu.borrow_mut().next();
                self.ppu.borrow_mut().next();
                
                if self.is_frame_updated {
                    self.clock.tick(60);
                    self.is_frame_updated = false;
                }
            }

            let mut event_pump = self.sdl_context.borrow_mut().event_pump().unwrap();
            for event in event_pump.poll_iter() {
                use sdl2::event::Event;
                match event {
                    Event::Quit {..} |
                    Event::KeyDown {
                        keycode: Some(sdl2::keyboard::Keycode::Q), ..
                    } => { continuer = false},
                    _ => ()
                }
            }
        }
    }

    /// Toggles pause on the emulator execution
    pub fn toggle_pause(&mut self) {
        self.pause = !self.pause;
    }

    /// Raises an NMI interrupt
    pub fn raise_nmi(&mut self) {
        self.is_nmi = true;
    }

    /// Raises an IRQ interrupt
    pub fn raise_irq(&mut self) {
        self.is_irq = true;
    }

    /// Set is_frame_updated to true
    pub fn set_frame_updated(&mut self) {
        self.is_frame_updated = true;
    }
}
