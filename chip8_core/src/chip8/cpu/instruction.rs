use chip8::{Address, Register};
use std::fmt;

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
    WaitForKey(Register),
}

impl fmt::Debug for OpCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            OpCode::Set(vx, value) => write!(f, "Set({:1x},{:2x})", vx, value),
            OpCode::Copy(vx, vy) => write!(f, "Copy({:1x},{:1x})", vx, vy),
            OpCode::Add(vx, value) => write!(f, "Add({:1x},{:2x})", vx, value),
            OpCode::Or(vx, vy) => write!(f, "Or({:1x},{:1x})", vx, vy),
            OpCode::And(vx, vy) => write!(f, "And({:1x},{:1x})", vx, vy),
            OpCode::Xor(vx, vy) => write!(f, "Xor({:1x},{:1x})", vx, vy),
            OpCode::AddVy(vx, vy) => write!(f, "AddVy({:1x},{:1x})", vx, vy),
            OpCode::SubVy(vx, vy) => write!(f, "SubVy({:1x},{:1x})", vx, vy),
            OpCode::ShiftRight(vx, vy) => write!(f, "ShiftRight({:1x},{:1x})", vx, vy),
            OpCode::SubVx(vx, vy) => write!(f, "SubVx({:1x},{:1x})", vx, vy),
            OpCode::ShiftLeft(vx, vy) => write!(f, "ShiftLeft({:1x},{:1x})", vx, vy),
            OpCode::JneqVy(vx, vy) => write!(f, "JneqVy({:1x},{:1x})", vx, vy),
            OpCode::SetI(address) => write!(f, "SetI({:3x})", address),
            OpCode::Jmp(address) => write!(f, "Jmp({:3x})", address),
            OpCode::Jeq(vx, value) => write!(f, "Jeq({:1x},{:2x})", vx, value),
            OpCode::Jneq(vx, value) => write!(f, "Jneq({:1x},{:2x})", vx, value),
            OpCode::JeqVy(vx, vy) => write!(f, "JeqVy({:1x},{:1x})", vx, vy),
            OpCode::JmpV0(address) => write!(f, "JmpV0({:3x})", address),
            OpCode::JmpK(vx) => write!(f, "JmpK({:1x})", vx),
            OpCode::JmpNK(vx) => write!(f, "JmpNK({:1x})", vx),
            OpCode::Store(vx) => write!(f, "Store({:1x})", vx),
            OpCode::Load(vx) => write!(f, "Load({:1x})", vx),
            OpCode::Return() => write!(f, "Return()"),
            OpCode::Call(address) => write!(f, "Call({:3x})", address),
            OpCode::SetDelayTimer(vx) => write!(f, "SetDelayTimer({:1x})", vx),
            OpCode::LdDelayTimer(vx) => write!(f, "LdDelayTimer({:1x})", vx),
            OpCode::SetSoundTimer(vx) => write!(f, "SetSoundTimer({:1x})", vx),
            OpCode::DrawSprite(vx, vy, value) => {
                write!(f, "DrawSprite({:1x},{:1x},{:1x})", vx, vy, value)
            }
            OpCode::Font(vx) => write!(f, "Font({:1x})", vx),
            OpCode::ClearScreen() => write!(f, "ClearScreen()"),
            OpCode::AddIVx(vx) => write!(f, "AddIVx({:1x})", vx),
            OpCode::BCD(vx) => write!(f, "BCD({:1x})", vx),
            OpCode::Random(vx, value) => write!(f, "Random({:1x},{:2x})", vx, value),
            OpCode::WaitForKey(vx) => write!(f, "WaitForKey({:1x})", vx),
            // _ => write!(f, ""),
        }
    }
}

pub struct Instruction {
    value: u16,
}

