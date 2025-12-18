
use winit::{
    event::{WindowEvent},
};

pub enum WinitEvent {
    WindowEvent(WindowEvent),
}


impl From<WindowEvent> for WinitEvent {
    fn from(value: WindowEvent) -> Self {
        Self::WindowEvent(value)
    }
}
