#[cfg(test)]
mod tests {
    #[test]
    fn clock_tick() {
        //! This test checks if a tick duration last for the right duration with a 5% tolerance
        let mut clock  = crate::nes_emulator::clock::Clock::new(60);
        const TOLERANCE_MARGIN: f64 = 0.05f64;
        const FRAME_DURATION_NANOS: f64 = 1_000_000_000f64 / 60f64;
        let tolerance: u64 = std::time::Duration::from_nanos((FRAME_DURATION_NANOS * TOLERANCE_MARGIN) as u64); // 5% tolerance compare to 1/60th seconds
        let expected_duration: u64 = std::time::Duration::from_nanos(FRAME_DURATION_NANOS as u64);
        let upper: u64 = expected_duration + tolerance;
        let lower: u64 = expected_duration - tolerance;
        let now = std::time::SystemTime::now();
        self.tick();
        let elapsed = now.elapsed();
        assert!(now.elapsed() <= upper);
        assert!(now.elapsed() >= lower);
    }

    #[test]
    fn clock_fps() {
        //! This test checks if the clock fps measurement give the right duration with 2 fps margin
        const TARGET_FRAMERATE: u32 = 60u32;
        const TARGET_FRAMERATE_MARGIN: u32 = 60u32;
        const UPPER: f64 = (TARGET_FRAMERATE + TARGET_FRAMERATE_MARGIN) as f64;
        const LOWER: f64 = (TARGET_FRAMERATE - TARGET_FRAMERATE_MARGIN) as f64;
        let mut clock  = crate::nes_emulator::clock::Clock::new(target_framerate);
        for _i in 1..15 {
            clock.tick();
        }
        let fps = clock.get_fps();
        assert!(fps < UPPER);
        assert!(fps > LOWER);
    }
}
