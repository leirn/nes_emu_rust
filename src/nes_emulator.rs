
use std::{thread, time::Duration};

pub struct NesEmulator {
    is_nmi:bool,
    is_irq:bool,
    pause:bool,
}

impl NesEmulator {
    pub fn new() -> NesEmulator {
        NesEmulator{
            is_nmi: false,
            is_irq: false,
            pause: false,
        }
    }

    pub fn start(&self, sdl_context:sdl2::Sdl) {

        let mut continuer:bool = true;

        while continuer {
            let mut event_pump = sdl_context.event_pump().unwrap();
            for event in event_pump.poll_iter() {
                use sdl2::event::Event;
                match event {
                    Event::KeyDown {..} => { continuer = false},
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