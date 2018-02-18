mod cpu;
mod memory;
mod stack;
mod vram;
mod keyboard;

use self::cpu::Cpu;
use self::memory::Memory;
use self::vram::Vram;
use self::keyboard::Keyboard;
use program::Program;
use glutin;
use glutin::GlContext;

pub type Address = u16;
pub type Word = u8;
pub type DWord = u16;
pub type Register = u8;

pub struct Chip8 {
    cpu: Cpu,
    memory: Memory,
    vram: Vram,
    keyboard: Keyboard,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 {
            cpu: Cpu::new(),
            memory: Memory::new(),
            vram: Vram::new(),
            keyboard: Keyboard::new(),
        }
    }
    pub fn load_program(&mut self, program: Program) {
        self.memory.load_program(program);
    }
    pub fn run(&mut self) {
        let mut events_loop = glutin::EventsLoop::new();
        let window = glutin::WindowBuilder::new()
            .with_title("Hello, world!")
            .with_dimensions(200, 200);
        let context = glutin::ContextBuilder::new().with_vsync(true);
        let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

        unsafe {
            gl_window.make_current().unwrap();
        }
        let mut running = true;
        while running {
            events_loop.poll_events(|event| match event {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::Closed => running = false,
                    glutin::WindowEvent::KeyboardInput { device_id: _, input } => {
                        self.keyboard.update_key(input);
                    }
                    _ => (),
                },
                _ => (),
            });
            self.cpu.tick(&mut self.memory, &mut self.vram, &mut self.keyboard);
            //TODO: draw frame
        }
    }
}
