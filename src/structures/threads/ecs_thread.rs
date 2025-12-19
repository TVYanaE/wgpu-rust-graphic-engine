use std::{
    thread::{self, JoinHandle},
};
use flume::{
    Receiver, Sender,
};
use crate::{
    enums::{
        signals::{
            ecs_thread_signal_enums::ECSThreadSignal,
        },
    },
};



pub struct ECSThread {
    handle: JoinHandle<()>
}

impl ECSThread {
    pub fn start_thread(
        ecs_thread_input_channel_receiver: Receiver<ECSThreadSignal>,
        
    ) -> Self {
        let handle = thread::spawn( move ||{

            

        });

        Self { handle }
    }
}
