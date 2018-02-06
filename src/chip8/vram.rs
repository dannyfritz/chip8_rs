use chip8::memory::Memory;
use chip8::Address;

const WIDTH: usize = 32;
const HEIGHT: usize = 64;
const SPRITE_WIDTH: u8 = 8;

pub struct Vram {
    data: [[bool; HEIGHT]; WIDTH],
}

fn print_pixel(pixel: &bool) {
    if *pixel == true {
        print!("#");
    } else {
        print!(" ");
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
            let data = memory.read(addr);
            let mut cursor = 0b1000_0000;
            for col in 0..SPRITE_WIDTH {
                self.data[(x + col) as usize][(row + y) as usize] ^= data & cursor == 1;
                cursor >> 1;
            }
        }
        self.print();
        //TODO: Return true if a pixel was changed to 1
        return false;
    }
    pub fn print(&mut self) {
        for row in self.data.iter() {
            for pixel in row.iter() {
                print_pixel(pixel);
            }
            print!("\n");
        }
    }
}
