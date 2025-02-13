use glfw::Window;

pub fn render(window: &Window, emulator: &super::emulator::Emulator) {
    let frame_data = emulator.get_frame_data();
    unsafe {
        gl::Clear(gl::COLOR_BUFFER_BIT);
        gl::DrawPixels(emulator.width, emulator.height, gl::RGB, gl::UNSIGNED_BYTE, frame_data.as_ptr() as *const _);
    }
}