use chip8::{Address, DWord, Word};
use program::Program;

const MEMORY_SIZE: usize = 4096;

pub struct Memory {
    data: [u8; MEMORY_SIZE],
}

impl Memory {
    pub fn new() -> Memory {
        Memory { data: [0; 4096] }
    }
    pub fn load_program(&mut self, program: Program) {
        let data = &mut vec![0; 0x200];
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
