//! Screen component

use sdl2::pixels::{Color, Palette, PixelFormatEnum};
use sdl2::render::Canvas;
use sdl2::surface::Surface;
use sdl2::video::Window;
use std::cell::RefCell;
use std::rc::Rc;
pub struct Screen<'a> {
    scale: u32,
    canvas: Canvas<Window>,
    surface: Surface<'a>,
    g_scaling_mode: ScalingMode,
}

const SCREEN_WIDTH: u32 = 256;
const SCREEN_HEIGHT: u32 = 240;

#[derive(PartialEq, Clone, Copy)]
enum ScalingMode {
    ScalingModeAspectFit,
    ScalingModeAspectFill,
    ScalingModeIntegerFactor,
    ScalingModeFullscreen,
    ScalingModeAspectCorrect,
    ScalingModeCount,
}

pub const TEXTURE_ASPECT_RATIO: f64 = SCREEN_WIDTH as f64 / SCREEN_HEIGHT as f64;

impl Screen<'_> {
    /// Instantiate Screen component
    pub fn new(sdl_context: Rc<RefCell<sdl2::Sdl>>) -> Screen<'static> {
        let scale = 3;
        let _video_subsystem = sdl_context.borrow_mut().video().unwrap();
        let _window = _video_subsystem
            .window("Window", SCREEN_WIDTH, SCREEN_HEIGHT)
            .opengl() // this line DOES NOT enable opengl, but allows you to create/get an OpenGL context from your window.
            .build()
            .unwrap();
        let mut _canvas = _window
            .into_canvas()
            .index(Screen::find_sdl_gl_driver().unwrap())
            .build()
            .unwrap();

        let mut _surface =
            Surface::new(SCREEN_WIDTH, SCREEN_HEIGHT, PixelFormatEnum::Index8).unwrap();
        let palette = Palette::with_colors(&PALETTE).unwrap();
        _surface.set_palette(&palette).unwrap();

        _canvas
            .window_mut()
            .set_size(
                (3 * SCREEN_WIDTH).try_into().unwrap(),
                (3 * SCREEN_HEIGHT).try_into().unwrap(),
            )
            .unwrap();

        Screen {
            scale: scale,
            canvas: _canvas,
            surface: _surface,
            g_scaling_mode: ScalingMode::ScalingModeAspectFit,
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
        self.canvas
            .set_draw_color(sdl2::pixels::Color::RGB(0, 0, 0));
        // fills the canvas with the color we set in `set_draw_color`.
        self.canvas.clear();

        // change the color of our drawing with a gold-color ...
        self.canvas.set_draw_color(PALETTE[36]);
        // A draw a rectangle which almost fills our window with it !
        if let Err(e) = self.canvas.fill_rect(sdl2::rect::Rect::new(
            10,
            10,
            256 * self.scale - 20,
            240 * self.scale - 20,
        )) {
            println!("{:?}", e);
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
    /*pub fn update_pixel(&mut self, x: u8, y: u8, color_index: u8) {
        let color_index = color_index % 64;
        self.canvas.set_draw_color(PALETTE[color_index as usize]);
        if let Err(e) = self.canvas.fill_rect(sdl2::rect::Rect::new(
            self.scale as i32 * x as i32,
            self.scale as i32 * y as i32,
            self.scale,
            self.scale,
        )) {
            println!("{:?}", e);
        }
    }*/

    pub fn update_pixel(&mut self, x: u8, y: u8, color: u8) {
        let address = x as usize + y as usize * SCREEN_WIDTH as usize;
        self.surface.without_lock_mut().unwrap()[address] = color;
    }

    /// Refresh the windows with the buffered canvas
    pub fn present(&mut self) {
        self.canvas.clear();
        let creator = self.canvas.texture_creator();
        let texture = self.surface.as_texture(&creator).unwrap();

        self.canvas
            .copy(&texture, None, self.canvas.viewport())
            .unwrap();

        self.canvas.present();
    }

    pub fn update_window_viewport(&mut self) {
        let (window_height, window_width) = self.canvas.window().size();

        // If the scaling mode is fullscreen, use the window size
        if self.g_scaling_mode == ScalingMode::ScalingModeFullscreen {
            self.canvas.viewport().set_x(0);
            self.canvas.viewport().set_y(0);
            self.canvas.viewport().set_width(window_width);
            self.canvas.viewport().set_height(window_height);
            return;
        }

        let texture_aspect_ratio;
        if self.g_scaling_mode == ScalingMode::ScalingModeAspectCorrect {
            texture_aspect_ratio = 4 as f64 / 3 as f64;
        } else {
            texture_aspect_ratio = TEXTURE_ASPECT_RATIO;
        }

        let mut max_viewport_width = window_width;
        let mut max_viewport_height = window_height;

        // For "integer factor" scaling, pick the highest integer factor that fits into the window
        if self.g_scaling_mode == ScalingMode::ScalingModeIntegerFactor {
            max_viewport_width =
                ((window_width as f64 / SCREEN_WIDTH as f64) * SCREEN_WIDTH as f64) as u32;
            max_viewport_height =
                ((window_height as f64 / SCREEN_HEIGHT as f64) * SCREEN_HEIGHT as f64) as u32;
        }

        // If the resulting viewport is too small, do proportional scaling according to the window size
        if max_viewport_width == 0 {
            max_viewport_width = window_width;
        }
        if max_viewport_height == 0 {
            max_viewport_height = window_height;
        }

        let screen_aspect_ratio = window_width as f64 / window_height as f64;
        let mut should_preserve_width = texture_aspect_ratio > screen_aspect_ratio;

        // The only difference between aspect fill and fit is that fit will leave black bars
        // and fill will crop the image.
        // TODO : does not seem to work, always in fit mode without preserving ratio
        if self.g_scaling_mode == ScalingMode::ScalingModeAspectFill {
            should_preserve_width = !should_preserve_width;
        }

        if should_preserve_width {
            self.canvas
                .viewport()
                .set_x((window_width as i32 - max_viewport_width as i32) >> 1);
            self.canvas.viewport().set_width(max_viewport_width);
            let viewport_width = self.canvas.viewport().width();
            self.canvas
                .viewport()
                .set_height(viewport_width / texture_aspect_ratio as u32);
            let viewport_height = self.canvas.viewport().height();
            self.canvas
                .viewport()
                .set_y((window_height as i32 - viewport_height as i32) >> 1);
        } else {
            self.canvas
                .viewport()
                .set_y((window_height as i32 - max_viewport_height as i32) >> 1);
            self.canvas.viewport().set_height(max_viewport_height);
            let viewport_height = self.canvas.viewport().height();
            self.canvas
                .viewport()
                .set_width(viewport_height * texture_aspect_ratio as u32);
            let viewport_width = self.canvas.viewport().width();
            self.canvas
                .viewport()
                .set_x((window_width as i32 - viewport_width as i32) >> 1);
        }

        self.canvas.present();
    }

    pub fn get_scaling_mode(&self) -> ScalingMode {
        self.g_scaling_mode
    }

    pub fn set_scaling_mode(&mut self, mode: ScalingMode) {
        self.g_scaling_mode = mode;
        self.update_window_viewport();
    }
}

/// NES color palette
const PALETTE: [Color; 64] = [
    Color::RGB(84, 84, 84),
    Color::RGB(0, 30, 116),
    Color::RGB(8, 16, 144),
    Color::RGB(48, 0, 136),
    Color::RGB(68, 0, 100),
    Color::RGB(92, 0, 48),
    Color::RGB(84, 4, 0),
    Color::RGB(60, 24, 0),
    Color::RGB(32, 42, 0),
    Color::RGB(8, 58, 0),
    Color::RGB(0, 64, 0),
    Color::RGB(0, 60, 0),
    Color::RGB(0, 50, 60),
    Color::RGB(0, 0, 0),
    Color::RGB(0, 0, 0),
    Color::RGB(0, 0, 0),
    Color::RGB(152, 150, 152),
    Color::RGB(8, 76, 196),
    Color::RGB(48, 50, 236),
    Color::RGB(92, 30, 228),
    Color::RGB(136, 20, 176),
    Color::RGB(160, 20, 100),
    Color::RGB(152, 34, 32),
    Color::RGB(120, 60, 0),
    Color::RGB(84, 90, 0),
    Color::RGB(40, 114, 0),
    Color::RGB(8, 124, 0),
    Color::RGB(0, 118, 40),
    Color::RGB(0, 102, 120),
    Color::RGB(0, 0, 0),
    Color::RGB(0, 0, 0),
    Color::RGB(0, 0, 0),
    Color::RGB(236, 238, 236),
    Color::RGB(76, 154, 236),
    Color::RGB(120, 124, 236),
    Color::RGB(176, 98, 236),
    Color::RGB(228, 84, 236),
    Color::RGB(236, 88, 180),
    Color::RGB(236, 106, 100),
    Color::RGB(212, 136, 32),
    Color::RGB(160, 170, 0),
    Color::RGB(116, 196, 0),
    Color::RGB(76, 208, 32),
    Color::RGB(56, 204, 108),
    Color::RGB(56, 180, 204),
    Color::RGB(60, 60, 60),
    Color::RGB(0, 0, 0),
    Color::RGB(0, 0, 0),
    Color::RGB(236, 238, 236),
    Color::RGB(168, 204, 236),
    Color::RGB(188, 188, 236),
    Color::RGB(212, 178, 236),
    Color::RGB(236, 174, 236),
    Color::RGB(236, 174, 212),
    Color::RGB(236, 180, 176),
    Color::RGB(228, 196, 144),
    Color::RGB(204, 210, 120),
    Color::RGB(180, 222, 120),
    Color::RGB(168, 226, 144),
    Color::RGB(152, 226, 180),
    Color::RGB(160, 214, 228),
    Color::RGB(160, 162, 160),
    Color::RGB(0, 0, 0),
    Color::RGB(0, 0, 0),
];
