use std::{
    rc::Rc,
    cell::RefCell,
};

use crate::{
    structures::{
        logic_thread::{
            buses::{
                logic_thread_message_bus::LogicThreadMessageBus
            },
        },
    },
};

pub struct LogicThreadSimulationHandler {
    logic_thread_message_bus: Rc<RefCell<LogicThreadMessageBus>>
}

impl LogicThreadSimulationHandler {
    pub fn new(
        logic_thread_message_bus: Rc<RefCell<LogicThreadMessageBus>>
    ) -> Self {
        Self { 
            logic_thread_message_bus: logic_thread_message_bus,
        }
    }
}
