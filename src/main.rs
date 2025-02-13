use glfw::{Action, Context, Key};
use std::ffi::CString;
use std::ptr;
use std::sync::mpsc::Receiver;

mod graphics;
mod input;
mod emulator;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    let (mut window, events) = glfw.create_window(800, 600, "Xbox One Emulator", glfw::WindowMode::Windowed).unwrap();
    window.make_current();
    window.set_key_polling(true);

    let mut emulator = emulator::Emulator::new();

    while !window.should_close() {
        emulator.update();
        graphics::render(&window, &emulator);
        window.swap_buffers();
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
                _ => input::handle_event(&event, &mut emulator),
            }
        }
    }
}