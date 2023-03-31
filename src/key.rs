use rdev::Key;
pub trait ToKey {
    fn to_key(&self) -> Option<Key>;
}
impl ToKey for String {
    fn to_key(&self) -> Option<Key> {
        match self.as_str() {
            "a" => {
                return Some(Key::KeyA);
            },
            "b" => {
                return Some(Key::KeyB);
            },
            "c" => {
                return Some(Key::KeyC);
            },
            "d" => {
                return Some(Key::KeyD);
            },
            "e" => {
                return Some(Key::KeyE);
            },
            "f" => {
                return Some(Key::KeyF);
            },
            "g" => {
                return Some(Key::KeyG);
            },
            "h" => {
                return Some(Key::KeyH);
            },
            "i" => {
                return Some(Key::KeyI);
            },
            "j" => {
                return Some(Key::KeyJ);
            },
            "k" => {
                return Some(Key::KeyK);
            },
            "l" => {
                return Some(Key::KeyL);
            },
            "m" => {
                return Some(Key::KeyM);
            },
            "n" => {
               return Some(Key::KeyN);
            },
            "o" => {
                return Some(Key::KeyO);
            },
            "p" => {
                return Some(Key::KeyP);
            },
            "q" => {
                return Some(Key::KeyQ);
            },
            "r" => {
                return Some(Key::KeyR);
            },
            "s" => {
                return Some(Key::KeyS);
            },
            "t" => {
                return Some(Key::KeyT);
            },
            "u" => {
                return Some(Key::KeyU);
            },
            "v" => {
                return Some(Key::KeyV);
            },
            "w" => {
                return Some(Key::KeyW);
            },
            "x" => {
                return Some(Key::KeyX);
            },
            "y" => {
                return Some(Key::KeyY);
            },
            "z" => {
                return Some(Key::KeyZ);
            },
            "shift" => {
                return Some(Key::ShiftLeft);
            },
            "backspace" => {
                return Some(Key::Backspace);
            },
            "enter" => {
                return Some(Key::Return);
            }
            _ => {return None;}
        }
    }
}
