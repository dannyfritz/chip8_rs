use chip8::memory::Memory;
use chip8::Address;
use std::fmt;

const WIDTH: usize = 32;
const HEIGHT: usize = 64;
const SPRITE_WIDTH: u8 = 8;

pub struct Vram {
    data: [[bool; HEIGHT]; WIDTH],
}

impl fmt::Debug for Vram {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in self.data.iter() {
            for pixel in row.iter() {
                if *pixel == true {
                    write!(f, "#")?;
                } else {
                    write!(f, " ")?;
                }
            }
            write!(f, "\n")?;
        }
        write!(f, "")
    }
}

impl Vram {
    pub fn new() -> Vram {
        Vram {
            data: [[false; HEIGHT]; WIDTH],
        }
    }
    pub fn clear(&mut self) {
        self.data = [[false; HEIGHT]; WIDTH];
    }
    pub fn draw_sprite(&mut self, memory: &Memory, addr: Address, x: u8, y: u8, rows: u8) -> bool {
        for row in 0..rows {
            let data = memory.read(addr + row as u16);
            for col in 0..SPRITE_WIDTH {
                let (result, _) = data.overflowing_shr((SPRITE_WIDTH - col).into());
                self.data[(x + col) as usize][(row + y) as usize] ^= result & 1 == 1;
            }
        }
        // println!("{:?}", self);
        //TODO: Return true if a pixel was changed to 1
        return false;
    }
}
