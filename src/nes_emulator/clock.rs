//! CPU clock
//! Allows to managed the framerate

pub struct Clock {
    frame_history:Vec<u64>,
}

impl Clock {
    /// Instantiate new clock
    pub fn new() -> Clock{
        Clock{
            frame_history: Vec::with_capacity(10),
        }
    }

    /// Tick at each frame and wait to reach the target frame rate
    pub fn tick(&mut self, target_frame_rate: u32) {

    }

    /// Get current fps
    pub fn get_fps(&self) -> f32 {
        0.0
    }
}
