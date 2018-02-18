use glutin::{ElementState, KeyboardInput};

pub struct Keyboard {
    keys: [KeyState; 0x10],
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {
            /* INFO:
            chip8      qwerty
            1 2 3 C    1 2 3 4
            4 5 6 D    q w e r
            7 8 9 E    a s d f
            A 0 B F    z x c v
            */
            keys: [
                KeyState::new(45),
                KeyState::new(2),
                KeyState::new(3),
                KeyState::new(4),
                KeyState::new(16),
                KeyState::new(17),
                KeyState::new(18),
                KeyState::new(30),
                KeyState::new(31),
                KeyState::new(32),
                KeyState::new(44),
                KeyState::new(46),
                KeyState::new(5),
                KeyState::new(19),
                KeyState::new(33),
                KeyState::new(47),
            ],
        }
    }
    pub fn update_key(&mut self, input: KeyboardInput) {
        match self.keys
            .iter_mut()
            .find(|key| key.scancode == input.scancode)
        {
            Some(key_state) => key_state.pressed = input.state == ElementState::Pressed,
            None => return,
        }
    }
    pub fn get_pressed(&mut self, key: u8) -> bool {
        self.keys[key as usize].pressed
    }
}

struct KeyState {
    scancode: u32,
    pressed: bool,
}

impl KeyState {
    fn new(scancode: u32) -> KeyState {
        KeyState {
            scancode: scancode,
            pressed: false,
        }
    }
}
