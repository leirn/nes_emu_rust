//! CPU clock
//! Allows to managed the framerate

use std::collections::VecDeque;
use std::time::{Duration, SystemTime};
use std::thread::sleep;

/// Internal clock component, used to cadence the whole execution
pub struct Clock {
    target_frame_duration: Duration,
    frame_history:VecDeque<Duration>,
}

impl Clock {
    /// Instantiate new clock
    pub fn new(_target_framerate: u32) -> Clock{
        let _target_frame_duration:Duration = Duration::from_nanos(1_000_000_000f64 / _target_framerate as f64);
        Clock{
            target_frame_duration: _target_frame_duration,
            frame_history: VecDeque::from([SystemTime::now()]),
        }
    }

    /// Tick at each frame and wait to reach the target frame rate
    pub fn tick(&mut self) {
        let now = SystemTime::now();
        sleep(self.target_frame_duration.saturating_sub((now.duration_since(&self.frame_history.back()))));
        self.frame_history.push_back(now);
        if self.frame_history.len() > 11 {
            self.frame_history.pop_front();
        }
    }

    /// Get current fps
    pub fn get_fps(&self) -> f64 {
        if self.frame_history.len() < 11 {
            return 0f64;
        }
        println!("{}", self.frame_history.back().unwrap().as_micros());
        println!("{}", self.frame_history.front().unwrap().as_micros());
        let seconds_per_10_frames:f64 = (self.frame_history.back().unwrap().as_micros() - self.frame_history.front().unwrap().as_micros()) as f64 / 1_000_000_000f64;
        10f64 / (seconds_per_10_frames)
    }
}
