use chip8::{Address, Register};

macro_rules! no_opcode {
    ($x:expr) => {
        panic!(format!("No OpCode for 0x{:04X} yet!", $x));
    }
}

pub enum OpCode {
    Set(Register, u8),
    Copy(Register, Register),
    Add(Register, u8),
    AddVy(Register, Register),
    SubVx(Register, Register),
    SubVy(Register, Register),
    And(Register, Register),
    Or(Register, Register),
    Xor(Register, Register),
    ShiftRight(Register, Register),
    ShiftLeft(Register, Register),
    Jmp(Address),
    JmpV0(Address),
    Call(Address),
    Return(),
    Jeq(Register, u8),
    JeqVy(Register, Register),
    Jneq(Register, u8),
    JneqVy(Register, Register),
    SetDelayTimer(Register),
    LdDelayTimer(Register),
    SetSoundTimer(Register),
    SetI(Address),
    SetIVx(Register),
    ClearScreen(),
    DrawSprite(Register, Register, u8),
    Store(Register),
    Load(Register),
    BCD(Register),
    Font(Register),
}

pub struct Instruction {
    value: u16,
}

impl Instruction {
    pub fn new(value: u16) -> Instruction {
        Instruction { value: value }
    }
    pub fn to_opcode(&self) -> OpCode {
        match (self.value & 0xF000) >> 12 {
            0x0 => match self.value & 0x0FFF {
                0x0E0 => OpCode::ClearScreen(),
                0x0EE => OpCode::Return(),
                _ => no_opcode!(self.value),
            },
            0x1 => OpCode::Jmp(self.get_address()),
            0x2 => OpCode::Call(self.get_address()),
            0x3 => OpCode::Jeq(self.get_vx(), self.get_8bconst()),
            0x4 => OpCode::Jneq(self.get_vx(), self.get_8bconst()),
            0x5 => OpCode::JeqVy(self.get_vx(), self.get_vy()),
            0x6 => OpCode::Set(self.get_vx(), self.get_8bconst()),
            0x7 => OpCode::Add(self.get_vx(), self.get_8bconst()),
            0x8 => match self.value & 0x000F {
                0x0 => OpCode::Copy(self.get_vx(), self.get_vy()),
                0x1 => OpCode::Or(self.get_vx(), self.get_vy()),
                0x2 => OpCode::And(self.get_vx(), self.get_vy()),
                0x3 => OpCode::Xor(self.get_vx(), self.get_vy()),
                0x4 => OpCode::AddVy(self.get_vx(), self.get_vy()),
                0x5 => OpCode::SubVy(self.get_vx(), self.get_vy()),
                0x6 => OpCode::ShiftRight(self.get_vx(), self.get_vy()),
                0x7 => OpCode::SubVx(self.get_vx(), self.get_vy()),
                0xE => OpCode::ShiftLeft(self.get_vx(), self.get_vy()),
                _ => no_opcode!(self.value),
            },
            0x9 => OpCode::JneqVy(self.get_vx(), self.get_vy()),
            0xA => OpCode::SetI(self.get_address()),
            0xB => OpCode::JmpV0(self.get_address()),
            0xD => OpCode::DrawSprite(self.get_vx(), self.get_vy(), self.get_4bconst()),
            0xF => match self.value & 0x00FF {
                0x15 => OpCode::SetDelayTimer(self.get_vx()),
                0x07 => OpCode::LdDelayTimer(self.get_vx()),
                0x18 => OpCode::SetSoundTimer(self.get_vx()),
                0x1E => OpCode::SetIVx(self.get_vx()),
                0x29 => OpCode::Font(self.get_vx()),
                0x33 => OpCode::BCD(self.get_vx()),
                0x55 => OpCode::Store(self.get_vx()),
                0x65 => OpCode::Load(self.get_vx()),
                _ => no_opcode!(self.value),
            }
            _ => no_opcode!(self.value),
        }
    }
    pub fn get_address(&self) -> Address {
        self.value & 0x0FFF
    }
    pub fn get_vx(&self) -> Register {
        ((self.value & 0x0F00) >> 8) as u8
    }
    pub fn get_vy(&self) -> Register {
        ((self.value & 0x00F0) >> 4) as u8
    }
    pub fn get_8bconst(&self) -> u8 {
        (self.value & 0x00FF) as u8
    }
    pub fn get_4bconst(&self) -> u8 {
        (self.value & 0x000F) as u8
    }
}
