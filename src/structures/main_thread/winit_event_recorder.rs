use flume::{
    Sender
};
use crate::{ 
    enums::{
        signals::{
            io_thread_signal_enums::IOThreadInputSignal
        },
        winit_event_enum::WinitEvent,
    },
};

pub struct WinitEventRecorder {
    io_thread_input_channel_sender: Sender<IOThreadInputSignal>, 
}

impl WinitEventRecorder {
    pub fn new(io_thread_input_channel_sender: Sender<IOThreadInputSignal>,) -> Self {
        Self { io_thread_input_channel_sender: io_thread_input_channel_sender }
    }

    pub fn register_input_event(&self, winit_event: impl Into<WinitEvent>) {
        self.io_thread_input_channel_sender.send(IOThreadInputSignal::WinitEvent(winit_event.into()));
    }
}




