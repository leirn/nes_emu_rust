use sdl2;
mod screen;

pub struct Ppu {
    screen: screen::Screen,
}

unsafe impl Sync for Ppu {}

impl Ppu {
    pub fn new() -> Ppu {
        Ppu {
            screen : screen::Screen::new()
        }
    }

    pub fn start(&self) {
        self.screen.start();
    }

    pub fn next(&self) {
        
    }
}