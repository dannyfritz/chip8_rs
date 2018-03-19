use chip8::memory::Memory;
use chip8::Address;
use std::fmt;

const WIDTH: usize = 64;
const HEIGHT: usize = 32;
const SPRITE_WIDTH: u8 = 8;
pub struct PixelBuffer(pub [bool; WIDTH * HEIGHT]);

pub struct VideoSink {
    pub buffer: Option<PixelBuffer>,
}

impl VideoSink {
    pub fn new() -> VideoSink {
        VideoSink { buffer: None }
    }
    pub fn get(&mut self) -> Option<PixelBuffer> {
        self.buffer.take()
    }
}

pub struct Vram {
    //TODO: unroll this to [bool; WIDTH * HEIGHT]
    data: [[bool; WIDTH]; HEIGHT],
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
            data: [[false; WIDTH]; HEIGHT],
        }
    }
    pub fn clear(&mut self) {
        self.data = [[false; WIDTH]; HEIGHT];
    }
    pub fn draw_sprite(
        &mut self,
        memory: &Memory,
        addr: Address,
        x: u8,
        y: u8,
        rows: u8,
        sink: &mut VideoSink,
    ) -> bool {
        let mut pixel_unset = false;
        for row in 0..rows {
            if row + y >= HEIGHT as u8 {
                continue;
            }
            let data = memory.read(addr + row as u16);
            for col in 0..SPRITE_WIDTH {
                if col + x >= WIDTH as u8 {
                    continue;
                }
                let (result, _) = data.overflowing_shr((SPRITE_WIDTH - col).into());
                let pixel = &mut self.data[(row + y) as usize][(col + x) as usize];
                let existing_pixel = *pixel;
                *pixel ^= result & 1 == 1;
                if existing_pixel == true && *pixel == false {
                    pixel_unset = true;
                }
            }
        }
        let mut buffer = PixelBuffer([false; WIDTH * HEIGHT]);
        for row in 0..HEIGHT {
            for col in 0..WIDTH {
                let pixel = &mut self.data[row][col];
                buffer.0[row * WIDTH + col] = *pixel;
            }
        }
        sink.buffer = Some(buffer);
        return pixel_unset;
    }
}
