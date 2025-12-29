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
    enums::signals::{
        logic_thread_signal_enums::LogicThreadInputSignal, 
        executeur_thread_signal_enums::ExecuteurThreadInputSignal, 
        render_thread_signal_enums::RenderThreadInputSignal
    }, 
    structures::{
        executeur_thread::{
            buses::{
                executeur_thread_data_bus::ExecuteurThreadDataBus, 
                executeur_thread_message_bus::ExecuteurThreadMessageBus
            },
            states::{
                executeur_thread_time_state::ExecuteurThreadTimeState,
            },
            managers::{
                executeur_thread_time_manager::ExecuteurThreadTimeManager,
            },
            executeur_thread_signal_dispatcher::ExecuteurThreadSignalDispatcher,
            executeur_thread_task_controller::ExecuteurThreadTaskController, 
            executeur_thread_time_controller::ExecuteurThreadTimeController,
            executeur_thread_global_executeur::ExecuteurThreadGlobalExecuteur,
        },  
        main_thread::{
            states::{
                dynamic_shared_thread_state::DynamicSharedThreadState,
            },
        },
    }
};


pub struct ExecuteurThread {
    pub handle: JoinHandle<()>,
}


impl ExecuteurThread {
    pub fn start_thread(
        executeur_thread_input_channel_receiver: Receiver<ExecuteurThreadInputSignal>,
        dynamic_shared_thread_state: Arc<Mutex<DynamicSharedThreadState>>,
        logic_thread_input_channel_sender: Sender<LogicThreadInputSignal>,
        render_thread_input_channel_sender: Sender<RenderThreadInputSignal>,
    ) -> Self {
        let handle = thread::spawn(move||{
            let executeur_thread_message_bus = Rc::new(
                RefCell::new(ExecuteurThreadMessageBus::new())
            );

            let executeur_thread_data_bus = Rc::new(
                RefCell::new(ExecuteurThreadDataBus::new())
            );

            let executeur_thread_signal_dispatcher = ExecuteurThreadSignalDispatcher::new(
                executeur_thread_input_channel_receiver, 
                executeur_thread_message_bus.clone()
            ); 
            
            let executeur_thread_time_state = Rc::new(
                RefCell::new(ExecuteurThreadTimeState::default())
            );

            let executeur_thread_time_manager = ExecuteurThreadTimeManager::new(
                executeur_thread_time_state.clone(), 
                executeur_thread_message_bus.clone()
            );

            let executeur_thread_task_controller = ExecuteurThreadTaskController::new(
                executeur_thread_message_bus.clone(), 
                executeur_thread_data_bus.clone(), 
                dynamic_shared_thread_state
            );
            

            let mut executeur_thread_time_controller = ExecuteurThreadTimeController::new(
                executeur_thread_message_bus.clone(), 
                executeur_thread_data_bus.clone(), 
                executeur_thread_time_state
            ); 

            let executeur_thread_global_executeur = ExecuteurThreadGlobalExecuteur::new(
                executeur_thread_message_bus, 
                executeur_thread_data_bus,
                logic_thread_input_channel_sender,
                render_thread_input_channel_sender,
            );

            loop {
                executeur_thread_signal_dispatcher.start();
                executeur_thread_time_manager.start();
                executeur_thread_task_controller.start();
                executeur_thread_time_controller.start();
                executeur_thread_global_executeur.start();
            }

        }); 

        Self { handle: handle }
    }  
}
