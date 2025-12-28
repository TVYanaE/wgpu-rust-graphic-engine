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
    enums::{
        execute_thread_message_enums::{
            ExecuteurThreadTimeManagerMessage
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
   
    pub fn start(&self) { 
        for message in self
            .executeur_thread_message_bus
            .borrow_mut()
            .drain_time_manager_message_buffer() {
            match message {
                ExecuteurThreadTimeManagerMessage::LogicStart => {
                    self.time_state.borrow_mut().logic_time_budget.refresh_avaiable_budget();
                },
                ExecuteurThreadTimeManagerMessage::FrameStart => {
                    self.time_state.borrow_mut().render_time_budget.refresh_avaiable_budget();
                },
            }
        }
    }
    
}
