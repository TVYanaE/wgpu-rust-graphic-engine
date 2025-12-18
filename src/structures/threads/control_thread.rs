use std::{
    thread::{JoinHandle, self},
};
use flume::{
    Receiver, Sender,
};
use crate::{
    structures::{
        states::{
            control_thread_state::ControlThreadState,
        },
    },
    enums::{
        signals::{
            control_thread_signal_enums::ControlThreadInputSignal,
            io_thread_signal_enums::IOThreadInputSignal,
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
        io_thread_signal_input_channel_sender: Sender<IOThreadInputSignal>,
    ) -> Self {
        let handle = thread::spawn(move ||{
            let mut control_thread_state = ControlThreadState::new();

            let mut event_buffer = Vec::new();

            loop {
                match control_thread_input_channel_receiver.recv() {
                    Ok(input_signal) => {
                        match input_signal {
                            ControlThreadInputSignal::Init => { control_thread_state.init(); },
                            ControlThreadInputSignal::LogicTick => {
                                io_thread_signal_input_channel_sender.send(IOThreadInputSignal::SendEventBuffer);
                            },
                            ControlThreadInputSignal::EventBuffer(mut income_event_buffer) => {
                                if let Some(event) = income_event_buffer.pop() {
                                    match event {
                                        Event::Shutdown => { 
                                            // TODO!: Create logic for gracefull Shutdown
                                            break; 
                                        },
                                        _ => { event_buffer.push(event); } 
                                    }
                                }
                                else {
                                    control_thread_state.run_logic(event_buffer.drain(..)); 
                                }
                            },
                            ControlThreadInputSignal::FrameTick => {
                                control_thread_state.run_drawing();
                            },
                        }
                    },
                    Err(_) => break,
                }
            }
        });

        Self { handle }
    }
}
