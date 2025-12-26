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
        buses::{
            control_thread_data_bus::ControlThreadDataBus,
            io_bus::IOBus,
        },
        managers::{
            control_thread_phase_manager::ControlThreadPhaseManager,
            control_thread_io_event_manager::ControlThreadIOEventManager,
        },  
        states::{
            phase_state::PhaseState,
            dynamic_shared_thread_state::DynamicSharedThreadState,
        },
        control_thread_signal_storage::ControlThreadSignalStorage,
        task_generator::TaskGenerator,
        scheduler::Scheduler,
        time_menu::TimeMenu,
    },
    enums::{
        signals::{
            control_thread_signal_enums::ControlThreadInputSignal,
            executeur_thread_signal_enums::ExecuteurThreadInputSignal,
        },
    },
};

pub struct ControlThread {
    pub handle: JoinHandle<()>,
}

impl ControlThread { 
    pub fn start_thread(
        control_thread_input_channel_receiver: Receiver<ControlThreadInputSignal>,
        io_bus: Arc<Mutex<IOBus>>,
        dynamic_shared_thread_state: Arc<Mutex<DynamicSharedThreadState>>,
        executeur_thread_input_channel_sender: Sender<ExecuteurThreadInputSignal>,
        time_menu: Arc<Mutex<TimeMenu>>,
    ) -> Self {
        let handle = thread::spawn(move ||{

            let control_thread_data_bus = Rc::new(
                RefCell::new(ControlThreadDataBus::new())
            );

            let phase_state= Rc::new(RefCell::new(PhaseState::new()));

            let control_thread_signal_storage = Rc::new(
                RefCell::new(ControlThreadSignalStorage::new(control_thread_input_channel_receiver))
            ); 

            let control_thread_io_event_manager = Rc::new(
                RefCell::new(ControlThreadIOEventManager::new(io_bus, control_thread_data_bus.clone()))
            );

            let mut control_thread_phase_manager = ControlThreadPhaseManager::new(
                phase_state.clone(),
                control_thread_io_event_manager,
                control_thread_signal_storage.clone(),
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
                control_thread_signal_storage.borrow_mut().start().unwrap();
                control_thread_phase_manager.start();
                task_generator.start();
                scheduler.start();
            }
        });

        Self { handle }
    }
}
