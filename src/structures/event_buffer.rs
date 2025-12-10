use std::{
    collections::HashSet,
};
use winit::{
    keyboard::KeyCode,
    event::MouseButton,
};
use crate::{
    enums::{
        engine_event_enum::EngineEvent,
    },
};

pub struct EventBuffer {
    event_srorage: Vec<EngineEvent>,
    keys_down: HashSet<KeyCode>,
    mouse_buttons_down: HashSet<MouseButton>,
    mouse_position: (f32, f32),
}

impl EventBuffer {
    pub fn new() -> Self {
        Self { 
            event_srorage: Vec::new(), 
            keys_down: HashSet::new(), 
            mouse_buttons_down: HashSet::new(), 
            mouse_position: (0.0, 0.0) 
        }
    }

    pub fn register_event(&mut self, event: EngineEvent) {
        match event {
            EngineEvent::KeyPressed(key_code) => {
                self.keys_down.insert(key_code);
            },
            EngineEvent::KeyReleased(key_code) => {
                self.keys_down.remove(&key_code);
            },
            EngineEvent::MouseButtonPressed(mouse_button) => {
                self.mouse_buttons_down.insert(mouse_button);
            },
            EngineEvent::MouseBottonReleased(mouse_button) => {
                self.mouse_buttons_down.remove(&mouse_button);
            },
            EngineEvent::MouseMoved { dx, dy } => {
                self.mouse_position.0 += dx;
                self.mouse_position.1 += dy;
            }
            _ => {},
        }
        self.event_srorage.push(event);
    }

    pub fn clear_event_(&mut self) {
        self.event_srorage.clear();
    }
}
