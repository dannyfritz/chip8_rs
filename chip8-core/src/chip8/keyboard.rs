use std::convert::From;
/* INFO:
    1 2 3 C 
    4 5 6 D 
    7 8 9 E 
    A 0 B F 
*/
#[derive(Debug)]
pub enum HexKey {
    X0,
    X1,
    X2,
    X3,
    X4,
    X5,
    X6,
    X7,
    X8,
    X9,
    Xa,
    Xb,
    Xc,
    Xd,
    Xe,
    Xf,
}

impl From<u8> for HexKey {
    fn from(index: u8) -> Self {
        match index {
            0x0 => HexKey::X0,
            0x1 => HexKey::X1,
            0x2 => HexKey::X2,
            0x3 => HexKey::X3,
            0x4 => HexKey::X4,
            0x5 => HexKey::X5,
            0x6 => HexKey::X6,
            0x7 => HexKey::X7,
            0x8 => HexKey::X8,
            0x9 => HexKey::X9,
            0xa => HexKey::Xa,
            0xb => HexKey::Xb,
            0xc => HexKey::Xc,
            0xd => HexKey::Xd,
            0xe => HexKey::Xe,
            0xf => HexKey::Xf,
            _ => panic!(format!("Not a valid keyboard key, {}.", index)),
        }
    }
}

impl From<HexKey> for usize {
    fn from(key: HexKey) -> usize {
        match key {
            HexKey::X0 => 0x0,
            HexKey::X1 => 0x1,
            HexKey::X2 => 0x2,
            HexKey::X3 => 0x3,
            HexKey::X4 => 0x4,
            HexKey::X5 => 0x5,
            HexKey::X6 => 0x6,
            HexKey::X7 => 0x7,
            HexKey::X8 => 0x8,
            HexKey::X9 => 0x9,
            HexKey::Xa => 0xa,
            HexKey::Xb => 0xb,
            HexKey::Xc => 0xc,
            HexKey::Xd => 0xd,
            HexKey::Xe => 0xe,
            HexKey::Xf => 0xf,
        }
    }
}

pub struct Keyboard {
    keys: [bool; 0x10],
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {
            keys: [false; 0x10],
        }
    }
    pub fn update_key(&mut self, key: HexKey, pressed: bool) {
        self.keys[usize::from(key)] = pressed;
    }
    pub fn get_pressed(&self, key: HexKey) -> bool {
        self.keys[usize::from(key)]
    }
}
