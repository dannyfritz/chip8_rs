use chip8::{Address, Register};
use std::fmt;

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
    Jeq(Register, u8),
    JeqVy(Register, Register),
    Jneq(Register, u8),
    JneqVy(Register, Register),
    JmpK(Register),
    JmpNK(Register),
    Load(Register),
    Store(Register),
    Call(Address),
    Return(),
    SetDelayTimer(Register),
    LdDelayTimer(Register),
    SetSoundTimer(Register),
    SetI(Address),
    AddIVx(Register),
    DrawSprite(Register, Register, u8),
    Font(Register),
    ClearScreen(),
    BCD(Register),
    Random(Register, u8),
}

impl fmt::Debug for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &OpCode::Set(vx, value) => write!(f, "Set({:1x},{:2x})", vx, value),
            &OpCode::Copy(vx, vy) => write!(f, "Copy({:1x},{:1x})", vx, vy),
            &OpCode::Add(vx, value) => write!(f, "Add({:1x},{:2x})", vx, value),
            &OpCode::Or(vx, vy) => write!(f, "Or({:1x},{:1x})", vx, vy),
            &OpCode::And(vx, vy) => write!(f, "And({:1x},{:1x})", vx, vy),
            &OpCode::Xor(vx, vy) => write!(f, "Xor({:1x},{:1x})", vx, vy),
            &OpCode::AddVy(vx, vy) => write!(f, "AddVy({:1x},{:1x})", vx, vy),
            &OpCode::SubVy(vx, vy) => write!(f, "SubVy({:1x},{:1x})", vx, vy),
            &OpCode::ShiftRight(vx, vy) => write!(f, "ShiftRight({:1x},{:1x})", vx, vy),
            &OpCode::SubVx(vx, vy) => write!(f, "SubVx({:1x},{:1x})", vx, vy),
            &OpCode::ShiftLeft(vx, vy) => write!(f, "ShiftLeft({:1x},{:1x})", vx, vy),
            &OpCode::JneqVy(vx, vy) => write!(f, "JneqVy({:1x},{:1x})", vx, vy),
            &OpCode::SetI(address) => write!(f, "SetI({:3x})", address),
            &OpCode::Jmp(address) => write!(f, "Jmp({:3x})", address),
            &OpCode::Jeq(vx, value) => write!(f, "Jeq({:1x},{:2x})", vx, value),
            &OpCode::Jneq(vx, value) => write!(f, "Jneq({:1x},{:2x})", vx, value),
            &OpCode::JeqVy(vx, vy) => write!(f, "JeqVy({:1x},{:1x})", vx, vy),
            &OpCode::JmpV0(address) => write!(f, "JmpV0({:3x})", address),
            &OpCode::JmpK(vx) => write!(f, "JmpK({:1x})", vx),
            &OpCode::JmpNK(vx) => write!(f, "JmpNK({:1x})", vx),
            &OpCode::Store(vx) => write!(f, "Store({:1x})", vx),
            &OpCode::Load(vx) => write!(f, "Load({:1x})", vx),
            &OpCode::Return() => write!(f, "Return()"),
            &OpCode::Call(address) => write!(f, "Call({:3x})", address),
            &OpCode::SetDelayTimer(vx) => write!(f, "SetDelayTimer({:1x})", vx),
            &OpCode::LdDelayTimer(vx) => write!(f, "LdDelayTimer({:1x})", vx),
            &OpCode::SetSoundTimer(vx) => write!(f, "SetSoundTimer({:1x})", vx),
            &OpCode::DrawSprite(vx, vy, value) => {
                write!(f, "DrawSprite({:1x},{:1x},{:1x})", vx, vy, value)
            }
            &OpCode::Font(vx) => write!(f, "Font({:1x})", vx),
            &OpCode::ClearScreen() => write!(f, "ClearScreen()"),
            &OpCode::AddIVx(vx) => write!(f, "AddIVx({:1x})", vx),
            &OpCode::BCD(vx) => write!(f, "BCD({:1x})", vx),
            &OpCode::Random(vx, value) => write!(f, "Random({:1x},{:2x})", vx, value),
            // _ => write!(f, ""),
        }
    }
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
            0xC => OpCode::Random(self.get_vx(), self.get_8bconst()),
            0xD => OpCode::DrawSprite(self.get_vx(), self.get_vy(), self.get_4bconst()),
            0xE => match self.value & 0x00FF {
                0x9E => OpCode::JmpK(self.get_vx()),
                0xA1 => OpCode::JmpNK(self.get_vx()),
                _ => no_opcode!(self.value),
            },
            0xF => match self.value & 0x00FF {
                0x15 => OpCode::SetDelayTimer(self.get_vx()),
                0x07 => OpCode::LdDelayTimer(self.get_vx()),
                0x18 => OpCode::SetSoundTimer(self.get_vx()),
                0x1E => OpCode::AddIVx(self.get_vx()),
                0x29 => OpCode::Font(self.get_vx()),
                0x33 => OpCode::BCD(self.get_vx()),
                0x55 => OpCode::Store(self.get_vx()),
                0x65 => OpCode::Load(self.get_vx()),
                _ => no_opcode!(self.value),
            },
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
