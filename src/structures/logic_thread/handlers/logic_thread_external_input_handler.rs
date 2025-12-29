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

pub struct LogicThreadExternalInputHandler {
    logic_thread_message_bus: Rc<RefCell<LogicThreadMessageBus>>
} 

impl LogicThreadExternalInputHandler {
    pub fn new(
        logic_thread_message_bus: Rc<RefCell<LogicThreadMessageBus>>
    ) -> Self {
        Self { 
            logic_thread_message_bus: logic_thread_message_bus,
        }
    }
}
