use std::{
    collections::{
        HashSet,
    },
};
use shipyard::{
    Unique
};
use winit::{
    keyboard::{KeyCode, PhysicalKey},
    event::{KeyEvent, ElementState},
};


#[derive(Debug, Unique)]
pub struct KeyboardState {
    pressed_buttons: HashSet<KeyCode>,
    just_pressed_buttons: HashSet<KeyCode>,
    just_released_buttons: HashSet<KeyCode>,
}

impl KeyboardState {
    pub fn new() -> Self {
        Self { 
            pressed_buttons: HashSet::with_capacity(4), 
            just_pressed_buttons: HashSet::with_capacity(4), 
            just_released_buttons: HashSet::with_capacity(4), 
        }
    }

    pub fn register_key_event(&mut self, key_event: KeyEvent) {
        match key_event.physical_key {
            PhysicalKey::Code(key_code) => {
                match key_event.state {
                    ElementState::Pressed => {
                        self.pressed_buttons.insert(key_code);
                        self.just_pressed_buttons.insert(key_code);
                    },
                    ElementState::Released => {
                        self.pressed_buttons.remove(&key_code);
                        self.just_released_buttons.insert(key_code);
                    }
                } 
            },
            PhysicalKey::Unidentified(_) => {},
        }
    }

    pub fn is_pressed(&self, key_code: &KeyCode) -> bool {
        self.pressed_buttons.contains(key_code)
    }

    pub fn clear_temp_sets(&mut self) {
        self.pressed_buttons.clear();
        self.just_released_buttons.clear();
    }
}
