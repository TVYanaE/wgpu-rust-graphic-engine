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

pub struct LogicThreadPostLogicHandler {
    logic_thread_message_bus: Rc<RefCell<LogicThreadMessageBus>>
}

impl LogicThreadPostLogicHandler {
    pub fn new(
        logic_thread_message_bus: Rc<RefCell<LogicThreadMessageBus>>
    ) -> Self {
        Self { 
            logic_thread_message_bus: logic_thread_message_bus,
        }
    }
}
