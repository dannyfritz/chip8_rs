extern crate chip8_core;
extern crate gl;
extern crate glutin;

use chip8_core::chip8::Chip8;
use chip8_core::chip8::keyboard::{KeyState, Keyboard};
use chip8_core::chip8::vram::{PixelBuffer, VideoSink};
use chip8_core::program::Program;
use glutin::{ElementState, GlContext};

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_title("Hello, world!")
        .with_dimensions(800, 600);
    let context = glutin::ContextBuilder::new().with_vsync(true);
    let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();
    unsafe {
        gl_window.make_current().unwrap();
    }
    let data = &mut [false; 64 * 32];
    let pixel_buffer = PixelBuffer(data, 64);
    let mut video_sink = VideoSink {
        buffer: pixel_buffer,
        is_populated: false,
    };
    let mut chip8 = Chip8::new();
    let tank = Program::new("../programs/tank.ch8");
    chip8.load_program(tank);
    let mut keyboard = Keyboard::new();
    let mut running = true;
    while running {
        events_loop.poll_events(|event| match event {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::Closed => running = false,
                glutin::WindowEvent::KeyboardInput {
                    device_id: _,
                    input,
                } => {
                    keyboard.update_key(KeyState::new(
                        input.scancode,
                        input.state == ElementState::Pressed,
                    ));
                }
                _ => (),
            },
            _ => (),
        });
        chip8.step(&keyboard, &mut video_sink);
        //TODO: draw frame
    }
}
