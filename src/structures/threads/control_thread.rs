use std::{
    thread::{JoinHandle, self},
    rc::Rc,
    cell::RefCell,
};
use flume::{
    Receiver, Sender,
};
use crate::{
    structures::{
        states::{
            control_thread_state::ControlThreadState,
        },
        buses::{
            control_thread_message_bus::ControlThreadMessagesBus,
            control_thread_data_bus::ControlThreadDataBus,
        },
        recorders::{
            control_thread_recorder::ControlThreadRecorder,
        },
    },
    enums::{
        signals::{
            control_thread_signal_enums::ControlThreadInputSignal,
        },
        event_enum::Event,
    },
};

pub struct ControlThread {
    pub handle: JoinHandle<()>,
}

impl ControlThread { 
    pub fn start_thread(
        control_thread_input_channel_receiver: Receiver<ControlThreadInputSignal>,
    ) -> Self {
        let handle = thread::spawn(move ||{
            let control_thread_message_bus = Rc::new(
                RefCell::new(ControlThreadMessagesBus::new())
            );

            let control_thread_data_bus = Rc::new(
                RefCell::new(ControlThreadDataBus::new())
            );

            let mut control_thread_recorder = ControlThreadRecorder::new(
                control_thread_input_channel_receiver, 
                control_thread_message_bus.clone(), 
                control_thread_data_bus.clone()
            );

            //let mut control_thread_state = ControlThreadState::new();

            //let mut event_buffer = Vec::new();
            
            let mut is_shutdown_signal_received = false;

            loop {
                if let None = control_thread_recorder.listen_input_channel() {
                    is_shutdown_signal_received = true; 
                }

                
                if is_shutdown_signal_received {
                    break;
                }
            }
        });

        Self { handle }
    }
}
