use std::{
    thread::{JoinHandle, self},
    rc::Rc,
    cell::RefCell,
    sync::{Arc, Mutex},
};
use flume::{
    Receiver, Sender,
};
use crate::{
    structures::{
        control_thread::{
            buses::{
                control_thread_data_bus::ControlThreadDataBus,
                control_thread_message_bus::ControlThreadMessagesBus
            },
            managers::{
                control_thread_phase_manager::ControlThreadPhaseManager,
                control_thread_scene_manager::ControlThreadSceneManager,
                control_thread_io_event_manager::ControlThreadIOEventManager,
                control_thread_request_manager::ControlThreadRequestManager,
            },
            states::{
                control_thread_phase_state::ControlThreadPhaseState,
                control_thread_request_state::ControlThreadRequestState,
                control_thread_scene_state::ControlThreadSceneState,
            }, 
            control_thread_scheduler::ControlThreadScheduler,
            control_thread_signal_dispatcher::ControlThreadSignalDispatcher,
            control_thread_task_generator::ControlThreadTaskGenerator,
        },
        main_thread::{
            io_bus::IOBus,
            states::{
                dynamic_shared_thread_state::DynamicSharedThreadState,
            },
        },
    },
    enums::{
        signals::{
            control_thread_signal_enums::ControlThreadInputSignal,
            executeur_thread_signal_enums::ExecuteurThreadInputSignal,
        },
    },
};

pub struct ControlThreadHandler {
    pub handle: JoinHandle<()>,
}

impl ControlThreadHandler { 
    pub fn start_thread(
        control_thread_input_channel_receiver: Receiver<ControlThreadInputSignal>,
        io_bus: Arc<Mutex<IOBus>>,
        dynamic_shared_thread_state: Arc<Mutex<DynamicSharedThreadState>>,
        executeur_thread_input_channel_sender: Sender<ExecuteurThreadInputSignal>,
    ) -> Self {
        let handle = thread::spawn(move ||{

            let control_thread_data_bus = Rc::new(
                RefCell::new(ControlThreadDataBus::new())
            );

            let control_thread_message_bus = Rc::new(
                RefCell::new(ControlThreadMessagesBus::new())
            ); 


            let control_thread_signal_dispatcher = ControlThreadSignalDispatcher::new(
                control_thread_input_channel_receiver, 
                control_thread_message_bus.clone()
            ); 

            let control_thread_scene_state = Rc::new(
                RefCell::new(ControlThreadSceneState::new())
            );

            let control_thread_scene_manager = ControlThreadSceneManager::new(
                control_thread_message_bus.clone(), 
                control_thread_scene_state.clone()
            );

            let control_thread_request_state = Rc::new(
                RefCell::new(ControlThreadRequestState::new())
            );
           
            let control_thread_request_manager = ControlThreadRequestManager::new(
                control_thread_message_bus.clone(), 
                control_thread_request_state.clone()
            );

            let control_thread_io_event_manager = Rc::new(
                RefCell::new(ControlThreadIOEventManager::new(io_bus, control_thread_data_bus.clone()))
            );

            let control_thread_phase_state = Rc::new(
                RefCell::new(ControlThreadPhaseState::new())
            );

            let mut control_thread_phase_manager = ControlThreadPhaseManager::new(
                control_thread_request_state.clone(),
                control_thread_scene_state.clone(),
                control_thread_phase_state.clone(),
                control_thread_io_event_manager,
            );  

            let task_generator = TaskGenerator::new(
                control_thread_data_bus.clone(), 
                phase_state,
                dynamic_shared_thread_state.clone(),
                time_menu
            ); 

            let mut scheduler = Scheduler::new(
                control_thread_data_bus,
                dynamic_shared_thread_state,
                executeur_thread_input_channel_sender
            ); 
            
            loop {
                control_thread_signal_dispatcher.start();
                control_thread_scene_manager.start();
                control_thread_request_manager.start();
                control_thread_phase_manager.start();


                task_generator.start();
                scheduler.start();
            }
        });

        Self { handle }
    }
}
