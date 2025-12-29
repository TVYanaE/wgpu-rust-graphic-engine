use std::{
    rc::Rc,
    cell::RefCell,
};

use crate::{
    structures::{
        executeur_thread::{
            states::{
                executeur_thread_time_state::ExecuteurThreadTimeState,
            },
            buses::{
                executeur_thread_message_bus::ExecuteurThreadMessageBus,
            },     
        },
    },
    enums::{
        execute_thread_message_enums::{
            ExecuteurThreadTimeManagerMessage
        },
    },
};


pub struct ExecuteurThreadTimeManager {
    executeur_time_state: Rc<RefCell<ExecuteurThreadTimeState>>,
    executeur_thread_message_bus: Rc<RefCell<ExecuteurThreadMessageBus>>,
}

impl ExecuteurThreadTimeManager {
    pub fn new(
        executeur_thread_time_state: Rc<RefCell<ExecuteurThreadTimeState>>,
        executeur_thread_message_bus: Rc<RefCell<ExecuteurThreadMessageBus>>
    ) -> Self {
        Self { 
            executeur_time_state: executeur_thread_time_state,
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
                    self.executeur_time_state.borrow_mut().logic_time_budget.refresh_avaiable_budget();
                },
                ExecuteurThreadTimeManagerMessage::FrameStart => {
                    self.executeur_time_state.borrow_mut().render_time_budget.refresh_avaiable_budget();
                },
            }
        }
    }
    
}
