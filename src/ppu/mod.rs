mod screen;

pub struct Ppu {
    screen: screen::Screen,
}

unsafe impl Sync for Ppu {}
unsafe impl Send for Ppu {}

impl Ppu {
    pub fn new() -> Ppu {
        Ppu {
            screen : screen::Screen::new()
        }
    }

    pub fn start(&mut self) {
        self.screen.start();

        for i in 1..25 {
            for j in 1..25 {
                self.screen.update_pixel(110 + i, 105 + j, 5);
            }
        }
        self.screen.present();
    }

    pub fn next(&self) {
        
    }
}