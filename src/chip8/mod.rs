mod cpu;
mod memory;
mod stack;
mod vram;

use self::cpu::Cpu;
use self::memory::Memory;
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

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 {
            cpu: Cpu::new(),
            memory: Memory::new(),
            vram: Vram::new(),
        }
    }
    pub fn load_program(&mut self, program: Program) {
        self.memory.load_program(program);
    }
    pub fn run(&mut self) {
        loop {
            self.cpu.tick(&mut self.memory, &mut self.vram);
        }
    }
}
