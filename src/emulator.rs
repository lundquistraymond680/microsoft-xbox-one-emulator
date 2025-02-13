use std::vec::Vec;

pub struct Emulator {
    pub width: i32,
    pub height: i32,
    frame_data: Vec<u8>,
}

impl Emulator {
    pub fn new() -> Self {
        Self {
            width: 800,
            height: 600,
            frame_data: vec![0; 800 * 600 * 3],
        }
    }

    pub fn update(&mut self) {
        // Update game state and render frame
    }

    pub fn get_frame_data(&self) -> &[u8] {
        &self.frame_data
    }

    pub fn handle_input(&mut self, key: glfw::Key) {
        // Handle input from the user
    }
}