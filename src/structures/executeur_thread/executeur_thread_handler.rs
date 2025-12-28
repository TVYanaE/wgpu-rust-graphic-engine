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
            executeur_thread_data_bus::ExecuteurThreadDataBus,
        },
        managers::{
            time_manager::TimeManager,
        },
        executeurs::{
            global_executeur::GlobalExecuteur,
        },
        executeur_thread_signal_storage::ExecuteurThreadSignalStorage,
        task_controller::TaskController,
        time_controller::TimeController,
        
    },
    enums::{
        signals::{
            executeur_thread_signal_enums::ExecuteurThreadInputSignal,
            ecs_thread_signal_enums::ECSThreadInputSignal,
            render_thread_signal_enums::RenderThreadInputSignal,
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
        render_thread_input_channel_sender: Sender<RenderThreadInputSignal>,
    ) -> Self {
        let handle = thread::spawn(move||{
            let executeur_thread_message_bus = Rc::new(
                RefCell::new(ExecuteurThreadMessageBus::new())
            );

            let executeur_thread_data_bus = Rc::new(
                RefCell::new(ExecuteurThreadDataBus::new())
            );

            let executeur_thread_signal_storage = ExecuteurThreadSignalStorage::new(
                executeur_thread_input_channel_receiver, 
                executeur_thread_message_bus.clone()
            ); 

            let mut task_controller = TaskController::new(
                executeur_thread_message_bus.clone(), 
                executeur_thread_data_bus.clone(), 
                dynamic_shared_thread_state
            );

            let time_state = Rc::new(RefCell::new(TimeState::default()));

            let time_manager = TimeManager::new(
                time_state.clone(), 
                executeur_thread_message_bus.clone()
            );

            let mut time_controller = TimeController::new(
                executeur_thread_message_bus.clone(), 
                executeur_thread_data_bus.clone(), 
                time_state
            ); 

            let global_executeur = GlobalExecuteur::new(
                executeur_thread_message_bus, 
                executeur_thread_data_bus,
                ecs_thread_input_channel_sender,
                render_thread_input_channel_sender,
            );

            loop {
                time_manager.start();
                executeur_thread_signal_storage.start();
                task_controller.start();
                time_controller.start();
                global_executeur.start();
            }

        }); 

        Self { handle: handle }
    }  
}
