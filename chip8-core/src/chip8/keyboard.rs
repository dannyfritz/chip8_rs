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
                KeyState::new(45, false),
                KeyState::new(2, false),
                KeyState::new(3, false),
                KeyState::new(4, false),
                KeyState::new(16, false),
                KeyState::new(17, false), 
                KeyState::new(18, false),
                KeyState::new(30, false),
                KeyState::new(31, false),
                KeyState::new(32, false),
                KeyState::new(44, false),
                KeyState::new(46, false),
                KeyState::new(5, false),
                KeyState::new(19, false),
                KeyState::new(33, false),
                KeyState::new(47, false),
            ],
        }
    }
    pub fn update_key(&mut self, input: KeyState) {
        match self.keys
            .iter_mut()
            .find(|key| key.scancode == input.scancode)
        {
            Some(key_state) => key_state.pressed = input.pressed,
            None => return,
        }
    }
    pub fn get_pressed(& self, key: u8) -> bool {
        self.keys[key as usize].pressed
    }
}

pub struct KeyState {
    scancode: u32,
    pressed: bool,
}

impl KeyState {
    pub fn new(scancode: u32, pressed: bool) -> KeyState {
        KeyState {
            scancode: scancode,
            pressed: pressed,
        }
    }
}
