use chip8::Address;

pub struct Stack {
    frames: [u16; 16],
    sp: u8,
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            frames: [0; 16],
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
