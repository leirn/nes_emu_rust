#[cfg(test)]
mod tests {
    #[test]
    fn clock_tick() {
        //! This test checks if a tick duration last for the right duration with a 5% tolerance
        let mut clock  = crate::nes_emulator::clock::Clock::new(60);
        const tolerance_margin = 0.05f64;
        const frame_duration_nanos = 1_000_000_000f64 / 60f64;
        let tolerance = std::time::Duration::from_nanos(frame_duration_nanos * tolerance_margin); // 5% tolerance compare to 1/60th seconds
        let expected_duration = std::time::Duration::from_nanos(frame_duration_nanos);
        let upper = expected_duration + tolerance;
        let lower = expected_duration - tolerance;
        let now = std::time::SystemTime::now();
        self.tick();
        let elapsed = now.elapsed();
        assert!(now.elapsed() <= upper);
        assert!(now.elapsed() >= lower);
    }

    #[test]
    fn clock_fps() {
        //! This test checks if the clock fps measurement give the right duration with 2 fps margin
        const target_framerate = 60u32;
        const target_framerate_margin = 60u32;
        const upper = (target_framerate + target_framerate_margin) as f64;
        const lower = (target_framerate - target_framerate_margin) as f64;
        let mut clock  = crate::nes_emulator::clock::Clock::new(target_framerate);
        for _i in 1..15 {
            clock.tick();
        }
        let fps = clock.get_fps();
        assert!(fps < upper);
        assert!(fps > lower);
    }
}
