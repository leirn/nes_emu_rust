//! CPU clock
//! Allows to managed the framerate

use std::collections::VecDeque;
use std::time::{Duration, SystemTime};
use std::thread::sleep;

/// Internal clock component, used to cadence the whole execution
pub struct Clock {
    target_frame_duration: Duration,
    frame_history:VecDeque<SystemTime>,
}

impl Clock {
    /// Instantiate new clock
    pub fn new(_target_framerate: u32) -> Clock{
        let _target_frame_duration:Duration = Duration::from_nanos((1_000_000_000f64 / _target_framerate as f64) as u64);
        Clock{
            target_frame_duration: _target_frame_duration,
            frame_history: VecDeque::from([SystemTime::now()]),
        }
    }

    /// Tick at each frame and wait to reach the target frame rate
    pub fn tick(&mut self) {
        let now = SystemTime::now();
        sleep(self.target_frame_duration.saturating_sub(now.duration_since(*self.frame_history.back().unwrap()).unwrap()));
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
        let front = *self.frame_history.front().unwrap();
        let back = *self.frame_history.back().unwrap();
        let duration_10_frames = back.duration_since(front).unwrap();
        let fps:f64 = 10_000_000.0 / (duration_10_frames.as_micros()) as f64;
        fps
    }
}

#[cfg(test)]
mod tests {
    use super::Clock;

    #[test]
    fn clock_tick() {
        //! This test checks if a tick duration last for the right duration with a 10% tolerance
        const TARGET_FRAMERATE: u32 = 6u32;
        let mut clock  = Clock::new(TARGET_FRAMERATE);
        const TOLERANCE_MARGIN: f64 = 0.1f64;
        const FRAME_DURATION_NANOS: f64 = 1_000_000_000f64 / (TARGET_FRAMERATE as f64);
        let tolerance = std::time::Duration::from_nanos((FRAME_DURATION_NANOS * TOLERANCE_MARGIN) as u64); // 5% tolerance compare to 1/60th seconds
        let expected_duration = std::time::Duration::from_nanos(FRAME_DURATION_NANOS as u64);
        let upper = expected_duration + tolerance;
        let lower = expected_duration - tolerance;
        let now = std::time::SystemTime::now();
        clock.tick();
        let elapsed = now.elapsed().unwrap();
        assert!(elapsed <= upper);
        assert!(elapsed >= lower);
    }

    #[test]
    fn clock_fps() {
        //! This test checks if the clock fps measurement give the right duration with 6 fps margin
        const TARGET_FRAMERATE: u32 = 60u32;
        const TARGET_FRAMERATE_MARGIN: u32 = 6u32;
        const UPPER: f64 = (TARGET_FRAMERATE + TARGET_FRAMERATE_MARGIN) as f64;
        const LOWER: f64 = (TARGET_FRAMERATE - TARGET_FRAMERATE_MARGIN) as f64;
        let mut clock  = crate::nes_emulator::clock::Clock::new(TARGET_FRAMERATE);
        for _i in 1..15 {
            clock.tick();
        }
        let fps = clock.get_fps();
        assert!(fps < UPPER);
        assert!(fps > LOWER);
    }
}
