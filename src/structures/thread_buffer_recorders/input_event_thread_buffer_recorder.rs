use flume::{
    Sender
};
use crate::{ 
    enums::{
        input_event_enum::InputEvent,
    },
};

pub struct InputEventThreadBufferRecorder {
    input_event_channel_sender: Sender<InputEvent>, 
}

impl InputEventThreadBufferRecorder {
    pub fn new(input_event_channel_sender: Sender<InputEvent>) -> Self {
        Self { input_event_channel_sender }
    }

    pub fn register_input_event(&self, input_event: impl Into<InputEvent>) {
        self.input_event_channel_sender.send(input_event.into());
    }
}




