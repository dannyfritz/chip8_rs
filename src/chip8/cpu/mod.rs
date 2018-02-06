mod instruction;
use chip8::Address;
use self::instruction::{Instruction, OpCode};
use chip8::memory::Memory;
use chip8::stack::Stack;
use chip8::vram::Vram;

pub struct Cpu {
    v: [u8; 0xF],
    i: u16,
    delay_timer: u8,
    sound_timer: u8,
    pc: u16,
    stack: Stack,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            v: [0; 0xF],
            i: 0,
            delay_timer: 0,
            sound_timer: 0,
            pc: 0x200,
            stack: Stack::new(),
        }
    }
    pub fn tick(&mut self, memory: &mut Memory, vram: &mut Vram) {
        let data = memory.read_dword(self.pc);
        // println!("pc: 0x{:04X}, op: 0x{:04X}", self.pc, data);
        let instruction = Instruction::new(data);
        match instruction.to_opcode() {
            OpCode::Set(vx, value) => {
                self.v[vx as usize] = value;
            }
            OpCode::Copy(vx, vy) => {
                self.v[vx as usize] = self.v[vy as usize];
            }
            OpCode::Add(vx, value) => {
                self.v[vx as usize].wrapping_add(value);
            }
            OpCode::AddVy(vx, vy) => {
                self.v[vx as usize] += self.v[vy as usize];
            }
            OpCode::SubVx(vx, vy) => {
                self.v[vx as usize] -= self.v[vy as usize];
            }
            OpCode::SubVy(vx, vy) => {
                self.v[vx as usize] = self.v[vy as usize] - self.v[vx as usize];
            }
            OpCode::And(vx, vy) => {
                self.v[vx as usize] &= self.v[vy as usize];
            }
            OpCode::Or(vx, vy) => {
                self.v[vx as usize] |= self.v[vy as usize];
            }
            OpCode::Xor(vx, vy) => {
                self.v[vx as usize] ^= self.v[vy as usize];
            }
            OpCode::ShiftRight(vx, vy) => {
                self.v[vx as usize] = self.v[vy as usize] >> 1;
            }
            OpCode::ShiftLeft(vx, vy) => {
                self.v[vx as usize] = self.v[vy as usize] << 1;
            }
            OpCode::Jmp(address) => {
                self.pc = address - 2;
            }
            OpCode::JmpV0(address) => {
                self.pc = address + self.v[0] as Address - 2;
            }
            OpCode::Call(address) => {
                self.stack.push(self.pc);
                self.pc = address;
                self.pc -= 2;
            }
            OpCode::Return() => {
                let addr = self.stack.pop();
                self.pc = addr;
                self.pc -= 2;
            }
            OpCode::Jeq(vx, value) => {
                if self.v[vx as usize] == value {
                    self.pc += 2;
                }
            }
            OpCode::JeqVy(vx, vy) => {
                if self.v[vx as usize] == self.v[vy as usize] {
                    self.pc += 2;
                }
            }
            OpCode::Jneq(vx, value) => {
                if self.v[vx as usize] != value {
                    self.pc += 2;
                }
            }
            OpCode::JneqVy(vx, vy) => {
                if self.v[vx as usize] != self.v[vy as usize] {
                    self.pc += 2;
                }
            }
            OpCode::SetDelayTimer(vx) => {
                self.delay_timer = self.v[vx as usize];
            }
            OpCode::LdDelayTimer(vx) => {
                self.v[vx as usize] = self.delay_timer;
            }
            OpCode::SetSoundTimer(vx) => {
                if self.v[vx as usize] > 2 {
                    self.sound_timer = self.v[vx as usize];
                }
            }
            OpCode::SetI(value) => {
                self.i = value;
            }
            OpCode::SetIVx(vx) => {
                self.i = self.v[vx as usize] as Address;
            }
            OpCode::ClearScreen() => {
                vram.clear();
            }
            OpCode::DrawSprite(vx, vy, value) => {
                vram.draw_sprite(memory, self.i, vx, vy, value);
            }
            OpCode::Store(vx) => {
                let mut r = 0;
                for addr in self.i..self.i + (vx as u16) {
                    memory.write(addr, self.v[r as usize]);
                    r += 1;
                }
                self.i += r;
            }
            OpCode::Load(vx) => {
                let mut r = 0;
                for addr in self.i..self.i + (vx as u16) {
                    self.v[r as usize] = memory.read(addr);
                    r += 1;
                }
                self.i += r;
            }
            OpCode::BCD(vx) => {
                println!("BCD Not Implemented! 0xFX33");
            }
            OpCode::Font(vx) => {
                println!("Font Not Implemented! 0xFX29");
            }
        }
        self.pc += 2;
    }
}
