//! CPU clock
//! Allows to managed the framerate

use std::collections::VecDeque;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::thread::sleep;

pub struct Clock {
    frame_history:VecDeque<Duration>,
    start: std::time::SystemTime,
}

impl Clock {
    /// Instantiate new clock
    pub fn new() -> Clock{
        Clock{
            frame_history: VecDeque::new(),
            start: SystemTime::now(),
        }
    }

    /// Tick at each frame and wait to reach the target frame rate
    pub fn tick(&mut self, target_frame_rate: u32) {
        let since_the_epoch = self.start.duration_since(UNIX_EPOCH).unwrap();
        self.frame_history.push_back(since_the_epoch);
        if self.frame_history.len() > 10 {
            self.frame_history.pop_front();
            let frame_real_duration = since_the_epoch - *self.frame_history.back().unwrap();
            let frame_duration:u64 = ((1f64 / target_frame_rate as f64) * 1_000_000_000f64) as u64;
            sleep(Duration::from_nanos(frame_duration) - frame_real_duration);
        }
    }

    /// Get current fps
    pub fn get_fps(&self) -> f64 {
        if self.frame_history.len() < 10 {
            return 0f64;
        }
        let seconds_per_10_frames:f64 = (self.frame_history.back().unwrap().as_nanos() - self.frame_history.front().unwrap().as_nanos()) as f64 / 1_000_000_000f64;
        10f64 / (seconds_per_10_frames)
    }
}
