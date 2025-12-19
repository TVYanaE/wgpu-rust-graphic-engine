use flume::{
    Sender
};
use crate::{ 
    enums::{
        signals::{
            control_thread_signal_enums::ControlThreadInputSignal,
        },
    },
};

pub struct WinitEventRecorder {
    control_thread_input_channel_sender: Sender<ControlThreadInputSignal>, 
}

impl WinitEventRecorder {
    pub fn new(control_thread_input_channel_sender: Sender<ControlThreadInputSignal>,) -> Self {
        Self { control_thread_input_channel_sender: control_thread_input_channel_sender }
    }

    pub fn register_input_event(&self, input_event: impl Into<ControlThreadInputSignal>) {
        self.control_thread_input_channel_sender.send(input_event.into());
    }
}




