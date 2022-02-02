use crate::components::{PPU};

pub struct NesEmulator {
    is_nmi:bool,
    is_irq:bool,
    pause:bool,
    pub sdl_context: sdl2::Sdl, 
}

unsafe impl Sync for NesEmulator {}

impl NesEmulator {
    pub fn new() -> NesEmulator {
        let _sdl_context = sdl2::init().unwrap();
        NesEmulator{
            is_nmi: false,
            is_irq: false,
            pause: false,
            sdl_context: _sdl_context,
        }
    }

    pub fn start(&self) {
        PPU.lock().unwrap().start();

        let mut continuer:bool = true;

        while continuer {
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

    pub fn toggle_pause(&mut self) {
        self.pause = self.pause;
    }

    pub fn raise_nmi(&mut self) {
        self.is_nmi = true;
    }

    pub fn raise_irq(&mut self) {
        self.is_irq = true;
    }
}