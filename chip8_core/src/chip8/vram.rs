use chip8::memory::Memory;
use chip8::Address;
use std::fmt;

pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;
const SPRITE_WIDTH: usize = 8;
pub struct PixelBuffer {
    pub data: [bool; WIDTH * HEIGHT],
}

#[derive(Default)]
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
    data: [bool; WIDTH * HEIGHT],
}

impl fmt::Debug for Vram {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..HEIGHT {
            for col in 0..WIDTH {
                let pixel = &self.data[row * HEIGHT + col];
                if *pixel {
                    write!(f, "#")?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;
        }
        write!(f, "")
    }
}

impl Default for Vram {
    fn default() -> Self {
        Vram {
            data: [false; WIDTH * HEIGHT],
        }
    }
}

impl Vram {
    pub fn new() -> Vram {
        Self::default()
    }
    pub fn clear(&mut self) {
        self.data = [false; WIDTH * HEIGHT];
    }
    pub fn draw_sprite(
        &mut self,
        memory: &Memory,
        sprite_addr: Address,
        x: u8,
        y: u8,
        rows: u8,
        sink: &mut VideoSink,
    ) -> bool {
        let mut pixel_unset = false;
        for row in 0..rows as usize {
            if row + y as usize >= HEIGHT {
                continue;
            }
            let sprite = memory.read(sprite_addr + row as u16);
            for col in 0..SPRITE_WIDTH {
                if col + x as usize >= WIDTH {
                    continue;
                }
                let (blit, _) = sprite.overflowing_shr((SPRITE_WIDTH - col - 1) as u32);
                let pixel = &mut self.data[(row + y as usize) * WIDTH + (col + x as usize)];
                let existing_pixel = *pixel;
                *pixel ^= blit & 1 == 1;
                if existing_pixel && !(*pixel) {
                    pixel_unset = true;
                }
            }
        }
        let mut buffer = PixelBuffer {
            data: [false; WIDTH * HEIGHT],
        };
        for row in 0..HEIGHT {
            for col in 0..WIDTH {
                let pixel = &mut self.data[row * WIDTH + col];
                buffer.data[row * WIDTH + col] = *pixel;
            }
        }
        sink.buffer = Some(buffer);
        pixel_unset
    }
}
