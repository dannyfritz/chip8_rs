extern crate rand;
extern crate glutin;
mod chip8;
mod program;

use chip8::{ Chip8 };
use program::{ Program };

fn main() {
    let mut chip8 = Chip8::new();
    let tank = Program::new("./programs/Pong.ch8");
    chip8.load_program(tank);
    chip8.run();
}
