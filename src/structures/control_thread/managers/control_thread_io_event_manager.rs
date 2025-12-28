use std::{
    sync::{Arc, Mutex},
    rc::Rc,
    cell::RefCell,
};

use crate::{
    structures::{
        main_thread::{
            io_bus::IOBus
        }, 
        control_thread::{
            buses::{
                control_thread_data_bus::ControlThreadDataBus,
            },
        },
    },
    enums::{
        io_event_enum::IOEvent,
    },
};



pub struct ControlThreadIOEventManager {
    io_bus: Arc<Mutex<IOBus>>,
    control_thread_data_bus: Rc<RefCell<ControlThreadDataBus>>,
}


impl ControlThreadIOEventManager {
    pub fn new(
        io_bus: Arc<Mutex<IOBus>>,
        control_thread_data_bus: Rc<RefCell<ControlThreadDataBus>>,
    ) -> Self {
        Self { 
            io_bus: io_bus,
            control_thread_data_bus: control_thread_data_bus,
        }
    }

    /// Returns True if there are IO events
    /// If True has been returned IO events will beeing pushed to Data bus
    pub fn check_io_events(&mut self) -> bool {
        let mut io_bus_lock = self.io_bus.lock().unwrap();

        let io_events: Vec<IOEvent> = io_bus_lock.drain().collect();

        if io_events.is_empty() {
            return false;
        }
        else {
            let mut data_bus = self.control_thread_data_bus.borrow_mut();

            data_bus.push_io_events(io_events.into_iter());

            return true;
        }
    } 
}
