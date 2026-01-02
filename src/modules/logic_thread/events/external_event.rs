use winit::{ 
    event::{KeyEvent},
};

#[derive(Debug)]
pub enum ExternalEvent {
    KeyEvent(KeyEvent),
}

