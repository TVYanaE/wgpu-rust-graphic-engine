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
    },
    structures::{
        recorders::{
            io_thread_recorder::IOThreadRecorder,
        },
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
            let mut io_thread_recorder = 
                IOThreadRecorder::new(
                io_thread_input_channel_receiver, 
                control_thread_input_channel_sender
            );  
            
            loop {
                if let Some(_) = io_thread_recorder.listen_input_channel(){
                    continue;
                }
                else {
                    break;
                }
            }
        });
        Self { handle }
    } 
}
