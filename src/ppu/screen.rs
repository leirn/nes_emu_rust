//! Screen component
//!

use crate::components;

pub struct Screen {
    scale: u32,
    video_subsystem: sdl2::VideoSubsystem,
    //window: sdl2::video::Window,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
}


impl Screen {
    pub fn new() -> Screen {
        let _scale = 3;
        let _video_subsystem = components::EMULATOR.sdl_context.video().unwrap();
        let _window = _video_subsystem.window("Window", 256 * _scale, 240 * _scale)
            .opengl() // this line DOES NOT enable opengl, but allows you to create/get an OpenGL context from your window.
            .build()
            .unwrap();
        let _canvas = _window.into_canvas()
            .index(Screen::find_sdl_gl_driver().unwrap())
            .build()
            .unwrap();

        Screen {
            scale: _scale,
            video_subsystem: _video_subsystem,
            //window: _window,
            canvas: _canvas,
        }
    }

    fn find_sdl_gl_driver() -> Option<u32> {
        for (index, item) in sdl2::render::drivers().enumerate() {
            if item.name == "opengl" {
                return Some(index as u32);
            }
        }
        None
    }

    pub fn start(&mut self) {
        self.canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        // fills the canvas with the color we set in `set_draw_color`.
        self.canvas.clear();

        // change the color of our drawing with a gold-color ...
        self.canvas.set_draw_color(PALETTE[1]);
        // A draw a rectangle which almost fills our window with it !
        match self.canvas.fill_rect(sdl2::rect::Rect::new(10, 10, 256 * 3 - 20, 240 * 3 - 20)) {
            Err(e) => println!("{:?}", e),
            _ => ()
        }

        // However the canvas has not been updated to the window yet,
        // everything has been processed to an internal buffer,
        // but if we want our buffer to be displayed on the window,
        // we need to call `present`. We need to call this everytime
        // we want to render a new frame on the window.
        self.present();
        // present does not "clear" the buffer, that means that
        // you have to clear it yourself before rendering again,
        // otherwise leftovers of what you've renderer before might
        // show up on the window !
        //
        // A good rule of thumb is to `clear()`, draw every texture
        // needed, and then `present()`; repeat this every new frame.
    }

    pub fn update_pixel(&mut self, x: u8, y: u8, color_index: u8) {
        self.canvas.set_draw_color(PALETTE[color_index as usize]);
        match self.canvas.fill_rect(sdl2::rect::Rect::new(self.scale as i32 * x as i32, self.scale as i32 * y as i32, self.scale, self.scale)) {
            Err(e) => println!("{:?}", e),
            _ => ()
        }
    }

    pub fn present(&mut self) {
        self.canvas.present();
    }
}

const PALETTE: [sdl2::pixels::Color; 2] = [sdl2::pixels::Color::RGB(0, 0, 155), sdl2::pixels::Color::RGB(0, 155, 155), ];