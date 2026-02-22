use std::collections::HashSet;
use winit::keyboard::KeyCode;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Key {
    W,
    A,
    S,
    D,
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    Space,
    Enter,
    Escape,
}

fn map_keycode(code: KeyCode) -> Option<Key> {
    match code {
        KeyCode::KeyW => Some(Key::W),
        KeyCode::KeyA => Some(Key::A),
        KeyCode::KeyS => Some(Key::S),
        KeyCode::KeyD => Some(Key::D),
        KeyCode::ArrowUp => Some(Key::ArrowUp),
        KeyCode::ArrowDown => Some(Key::ArrowDown),
        KeyCode::ArrowLeft => Some(Key::ArrowLeft),
        KeyCode::ArrowRight => Some(Key::ArrowRight),
        KeyCode::Space => Some(Key::Space),
        KeyCode::Enter => Some(Key::Enter),
        KeyCode::Escape => Some(Key::Escape),
        _ => None,
    }
}

pub struct Input {
    held: HashSet<Key>,
}

impl Input {
    pub fn new() -> Self {
        Self {
            held: HashSet::new(),
        }
    }

    pub(crate) fn press(&mut self, code: KeyCode) {
        if let Some(key) = map_keycode(code) {
            self.held.insert(key);
        }
    }

    pub(crate) fn release(&mut self, code: KeyCode) {
        if let Some(key) = map_keycode(code) {
            self.held.remove(&key);
        }
    }

    pub fn is_held(&self, key: Key) -> bool {
        self.held.contains(&key)
    }
}
