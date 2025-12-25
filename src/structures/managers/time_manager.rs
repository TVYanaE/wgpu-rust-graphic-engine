use std::{
    rc::Rc,
    cell::RefCell,
};

use crate::{ 
    structures::{
        states::{
            time_state::TimeState,
        },
        buses::{
            executeur_thread_message_bus::ExecuteurThreadMessageBus,
        },
    },
};


pub struct TimeManager {
    time_state: Rc<RefCell<TimeState>>,
    executeur_thread_message_bus: Rc<RefCell<ExecuteurThreadMessageBus>>,
}

impl TimeManager {
    pub fn new(
        time_state: Rc<RefCell<TimeState>>,
        executeur_thread_message_bus: Rc<RefCell<ExecuteurThreadMessageBus>>
    ) -> Self {
        Self { 
            time_state: time_state,
            executeur_thread_message_bus: executeur_thread_message_bus,
        } 
    }
    
    
}
