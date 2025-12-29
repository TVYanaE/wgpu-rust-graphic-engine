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

pub struct LogicThreadSpawnDespawnHandler {
    logic_thread_message_bus: Rc<RefCell<LogicThreadMessageBus>>
}

impl LogicThreadSpawnDespawnHandler {
    pub fn new(
        logic_thread_message_bus: Rc<RefCell<LogicThreadMessageBus>>
    ) -> Self {
        Self { 
            logic_thread_message_bus: logic_thread_message_bus,
        }
    }
}
