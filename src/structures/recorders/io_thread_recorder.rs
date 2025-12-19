#![allow(dead_code)]
use flume::{
    Receiver, Sender,
};
use crate::{
    enums::{
        signals::{
            io_thread_signal_enums::IOThreadInputSignal,
            control_thread_signal_enums::ControlThreadInputSignal,
        },
    },
    structures::{
        buses::{
            io_thread_bus::IOThreadBus,
        },
    },
};

pub struct IOThreadRecorder {
    io_thread_input_channel_receiver: Receiver<IOThreadInputSignal>,
    control_thread_input_channel_sender: Sender<ControlThreadInputSignal>,
    io_thread_bus: IOThreadBus,
} 

impl IOThreadRecorder {
    pub fn new(
        io_thread_input_channel_receiver: Receiver<IOThreadInputSignal>,
        control_thread_input_channel_sender: Sender<ControlThreadInputSignal>,
    ) -> Self {
        Self { 
            io_thread_input_channel_receiver: io_thread_input_channel_receiver,
            control_thread_input_channel_sender: control_thread_input_channel_sender,
            io_thread_bus: IOThreadBus::new(),
        }
    } 
}
