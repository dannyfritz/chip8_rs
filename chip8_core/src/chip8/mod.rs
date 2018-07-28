mod cpu;
mod memory;
mod stack;
pub mod vram;
pub mod keyboard;

use self::cpu::Cpu;
use self::memory::Memory;
use self::vram::Vram;
use self::keyboard::Keyboard;
use self::vram::VideoSink;
use program::Program;

pub type Address = u16;
pub type Word = u8;
pub type DWord = u16;
pub type Register = u8;

pub struct Chip8 {
    cpu: Cpu,
    memory: Memory,
    vram: Vram,
}

impl Default for Chip8 {
    fn default() -> Self {
        Chip8 {
            cpu: Cpu::new(),
            memory: Memory::new(),
            vram: Vram::new(),
        }
    }
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Self::default()
    }
    pub fn load_program(&mut self, program: &Program) {
        self.memory.load_program(&program);
    }
    pub fn step(&mut self, keyboard: &Keyboard, mut video_sink: &mut VideoSink) {
        self.cpu
            .tick(&mut self.memory, &mut self.vram, keyboard, &mut video_sink);
    }
}
