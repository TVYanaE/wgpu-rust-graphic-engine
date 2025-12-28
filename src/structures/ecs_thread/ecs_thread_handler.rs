use std::{
    thread::{self, JoinHandle},
};
use flume::{
    Receiver,
};
use crate::{
    structures::{
        ecs_thread_signal_storage::{

        },
    },
    enums::{
        signals::{
            ecs_thread_signal_enums::ECSThreadInputSignal,
        },
    },
};



pub struct ECSThread {
    handle: JoinHandle<()>
}

impl ECSThread {
    pub fn start_thread(
        ecs_thread_input_channel_receiver: Receiver<ECSThreadInputSignal>,
        
    ) -> Self {
        let handle = thread::spawn( move ||{

            

        });

        Self { handle }
    }
}
