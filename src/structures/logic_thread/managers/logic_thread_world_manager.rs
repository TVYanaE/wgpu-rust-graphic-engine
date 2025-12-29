use std::{
    rc::Rc,
    cell::RefCell,
};
use shipyard::{
    World,
};
use crate::{
    structures::{
        logic_thread::{
            buses::{
                logic_thread_message_bus::LogicThreadMessageBus,
            },
        }, 
    },
    enums::{
        messages::{
            logic_thread_message_enums::{
                LogicThreadWorldManagerMessage
            },
        }, 
    },
};


pub struct LogicThreadWorldManager {
    logic_thread_message_bus: Rc<RefCell<LogicThreadMessageBus>>, 
    world: Option<World>,
}

impl LogicThreadWorldManager {
    pub fn new(
        logic_thread_message_bus: Rc<RefCell<LogicThreadMessageBus>>
    ) -> Self {
        Self { 
            logic_thread_message_bus: logic_thread_message_bus,
            world: None,
        }
    }

    pub fn start(&mut self) {
        for message in self
            .logic_thread_message_bus
            .borrow_mut()
            .drain_world_manager_message_buffer() {
            match message {
                LogicThreadWorldManagerMessage::Init => {
                    
                }   
            }
        }
    }
}
