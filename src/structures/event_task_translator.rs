use std::{
    rc::Rc,
    cell::RefCell,
};

use crate::{
    structures::{
        buses::{
            control_thread_message_bus::ControlThreadMessagesBus,
            control_thread_data_bus::ControlThreadDataBus,
        },
    },
};


pub struct EventTaskTranslator {
    control_thread_message_bus_ref: Rc<RefCell<ControlThreadMessagesBus>>,
    control_thread_data_bus_ref: Rc<RefCell<ControlThreadDataBus>>,
}


impl EventTaskTranslator {
    pub fn new(
        control_thread_message_bus_ref: Rc<RefCell<ControlThreadMessagesBus>>,
        control_thread_data_bus_ref: Rc<RefCell<ControlThreadDataBus>>,
    ) -> Self {
        Self { 
            control_thread_message_bus_ref: control_thread_message_bus_ref, 
            control_thread_data_bus_ref: control_thread_data_bus_ref, 
        }
    }

    pub fn translate_buses(&self) {

    };
}