impl Instruction {
    pub fn new(value: u16) -> Instruction {
        Instruction { value }
    }
    pub fn decode(&self) -> OpCode {
        macro_rules! no_opcode {
            ($x: expr) => {
                panic!(format!("No OpCode for 0x{:04X} yet!", $x));
            };
        }
        macro_rules! get_opcode {
            ($op_code:expr, $mask:expr, $output:expr) => {
                if self.value & $mask == $op_code {
                    return $output
                }
            };
        }
        get_opcode!(0x00E0, 0xFFFF, OpCode::ClearScreen());
        get_opcode!(0x00EE, 0xFFFF, OpCode::Return());
        get_opcode!(0x1000, 0xF000, OpCode::Jmp(self.get_address()));
        get_opcode!(0x2000, 0xF000, OpCode::Call(self.get_address()));
        get_opcode!(0x3000, 0xF000, OpCode::Jeq(self.get_vx(), self.get_8bconst()));
        get_opcode!(0x4000, 0xF000, OpCode::Jneq(self.get_vx(), self.get_8bconst()));
        get_opcode!(0x5000, 0xF00F, OpCode::JeqVy(self.get_vx(), self.get_vy()));
        get_opcode!(0x6000, 0xF000, OpCode::Set(self.get_vx(), self.get_8bconst()));
        get_opcode!(0x7000, 0xF000, OpCode::Add(self.get_vx(), self.get_8bconst()));
        get_opcode!(0x8000, 0xF00F, OpCode::Copy(self.get_vx(), self.get_vy()));
        get_opcode!(0x8001, 0xF00F, OpCode::Or(self.get_vx(), self.get_vy()));
        get_opcode!(0x8002, 0xF00F, OpCode::And(self.get_vx(), self.get_vy()));
        get_opcode!(0x8003, 0xF00F, OpCode::Xor(self.get_vx(), self.get_vy()));
        get_opcode!(0x8004, 0xF00F, OpCode::AddVy(self.get_vx(), self.get_vy()));
        get_opcode!(0x8005, 0xF00F, OpCode::SubVy(self.get_vx(), self.get_vy()));
        get_opcode!(0x8006, 0xF00F, OpCode::ShiftRight(self.get_vx(), self.get_vy()));
        get_opcode!(0x8007, 0xF00F, OpCode::SubVx(self.get_vx(), self.get_vy()));
        get_opcode!(0x8008, 0xF00F, OpCode::ShiftLeft(self.get_vx(), self.get_vy()));
        get_opcode!(0x9000, 0xF000, OpCode::JneqVy(self.get_vx(), self.get_vy()));
        get_opcode!(0xA000, 0xF000, OpCode::SetI(self.get_address()));
        get_opcode!(0xB000, 0xF000, OpCode::JmpV0(self.get_address()));
        get_opcode!(0xC000, 0xF000, OpCode::Random(self.get_vx(), self.get_8bconst()));
        get_opcode!(0xD000, 0xF000, OpCode::DrawSprite(self.get_vx(), self.get_vy(), self.get_4bconst()));
        get_opcode!(0xE09E, 0xF0FF, OpCode::JmpK(self.get_vx()));
        get_opcode!(0xE0A1, 0xF0FF, OpCode::JmpNK(self.get_vx()));
        get_opcode!(0xF00A, 0xF0FF, OpCode::WaitForKey(self.get_vx()));
        get_opcode!(0xF007, 0xF0FF, OpCode::LdDelayTimer(self.get_vx()));
        get_opcode!(0xF015, 0xF0FF, OpCode::SetDelayTimer(self.get_vx()));
        get_opcode!(0xF018, 0xF0FF, OpCode::SetSoundTimer(self.get_vx()));
        get_opcode!(0xF01E, 0xF0FF, OpCode::AddIVx(self.get_vx()));
        get_opcode!(0xF029, 0xF0FF, OpCode::Font(self.get_vx()));
        get_opcode!(0xF033, 0xF0FF, OpCode::BCD(self.get_vx()));
        get_opcode!(0xF055, 0xF0FF, OpCode::Store(self.get_vx()));
        get_opcode!(0xF065, 0xF0FF, OpCode::Load(self.get_vx()));
        no_opcode!(self.value);
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
