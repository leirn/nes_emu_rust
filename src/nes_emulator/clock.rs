//! CPU clock
//! Allows to managed the framerate

use std::collections::VecDeque;
use std::time::{SystemTime, UNIX_EPOCH};
use std::time::Duration;

pub struct Clock {
    frame_history:VecDeque<u64>,
    start: u64,
}

impl Clock {
    /// Instantiate new clock
    pub fn new() -> Clock{
        Clock{
            frame_history: Vec::with_capacity(10),
            start: SystemTime::now(),
        }
    }

    /// Tick at each frame and wait to reach the target frame rate
    pub fn tick(&mut self, target_frame_rate: u32) {
        let since_the_epoch = self.start.duration_since(UNIX_EPOCH).unwrap();
        self.frame_history.push_back(since_the_epoch.from_nanos());
        if self.frame_history.len() > 10 {
            self.frame_history.pop_front();
        }
    }

    /// Get current fps
    pub fn get_fps(&self) -> f64 {
        (self.frame_history.back() as f64 - self.frame_history.front() as f64) / 10f64
    }
}
