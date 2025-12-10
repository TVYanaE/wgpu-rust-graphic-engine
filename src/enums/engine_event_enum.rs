use winit::{
    keyboard::KeyCode,
    event::MouseButton,
};

#[derive(Debug, Clone, Copy)]
pub enum EngineEvent {
    KeyPressed(KeyCode),
    KeyReleased(KeyCode),
    MouseMoved{ dx: f32, dy: f32},
    MouseButtonPressed(MouseButton),
    MouseBottonReleased(MouseButton),
}

impl EngineEvent {
    pub fn from_keyboard_event(key_code: KeyCode, key_is_pressed: bool) -> Self {
        if key_is_pressed {
            Self::KeyPressed(key_code)
        }
        else {
            Self::KeyReleased(key_code)
        }
    }
}
