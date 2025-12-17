use std::{
    thread::{JoinHandle, self},
};
use flume::{
    Receiver,
};
use crate::{
    structures::{
        states::{
            control_thread_state::ControlThreadState,
        },
    },
    enums::{
        input_event_enum::InputEvent
    },
};

pub struct ControlThread {
    pub handle: JoinHandle<()>,
}

impl ControlThread { 
    pub fn start_thread(input_event_channel_receiver: Receiver<InputEvent>) -> Self {
        let handle = thread::spawn(move ||{
            let mut control_thread_state = ControlThreadState::new(input_event_channel_receiver);

            loop {
                match control_thread_state.input_event_channel_receiver.recv() {
                    Ok(input_event) => {
                        match input_event {
                            InputEvent::Shutdown => {
                                // TODO: Create logic for gracefull shutdown
                                break;
                            },
                            InputEvent::FrameStart => {
                                control_thread_state.frame_start();
                            },
                            _ => { 
                                control_thread_state.input_event_buffer.push(input_event);
                            }
                        }
                    },
                    Err(_) => break,
                }
            }
        });

        Self { handle }
    }
}
