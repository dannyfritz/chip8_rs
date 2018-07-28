use chip8_core::chip8::keyboard::{HexKey, Keyboard};
use chip8_core::chip8::vram::{VideoSink, HEIGHT, WIDTH};
use chip8_core::chip8::Chip8;
use chip8_core::program::Program;
use fb_now::glutin::{
    ElementState, Event, KeyboardInput, VirtualKeyCode, WindowBuilder, WindowEvent,
};
use fb_now::FbNow;
use std::env;
use std::thread;
use std::time::Duration;

macro_rules! keyboard_update {
    ($event:ident, $keyboard:ident, $chip8_keycode:path, $keycode:path) => {
        if let WindowEvent::KeyboardInput {
            input:
                KeyboardInput {
                    virtual_keycode: Some($keycode),
                    state,
                    ..
                },
            ..
        } = $event
        {
            $keyboard.update_key($chip8_keycode, state == ElementState::Pressed);
        }
    };
}

fn main() {
    if env::args().len() != 2 {
        eprintln!("chip8-client [CHIP8 FILE]");
        return;
    }
    let program_file = env::args().nth(1).unwrap();
    let mut fb = FbNow::new(WindowBuilder::new(), WIDTH as u32, HEIGHT as u32);
    let mut window_open = true;
    let mut video_sink = VideoSink::new();
    let mut chip8 = Chip8::new();
    let program = Program::new(&program_file);
    chip8.load_program(&program);
    let mut keyboard = Keyboard::new();
    while window_open {
        fb.events_loop.poll_events(|event| {
            if let Event::WindowEvent { event, .. } = event {
                if let WindowEvent::CloseRequested = event {
                    window_open = false;
                }
                /* INFO:
                    chip8      qwerty
                    1 2 3 C    1 2 3 4
                    4 5 6 D    q w e r
                    7 8 9 E    a s d f
                    A 0 B F    z x c v
                */
                keyboard_update!(event, keyboard, HexKey::X1, VirtualKeyCode::Key1);
                keyboard_update!(event, keyboard, HexKey::X2, VirtualKeyCode::Key2);
                keyboard_update!(event, keyboard, HexKey::X3, VirtualKeyCode::Key3);
                keyboard_update!(event, keyboard, HexKey::Xc, VirtualKeyCode::Key4);
                keyboard_update!(event, keyboard, HexKey::X4, VirtualKeyCode::Q);
                keyboard_update!(event, keyboard, HexKey::X5, VirtualKeyCode::W);
                keyboard_update!(event, keyboard, HexKey::X6, VirtualKeyCode::E);
                keyboard_update!(event, keyboard, HexKey::Xd, VirtualKeyCode::R);
                keyboard_update!(event, keyboard, HexKey::X7, VirtualKeyCode::A);
                keyboard_update!(event, keyboard, HexKey::X8, VirtualKeyCode::S);
                keyboard_update!(event, keyboard, HexKey::X9, VirtualKeyCode::D);
                keyboard_update!(event, keyboard, HexKey::Xe, VirtualKeyCode::F);
                keyboard_update!(event, keyboard, HexKey::Xa, VirtualKeyCode::Z);
                keyboard_update!(event, keyboard, HexKey::X0, VirtualKeyCode::X);
                keyboard_update!(event, keyboard, HexKey::Xb, VirtualKeyCode::C);
                keyboard_update!(event, keyboard, HexKey::Xf, VirtualKeyCode::V);
            }
        });
        chip8.step(&keyboard, &mut video_sink);
        if let Some(sink_buffer) = video_sink.get() {
            let buffer = sink_buffer
                .data
                .iter()
                .flat_map(|p| {
                    if *p {
                        vec![10u8, 250, 10]
                    } else {
                        vec![10u8, 10, 10]
                    }
                })
                .collect();
            fb.update_buffer(buffer);
            // TODO: This needs to be smarter
            thread::sleep(Duration::from_millis(3))
        }
    }
}
