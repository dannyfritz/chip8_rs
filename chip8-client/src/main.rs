use chip8_core::chip8::keyboard::{HexKey, Keyboard};
use chip8_core::chip8::vram::{VideoSink, HEIGHT, WIDTH};
use chip8_core::chip8::Chip8;
use chip8_core::program::Program;
use fb_now::{Event, FbNow, WindowBuilder, WindowEvent};

fn main() {
    let mut fb = FbNow::new(WindowBuilder::new(), WIDTH as u32, HEIGHT as u32);
    let mut window_open = true;
    let mut video_sink = VideoSink::new();
    let mut chip8 = Chip8::new();
    let program = Program::new("./programs/PONG2");
    chip8.load_program(&program);
    let keyboard = Keyboard::new();
    while window_open {
        chip8.step(&keyboard, &mut video_sink);
        if let Some(sink_buffer) = video_sink.get() {
            let buffer = sink_buffer
                .data
                .iter()
                .flat_map(|p| {
                    if *p {
                        vec![255u8, 255, 255]
                    } else {
                        vec![0u8, 0, 0]
                    }
                })
                .collect();
            fb.update_buffer(buffer);
        }
        fb.events_loop.poll_events(|event| {
            if let Event::WindowEvent { event, .. } = event {
                if let WindowEvent::CloseRequested = event {
                    window_open = false;
                }
            }
        });
    }
}
