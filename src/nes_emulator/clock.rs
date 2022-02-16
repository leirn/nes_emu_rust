//! CPU clock
//! Allows to managed the framerate

use std::collections::VecDeque;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::thread::sleep;

/// Internal clock component, used to cadence the whole execution
pub struct Clock {
    target_frame_duration: Duration,
    frame_history:VecDeque<Duration>,
    start: SystemTime,
}

impl Clock {
    /// Instantiate new clock
    pub fn new(_target_framerate: u32) -> Clock{
        let frame_duration:u64 = (1_000_000_000f64 / _target_framerate as f64) as u64;
        let _target_frame_duration = Duration::from_nanos(frame_duration);
        Clock{
            target_frame_duration: _target_frame_duration,
            frame_history: VecDeque::new(),
            start: SystemTime::now(),
        }
    }

    /// Tick at each frame and wait to reach the target frame rate
    pub fn tick(&mut self) {
        let since_the_epoch = self.start.duration_since(UNIX_EPOCH).unwrap();
        self.frame_history.push_back(since_the_epoch);
        if self.frame_history.len() > 11 {
            self.frame_history.pop_front();
            let frame_real_duration = since_the_epoch - *self.frame_history.back().unwrap();
            sleep(self.target_frame_duration - frame_real_duration);
        }
    }

    /// Get current fps
    pub fn get_fps(&self) -> f64 {
        if self.frame_history.len() < 11 {
            return 0f64;
        }
        let seconds_per_10_frames:f64 = (self.frame_history.back().unwrap().as_micros() - self.frame_history.front().unwrap().as_micros()) as f64 / 1_000_000_000f64;
        10f64 / (seconds_per_10_frames)
    }
}
