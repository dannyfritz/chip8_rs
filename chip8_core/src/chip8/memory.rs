use chip8::{Address, DWord, Word};
use program::Program;

const MEMORY_SIZE: usize = 4096;

pub struct Memory {
    data: [u8; MEMORY_SIZE],
}

impl Default for Memory {
    fn default() -> Self {
        Memory { data: [0; 4096] }
    }
}

impl Memory {
    pub fn new() -> Memory {
        Self::default()
    }
    pub fn load_program(&mut self, program: &Program) {
        let data: &mut Vec<u8> = &mut vec![
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xf0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];
        data.append(&mut vec![0; 0x1B0]);
        data.append(&mut program.data.clone());
        for (index, &byte) in data.iter().enumerate() {
            self.data[index] = byte;
        }
    }
    pub fn read(&self, address: Address) -> Word {
        if address as usize > MEMORY_SIZE {
            panic!(format!("Cannot access memory at {}!", address));
        }
        self.data[address as usize]
    }
    pub fn read_dword(&self, address: Address) -> DWord {
        let high = self.read(address);
        let low = self.read(address + 1);
        (high as u16) << 8 | (low as u16)
    }
    pub fn write(&mut self, address: Address, value: u8) {
        self.data[address as usize] = value;
    }
}
