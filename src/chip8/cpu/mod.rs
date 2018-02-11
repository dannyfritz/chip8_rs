mod instruction;
use chip8::Address;
use self::instruction::{Instruction, OpCode};
use chip8::memory::Memory;
use chip8::stack::Stack;
use chip8::vram::Vram;
use rand::{thread_rng, Rng};

pub struct Cpu {
    v: [u8; 0x10],
    i: u16,
    delay_timer: u8,
    sound_timer: u8,
    pc: u16,
    stack: Stack,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            v: [0; 0x10],
            i: 0,
            delay_timer: 0,
            sound_timer: 0,
            pc: 0x200,
            stack: Stack::new(),
        }
    }
    pub fn tick(&mut self, memory: &mut Memory, vram: &mut Vram) {
        let data = memory.read_dword(self.pc);
        let instruction = Instruction::new(data);
        //TODO: decrement timer correctly
        self.delay_timer = self.delay_timer.wrapping_sub(1);
        self.sound_timer = self.sound_timer.wrapping_sub(1);
        //TODO: Keyboard
        let opcode = instruction.to_opcode();
        println!("{:?}", opcode);
        match opcode {
            OpCode::Set(vx, value) => {
                self.v[vx as usize] = value;
            }
            OpCode::Copy(vx, vy) => {
                self.v[vx as usize] = self.v[vy as usize];
            }
            OpCode::Add(vx, value) => {
                let (result, overflow) = self.v[vx as usize].overflowing_add(value);
                self.v[vx as usize] = result;
                self.v[0xF as usize] = if overflow { 1 } else { 0 };
            }
            OpCode::AddVy(vx, vy) => {
                let value_x = self.v[vx as usize];
                let value_y = self.v[vy as usize];
                let (result, overflow) = value_x.overflowing_add(value_y);
                self.v[vx as usize] = result;
                self.v[0xF as usize] = if overflow { 1 } else { 0 };
            }
            OpCode::SubVx(vx, vy) => {
                let value_x = self.v[vx as usize];
                let value_y = self.v[vy as usize];
                self.v[0xF as usize] = if value_y > value_x { 1 } else { 0 };
                self.v[vx as usize] -= value_y;
            }
            OpCode::SubVy(vx, vy) => {
                let value_x = self.v[vx as usize];
                let value_y = self.v[vy as usize];
                self.v[0xF as usize] = if value_x > value_y { 1 } else { 0 };
                self.v[vx as usize] = value_y - value_x;
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
                let value = self.v[vy as usize];
                self.v[0xF as usize] = value & 0x1;
                self.v[vx as usize] = value >> 1;
            }
            OpCode::ShiftLeft(vx, vy) => {
                let value = self.v[vy as usize];
                self.v[0xF as usize] = value & 0b1000_0000 >> 7;
                self.v[vx as usize] = value << 1;
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
                // self.pc -= 2;
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
                let value_x = self.v[vx as usize];
                self.v[0xF as usize] = if self.i + value_x as u16 > 0xFFF {
                    1
                } else {
                    0
                };
                //TODO, wrap around 0xFFF
                self.i += self.v[vx as usize] as Address;
            }
            OpCode::ClearScreen() => {
                vram.clear();
            }
            OpCode::DrawSprite(vx, vy, value) => {
                self.v[0xF as usize] = if vram.draw_sprite(memory, self.i, vx, vy, value) {
                    1
                } else {
                    0
                };
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
                //TODO: VF
            }
            OpCode::Random(vx, mask) => {
                let mut rng = thread_rng();
                let random = rng.gen::<u8>();
                self.v[vx as usize] = random & mask;
            }
        }
        self.pc += 2;
    }
}
