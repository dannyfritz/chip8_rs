mod instruction;
use self::instruction::{Instruction, OpCode};
use chip8::keyboard::{HexKey, Keyboard};
use chip8::memory::Memory;
use chip8::stack::Stack;
use chip8::vram::{VideoSink, Vram};
use chip8::Address;
use chip8::DWord;
use rand::{thread_rng, Rng};
use std::fmt;

pub struct Cpu {
    v: [u8; 0x10],
    i: u16,
    delay_timer: u8,
    sound_timer: u8,
    pc: u16,
    stack: Stack,
}

impl fmt::Debug for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[pc] {:04x}", self.pc)?;
        write!(f, " [i] {:04x}", self.i)?;
        write!(f, " [v0]")?;
        write!(f, " {:02x}", self.v[0])?;
        write!(f, " {:02x}", self.v[1])?;
        write!(f, " {:02x}", self.v[2])?;
        write!(f, " {:02x}", self.v[3])?;
        write!(f, " [v4]")?;
        write!(f, " {:02x}", self.v[4])?;
        write!(f, " {:02x}", self.v[5])?;
        write!(f, " {:02x}", self.v[6])?;
        write!(f, " {:02x}", self.v[7])?;
        write!(f, " [v8]")?;
        write!(f, " {:02x}", self.v[8])?;
        write!(f, " {:02x}", self.v[9])?;
        write!(f, " [va]")?;
        write!(f, " {:02x}", self.v[10])?;
        write!(f, " {:02x}", self.v[11])?;
        write!(f, " {:02x}", self.v[12])?;
        write!(f, " {:02x}", self.v[13])?;
        write!(f, " {:02x}", self.v[14])?;
        write!(f, " [vf] {:02x}", self.v[15])?;
        write!(f, " [dt] {:02x}", self.delay_timer)?;
        write!(f, " [st] {:02x}", self.sound_timer)?;
        write!(f, "")
    }
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
    pub fn tick(
        &mut self,
        memory: &mut Memory,
        vram: &mut Vram,
        keyboard: &Keyboard,
        video_sink: &mut VideoSink,
    ) {
        // println!(" ? {:?}", self);
        // println!(" ? {:?}", self.stack);
        let data = self.fetch(memory);
        let opcode = self.decode(data);
        // println!("> {:?}", opcode);
        self.execute(&opcode, memory, vram, keyboard, video_sink);
        self.delay_timer = self.delay_timer.saturating_sub(1);
        self.sound_timer = self.sound_timer.saturating_sub(1);
    }
    pub fn fetch(&self, memory: &Memory) -> DWord {
        memory.read_dword(self.pc)
    }
    pub fn decode(&self, data: DWord) -> OpCode {
        let instruction = Instruction::new(data);
        instruction.decode()
    }
    pub fn execute(
        &mut self,
        opcode: &OpCode,
        memory: &mut Memory,
        vram: &mut Vram,
        keyboard: &Keyboard,
        video_sink: &mut VideoSink,
    ) {
        match *opcode {
            OpCode::Set(vx, value) => {
                self.v[vx as usize] = value;
                self.pc += 2;
            }
            OpCode::Copy(vx, vy) => {
                self.v[vx as usize] = self.v[vy as usize];
                self.pc += 2;
            }
            OpCode::Add(vx, value) => {
                let (result, overflow) = self.v[vx as usize].overflowing_add(value);
                self.v[vx as usize] = result;
                self.v[0xF as usize] = if overflow { 1 } else { 0 };
                self.pc += 2;
            }
            OpCode::AddVy(vx, vy) => {
                let value_x = self.v[vx as usize];
                let value_y = self.v[vy as usize];
                let (result, overflow) = value_x.overflowing_add(value_y);
                self.v[vx as usize] = result;
                self.v[0xF as usize] = if overflow { 1 } else { 0 };
                self.pc += 2;
            }
            OpCode::SubVx(vx, vy) => {
                let value_x = self.v[vx as usize];
                let value_y = self.v[vy as usize];
                self.v[0xF as usize] = if value_x > value_y { 1 } else { 0 };
                self.v[vx as usize] = value_y.saturating_sub(value_x);
                self.pc += 2;
            }
            OpCode::SubVy(vx, vy) => {
                let value_x = self.v[vx as usize];
                let value_y = self.v[vy as usize];
                self.v[0xF as usize] = if value_y > value_x { 1 } else { 0 };
                self.v[vx as usize] = value_x.saturating_sub(value_y);
                self.pc += 2;
            }
            OpCode::And(vx, vy) => {
                self.v[vx as usize] &= self.v[vy as usize];
                self.pc += 2;
            }
            OpCode::Or(vx, vy) => {
                self.v[vx as usize] |= self.v[vy as usize];
                self.pc += 2;
            }
            OpCode::Xor(vx, vy) => {
                self.v[vx as usize] ^= self.v[vy as usize];
                self.pc += 2;
            }
            OpCode::ShiftRight(vx, vy) => {
                let value = self.v[vy as usize];
                self.v[0xF as usize] = value & 0x1;
                self.v[vx as usize] = value >> 1;
                self.pc += 2;
            }
            OpCode::ShiftLeft(vx, vy) => {
                let value = self.v[vy as usize];
                self.v[0xF as usize] = value & 0b1000_0000 >> 7;
                self.v[vx as usize] = value << 1;
                self.pc += 2;
            }
            OpCode::Jmp(address) => {
                self.pc = address;
            }
            OpCode::JmpV0(address) => {
                self.pc = address + self.v[0] as Address;
            }
            OpCode::Jeq(vx, value) => {
                if self.v[vx as usize] == value {
                    self.pc += 2;
                }
                self.pc += 2;
            }
            OpCode::JeqVy(vx, vy) => {
                if self.v[vx as usize] == self.v[vy as usize] {
                    self.pc += 2;
                }
                self.pc += 2;
            }
            OpCode::Jneq(vx, value) => {
                if self.v[vx as usize] != value {
                    self.pc += 2;
                }
                self.pc += 2;
            }
            OpCode::JneqVy(vx, vy) => {
                if self.v[vx as usize] != self.v[vy as usize] {
                    self.pc += 2;
                }
                self.pc += 2;
            }
            OpCode::JmpK(vx) => {
                if keyboard.get_pressed(HexKey::from(self.v[vx as usize])) {
                    self.pc += 2;
                }
                self.pc += 2;
            }
            OpCode::JmpNK(vx) => {
                if !keyboard.get_pressed(HexKey::from(self.v[vx as usize])) {
                    self.pc += 2;
                }
                self.pc += 2;
            }
            OpCode::WaitForKey(vx) => {
                let mut key_pressed = false;
                for k in 0..0x10 {
                    if keyboard.get_pressed(HexKey::from(k)) {
                        self.v[vx as usize] = k;
                        key_pressed = true;
                    }
                }
                // println!("{:?}", keyboard);
                if key_pressed {
                    self.pc += 2;
                }
            }
            OpCode::Store(vx) => {
                let mut r = 0;
                for addr in self.i..self.i + (vx as u16) + 1 {
                    memory.write(addr, self.v[r as usize]);
                    r += 1;
                }
                self.i += r;
                self.pc += 2;
            }
            OpCode::Load(vx) => {
                let mut r = 0;
                for addr in self.i..self.i + (vx as u16) + 1 {
                    self.v[r as usize] = memory.read(addr);
                    r += 1;
                }
                self.i += r;
                self.pc += 2;
            }
            OpCode::Call(address) => {
                self.stack.push(self.pc);
                self.pc = address;
            }
            OpCode::Return() => {
                let addr = self.stack.pop();
                self.pc = addr;
                self.pc += 2;
            }
            OpCode::SetDelayTimer(vx) => {
                self.delay_timer = self.v[vx as usize];
                self.pc += 2;
            }
            OpCode::LdDelayTimer(vx) => {
                self.v[vx as usize] = self.delay_timer;
                self.pc += 2;
            }
            OpCode::SetSoundTimer(vx) => {
                if self.v[vx as usize] > 2 {
                    self.sound_timer = self.v[vx as usize];
                }
                self.pc += 2;
            }
            OpCode::SetI(value) => {
                self.i = value;
                self.pc += 2;
            }
            OpCode::AddIVx(vx) => {
                let value_x = self.v[vx as usize];
                self.v[0xF as usize] = if self.i + value_x as u16 > 0xFFF {
                    1
                } else {
                    0
                };
                //TODO, wrap around 0xFFF
                self.i += self.v[vx as usize] as Address;
                self.pc += 2;
            }
            OpCode::DrawSprite(vx, vy, value) => {
                let value_x = self.v[vx as usize];
                let value_y = self.v[vy as usize];
                self.v[0xF as usize] =
                    if vram.draw_sprite(memory, self.i, value_x, value_y, value, video_sink) {
                        1
                    } else {
                        0
                    };
                self.pc += 2;
            }
            OpCode::Font(vx) => {
                self.i = (self.v[vx as usize] * 5) as Address;
                self.pc += 2;
            }
            OpCode::ClearScreen() => {
                vram.clear();
                self.pc += 2;
            }
            OpCode::BCD(vx) => {
                let mut x = self.v[vx as usize];
                const DECIMAL_LENGTH: usize = 3;
                let mut digits = vec![0 as u8; DECIMAL_LENGTH];
                for digit_count in 0..3 {
                    digits[DECIMAL_LENGTH - digit_count - 1] = x % 10;
                    x /= 10;
                }
                let i = self.i;
                memory.write(i, digits[0]);
                memory.write(i + 1, digits[1]);
                memory.write(i + 2, digits[2]);
                self.pc += 2;
            }
            OpCode::Random(vx, mask) => {
                let mut rng = thread_rng();
                let random = rng.gen::<u8>();
                self.v[vx as usize] = random & mask;
                self.pc += 2;
            }
        }
    }
}
