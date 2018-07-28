pub mod audio;
mod cpu;
pub mod keyboard;
mod memory;
mod stack;
pub mod vram;

use self::audio::AudioSink;
use self::cpu::Cpu;
use self::keyboard::Keyboard;
use self::memory::Memory;
use self::vram::VideoSink;
use self::vram::Vram;
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
    pub fn step(
        &mut self,
        keyboard: &Keyboard,
        video_sink: &mut VideoSink,
        audio_sink: &mut AudioSink,
    ) {
        self.cpu.tick(
            &mut self.memory,
            &mut self.vram,
            keyboard,
            video_sink,
            audio_sink,
        );
    }
}
