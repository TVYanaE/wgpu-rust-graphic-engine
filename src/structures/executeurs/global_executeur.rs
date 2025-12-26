use std::{
    rc::Rc,
    cell::RefCell
};

use crate::{
    structures::{
        buses::{
            executeur_thread_message_bus::ExecuteurThreadMessageBus,
            executeur_thread_data_bus::ExecuteurThreadDataBus,
        },
    },
    enums::{
        execute_thread_message_enums::{
            ExecuteurThreadGlobalExecuteurMessage
        },
    },
};

pub struct GlobalExecuteur {
    executeur_thread_message_bus: Rc<RefCell<ExecuteurThreadMessageBus>>,
    executeur_thread_data_bus: Rc<RefCell<ExecuteurThreadDataBus>>,
}

impl GlobalExecuteur {
    pub fn new(
        executeur_thread_message_bus: Rc<RefCell<ExecuteurThreadMessageBus>>,
        executeur_thread_data_bus: Rc<RefCell<ExecuteurThreadDataBus>>,
    ) -> Self {
        Self {
            executeur_thread_message_bus, 
            executeur_thread_data_bus, 
        }
    }

    pub fn start(&self) {}
}
