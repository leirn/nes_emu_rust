use crate::components;

pub struct Screen {
    scale: u32,
    video_subsystem: sdl2::VideoSubsystem,
    //window: sdl2::video::Window,
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
}


impl Screen {
    pub fn new() -> Screen {
        sdl2::init();
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

    pub fn start(&self) {

    }

    pub fn next(&self) {

    }
}