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
            control_thread_message_bus::ControlThreadMessagesBus,
            io_bus::IOBus,
        },
        managers::{
            control_thread_phase_manager::ControlThreadPhaseManager,
        },  
        states::{
            phase_state::PhaseState,
        },
    },
    enums::{
        signals::{
            control_thread_signal_enums::ControlThreadInputSignal,
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
    ) -> Self {
        let handle = thread::spawn(move ||{
            let control_thread_message_bus = Rc::new(
                RefCell::new(ControlThreadMessagesBus::new())
            );

            let phase_state= Rc::new(RefCell::new(PhaseState::new()));

            let mut control_thread_phase_manager = ControlThreadPhaseManager::new(
                phase_state.clone(), 
                control_thread_input_channel_receiver, 
                control_thread_message_bus.clone()
            ); 

            //let mut control_thread_state = ControlThreadState::new();

            //let mut event_buffer = Vec::new();
            
            loop {
                control_thread_phase_manager.start().unwrap(); 
            }
        });

        Self { handle }
    }
}
