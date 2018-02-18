use chip8::Address;
use std::fmt;

const FRAME_COUNT: usize = 16;

pub struct Stack {
    frames: [u16; FRAME_COUNT],
    sp: u8,
}

impl fmt::Debug for Stack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[sp] {}", self.sp)?;
        write!(f, " [top] {:04x}", self.frames[self.sp as usize])
    }
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            frames: [0; FRAME_COUNT],
            sp: 0,
        }
    }
    pub fn push(&mut self, addr: Address) {
        self.frames[self.sp as usize] = addr;
        self.sp += 1;
    }
    pub fn pop(&mut self) -> Address {
        self.sp -= 1;
        self.frames[self.sp as usize]
    }
}
