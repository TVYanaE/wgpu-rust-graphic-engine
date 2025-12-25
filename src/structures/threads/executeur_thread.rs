use std::{
    thread::{
        self, JoinHandle,
    },
    sync::{Arc, Mutex},
    rc::Rc,
    cell::RefCell,
};
use flume::{
    Receiver, Sender,
};
use crate::{
    structures::{
        states::{
            dynamic_shared_thread_state::DynamicSharedThreadState,
            time_state::TimeState,
        }, 
        buses::{
            executeur_thread_message_bus::ExecuteurThreadMessageBus,
        },
        managers::{
            time_manager::TimeManager,
        },
        executeur_thread_signal_storage::ExecuteurThreadSignalStorage,
    },
    enums::{
        signals::{
            executeur_thread_signal_enums::ExecuteurThreadInputSignal,
            ecs_thread_signal_enums::ECSThreadInputSignal,
        },
    },
};


pub struct ExecuteurThread {
    pub handle: JoinHandle<()>,
}


impl ExecuteurThread {
    pub fn start_thread(
        executeur_thread_input_channel_receiver: Receiver<ExecuteurThreadInputSignal>,
        dynamic_shared_thread_state: Arc<Mutex<DynamicSharedThreadState>>,
        ecs_thread_input_channel_sender: Sender<ECSThreadInputSignal>,
    ) -> Self {
        let handle = thread::spawn(move||{
            let executeur_thread_message_bus = Rc::new(
                RefCell::new(ExecuteurThreadMessageBus::new())
            );

            let executeur_thread_signal_storage = ExecuteurThreadSignalStorage::new(
                executeur_thread_input_channel_receiver, 
                executeur_thread_message_bus.clone()
            ); 

            let time_state = Rc::new(RefCell::new(TimeState::default()));  

            let time_manager = TimeManager::new(
                time_state.clone(), 
                executeur_thread_message_bus
            );

            loop {
                executeur_thread_signal_storage.start();
            }

        }); 

        Self { handle: handle }
    }  
}
