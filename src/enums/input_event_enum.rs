use winit::{
    event::WindowEvent,
};

#[derive(Debug, Clone)]
pub enum InputEvent {
    WindowEvent(WindowEvent),
    FrameStart,
    Shutdown,
}

impl Into<InputEvent> for WindowEvent {
    fn into(self) -> InputEvent {
        InputEvent::WindowEvent(self)
    }
}
