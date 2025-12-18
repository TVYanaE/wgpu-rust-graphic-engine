use std::{
    thread::{
        self,
        JoinHandle,
    },
};
use flume::{
    Receiver, Sender,
};
use crate::{
    enums::{
        signals::{
            io_thread_signal_enums::IOThreadInputSignal,
            control_thread_signal_enums::ControlThreadInputSignal,
        },
        event_enum::Event,
    },
};

pub struct IOThread {
    pub handle: JoinHandle<()>
}

impl IOThread {
    pub fn start_thread(
        io_thread_input_channel_receiver: Receiver<IOThreadInputSignal>,
        control_thread_input_channel_sender: Sender<ControlThreadInputSignal>,
    ) -> Self {
        let handle = thread::spawn(move ||{
            let mut event_buffer: Vec<Event> = Vec::new();

            loop {
                match io_thread_input_channel_receiver.recv() {
                    Ok(io_input_signal) => {
                        match io_input_signal {
                            IOThreadInputSignal::Destroy => { break; },
                            IOThreadInputSignal::WinitEvent(winit_event) => {
                                event_buffer.push(Event::WinitEvent(winit_event));
                            },
                            IOThreadInputSignal::Init => {
                                control_thread_input_channel_sender.send(ControlThreadInputSignal::Init);
                            },
                            IOThreadInputSignal::Shutdown => {
                                event_buffer.push(Event::Shutdown);
                            },
                            IOThreadInputSignal::SendEventBuffer => {
                                let buffer: Vec<Event> = event_buffer.drain(..).collect();
                               
                                control_thread_input_channel_sender.send(ControlThreadInputSignal::EventBuffer(buffer));
                            },
                        } 
                    },
                    Err(_) => { break; }
                }
            }
        });
        Self { handle }
    } 
}
