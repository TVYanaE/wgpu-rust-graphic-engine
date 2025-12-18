use flume::{
    Sender
};
use crate::{ 
    enums::{
        signals::{
            io_thread_signal_enums::IOThreadInputSignal,
        },
    },
};

pub struct WinitEventRecorder {
    io_thread_input_channel_sender: Sender<IOThreadInputSignal>, 
}

impl WinitEventRecorder {
    pub fn new(io_thread_input_channel_sender: Sender<IOThreadInputSignal>,) -> Self {
        Self { io_thread_input_channel_sender: io_thread_input_channel_sender }
    }

    pub fn register_input_event(&self, input_event: impl Into<IOThreadInputSignal>) {
        self.io_thread_input_channel_sender.send(input_event.into());
    }
}




