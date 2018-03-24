extern crate chip8_core;
extern crate minifb;

use std::time;
use std::thread;
use chip8_core::chip8::Chip8;
use chip8_core::chip8::keyboard::{HexKey, Keyboard};
use chip8_core::chip8::vram::{VideoSink, HEIGHT, WIDTH};
use chip8_core::program::Program;
use minifb::{Key, Scale, Window, WindowOptions};

fn main() {
    let mut window = Window::new(
        "Chip8",
        WIDTH,
        HEIGHT,
        WindowOptions {
            borderless: false,
            title: true,
            resize: false,
            scale: Scale::X16,
        },
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });
    let mut buffer: Vec<u32> = vec![0; 64 * 32];
    let mut video_sink = VideoSink::new();
    let mut chip8 = Chip8::new();
    let program = Program::new("../programs/PONG2");
    chip8.load_program(program);
    let mut keyboard = Keyboard::new();
    while window.is_open() && !window.is_key_down(Key::Escape) {
        /* INFO:
            chip8      qwerty
            1 2 3 C    1 2 3 4
            4 5 6 D    q w e r
            7 8 9 E    a s d f
            A 0 B F    z x c v
        */
        keyboard.update_key(HexKey::X0, window.is_key_down(Key::X));
        keyboard.update_key(HexKey::X1, window.is_key_down(Key::Key1));
        keyboard.update_key(HexKey::X2, window.is_key_down(Key::Key2));
        keyboard.update_key(HexKey::X3, window.is_key_down(Key::Key3));
        keyboard.update_key(HexKey::X4, window.is_key_down(Key::Q));
        keyboard.update_key(HexKey::X5, window.is_key_down(Key::W));
        keyboard.update_key(HexKey::X6, window.is_key_down(Key::E));
        keyboard.update_key(HexKey::X7, window.is_key_down(Key::A));
        keyboard.update_key(HexKey::X8, window.is_key_down(Key::S));
        keyboard.update_key(HexKey::X9, window.is_key_down(Key::D));
        keyboard.update_key(HexKey::Xa, window.is_key_down(Key::Z));
        keyboard.update_key(HexKey::Xb, window.is_key_down(Key::C));
        keyboard.update_key(HexKey::Xc, window.is_key_down(Key::Key4));
        keyboard.update_key(HexKey::Xd, window.is_key_down(Key::R));
        keyboard.update_key(HexKey::Xe, window.is_key_down(Key::F));
        keyboard.update_key(HexKey::Xf, window.is_key_down(Key::V));
        //INFO: every iteration should be 1/60th of a second
        chip8.step(&keyboard, &mut video_sink);
        if let Some(sink_buffer) = video_sink.get() {
            for (i, v) in buffer.iter_mut().enumerate() {
                *v = if sink_buffer.data[i] {
                    u32::max_value()
                } else {
                    u32::min_value()
                };
            }
            window.update_with_buffer(&buffer).unwrap();
        }
        //TODO: This sleep duration needs to be smarter
        //time::Duration.from_millis(17) and time::Instant()
        thread::sleep(time::Duration::from_millis(3));
    }
}
