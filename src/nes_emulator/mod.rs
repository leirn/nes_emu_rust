//! Emulator main engine
use crate::components::{CPU, PPU};

pub struct NesEmulator {
    is_nmi: bool,
    is_irq: bool,
    pause: bool,
    is_frame_updated: bool,
    pub sdl_context: sdl2::Sdl,
    clock: clock::Clock,
}

unsafe impl Sync for NesEmulator {}

impl NesEmulator {
    /// Instantiate the Emulator
    pub fn new() -> NesEmulator {
        let _sdl_context = sdl2::init().unwrap();
        NesEmulator{
            is_nmi: false,
            is_irq: false,
            pause: false,
            is_frame_updated: false,
            sdl_context: _sdl_context,
            clock: clock::Clock::new(),
        }
    }

    /// Starts and runs the Emulator execution 
    pub fn start(&mut self) {
        PPU.lock().unwrap().start();
        CPU.lock().unwrap().start(None);
        PPU.lock().unwrap().next();
        PPU.lock().unwrap().next();
        PPU.lock().unwrap().next();

        let mut continuer:bool = true;

        while continuer {
            if !self.pause {
                if self.is_nmi {
                    self.is_nmi = false;
                    CPU.lock().unwrap().nmi();
                }
                if self.is_irq && CPU.lock().unwrap().getInterruptFlag() {
                    self.is_irq = false;
                    CPU.lock().unwrap().irq();
                }

                CPU.lock().unwrap().next();
                PPU.lock().unwrap().next();
                PPU.lock().unwrap().next();
                PPU.lock().unwrap().next();
                
                if self.is_frame_updated {
                    self.clock.tick(60);
                    self.is_frame_updated = false;
                }
            }

            let mut event_pump = self.sdl_context.event_pump().unwrap();
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
