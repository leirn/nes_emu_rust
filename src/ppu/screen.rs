//! Screen component

use crate::components;
use sdl2::pixels::Color;

pub struct Screen {
    scale: u32,
    //video_subsystem: sdl2::VideoSubsystem,
    //window: sdl2::video::Window,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
}


impl Screen {
    /// Instantiate Screen component
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
            //video_subsystem: _video_subsystem,
            //window: _window,
            canvas: _canvas,
        }
    }

    /// Find SDL GL Driver to initiate SDL window
    fn find_sdl_gl_driver() -> Option<u32> {
        for (index, item) in sdl2::render::drivers().enumerate() {
            if item.name == "opengl" {
                return Some(index as u32);
            }
        }
        None
    }

    /// Start the Screen component
    pub fn start(&mut self) {
        self.canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        // fills the canvas with the color we set in `set_draw_color`.
        self.canvas.clear();

        // change the color of our drawing with a gold-color ...
        self.canvas.set_draw_color(PALETTE[36]);
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

    /// Update a scaled pixel on the buffered canvas
    pub fn update_pixel(&mut self, x: u8, y: u8, color_index: u8) {
        self.canvas.set_draw_color(PALETTE[color_index as usize]);
        match self.canvas.fill_rect(sdl2::rect::Rect::new(self.scale as i32 * x as i32, self.scale as i32 * y as i32, self.scale, self.scale)) {
            Err(e) => println!("{:?}", e),
            _ => ()
        }
    }

    /// Refresh the windows with the buffered canvas
    pub fn present(&mut self) {
        self.canvas.present();
    }
}

/// NES color palette
const PALETTE: [Color; 64] = [
    Color::RGB(84,  84,  84), 	Color::RGB(0,  30, 116),	Color::RGB(8, 16, 144),	    Color::RGB(48, 0, 136), 	Color::RGB(68, 0, 100),  	Color::RGB(92, 0,  48),   	Color::RGB(84, 4, 0),   	Color::RGB(60, 24, 0),   	Color::RGB(32, 42, 0), 	    Color::RGB(8, 58, 0),    	Color::RGB(0, 64, 0),    	Color::RGB(0, 60, 0),    	Color::RGB(0, 50, 60),    	Color::RGB(0,   0,   0),	Color::RGB(0,   0,   0),	Color::RGB(0,   0,   0),
    Color::RGB(152, 150, 152),  Color::RGB(8,  76, 196),   	Color::RGB(48, 50, 236),   	Color::RGB(92, 30, 228),  	Color::RGB(136, 20, 176), 	Color::RGB(160, 20, 100),  	Color::RGB(152, 34, 32),  	Color::RGB(120, 60, 0),   	Color::RGB(84, 90, 0),   	Color::RGB(40, 114, 0),    	Color::RGB(8, 124, 0),    	Color::RGB(0, 118, 40),    	Color::RGB(0, 102, 120),    Color::RGB(0,   0,   0),	Color::RGB(0,   0,   0),	Color::RGB(0,   0,   0),
    Color::RGB(236, 238, 236),  Color::RGB(76, 154, 236),  	Color::RGB(120, 124, 236),  Color::RGB(176, 98, 236),  	Color::RGB(228, 84, 236), 	Color::RGB(236, 88, 180),  	Color::RGB(236, 106, 100),  Color::RGB(212, 136, 32),  	Color::RGB(160, 170, 0),  	Color::RGB(116, 196, 0),   	Color::RGB(76, 208, 32),   	Color::RGB(56, 204, 108),   Color::RGB(56, 180, 204),   Color::RGB(60,  60,  60),	Color::RGB(0,   0,   0),	Color::RGB(0,   0,   0),
    Color::RGB(236, 238, 236),  Color::RGB(168, 204, 236),  Color::RGB(188, 188, 236),  Color::RGB(212, 178, 236),  Color::RGB(236, 174, 236),	Color::RGB(236, 174, 212),  Color::RGB(236, 180, 176),  Color::RGB(228, 196, 144),  Color::RGB(204, 210, 120),  Color::RGB(180, 222, 120),  Color::RGB(168, 226, 144),  Color::RGB(152, 226, 180),  Color::RGB(160, 214, 228),  Color::RGB(160, 162, 160),	Color::RGB(0,   0,   0),	Color::RGB(0,   0,   0),
];
