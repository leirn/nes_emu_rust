struct Clock {
    frame_history:Vec<u64>,
}

impl Clock {
    pub fn new() -> Clock{
        Clock{
            frame_history: Vec::with_capacity(10),
        }
    }

    pub fn tick(target_frame_rate as u32) {

    }

    pub fn get_fps() -> u32 {
        0
    }
}