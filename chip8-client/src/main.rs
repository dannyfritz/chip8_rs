extern crate chip8_core;
extern crate minifb;

use std::time;
use std::thread;
use chip8_core::chip8::Chip8;
use chip8_core::chip8::keyboard::{KeyState, Keyboard};
use chip8_core::chip8::vram::{PixelBuffer, VideoSink};
use chip8_core::program::Program;
use minifb::{Key, Scale, Window, WindowOptions};

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

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
    let program = Program::new("../programs/pong.ch8");
    chip8.load_program(program);
    let mut keyboard = Keyboard::new();
    while window.is_open() && !window.is_key_down(Key::Escape) {
        chip8.step(&keyboard, &mut video_sink);
        match video_sink.get() {
            Some(pixel_buffer) => {
                for (i, v) in buffer.iter_mut().enumerate() {
                    *v = if pixel_buffer.0[i] {
                        u32::max_value()
                    } else {
                        u32::min_value()
                    };
                }
                window.update_with_buffer(&buffer).unwrap();
            }
            None => {}
        }
        //TODO: This sleep duration needs to be smarter
        thread::sleep(time::Duration::from_millis(3));
    }
}
