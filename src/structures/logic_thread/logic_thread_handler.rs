use std::{
    thread::{self, JoinHandle},
    rc::Rc,
    cell::RefCell,
};
use flume::{
    Receiver,
};
use crate::{
    structures::{
        logic_thread::{
            buses::{
                logic_thread_message_bus::LogicThreadMessageBus,
                logic_thread_data_bus::LogicThreadDataBus,
            },
            logic_thread_signal_dispatcher::LogicThreadSignalDispatcher,
        }, 
    },
    enums::{
        signals::{
            logic_thread_signal_enums::LogicThreadInputSignal,
        },
    },
};

// Phase order 
// External Input 
// Spawn Despawn
// Prelogic
// Simulation
// Postlogic
// Extract

pub struct LogicThreadHandler {
    handle: JoinHandle<()>
}

impl LogicThreadHandler {
    pub fn start_thread(
        logic_thread_input_channel_receiver: Receiver<LogicThreadInputSignal>,
        
    ) -> Self {
        let handle = thread::spawn( move ||{
            let logic_thread_message_bus = Rc::new(
                RefCell::new(LogicThreadMessageBus::new())
            );
            
            let logic_thread_data_bus = Rc::new(
                RefCell::new(LogicThreadDataBus::new())
            );
            
            let logic_thread_signal_dispatcher = LogicThreadSignalDispatcher::new(
                logic_thread_input_channel_receiver, 
                logic_thread_data_bus.clone(), 
                logic_thread_message_bus.clone()
            ); 
            
            loop {
                logic_thread_signal_dispatcher.start();
            }
        });

        Self { handle }
    }
}
