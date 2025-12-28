use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
};
use crate::{
    structures::{
        buses::{
            executeur_thread_message_bus::ExecuteurThreadMessageBus,
            executeur_thread_data_bus::ExecuteurThreadDataBus,
        },
        states::{
            dynamic_shared_thread_state::DynamicSharedThreadState,
        },
        task_chunk::TaskChunk,
    },
    enums::{
        execute_thread_message_enums::{
            ExecuteurThreadTaskControllerMessage,
            ExecuteurThreadTimeControllerMessage,
        },
    },
};

pub struct TaskController {
    executeur_thread_message_bus: Rc<RefCell<ExecuteurThreadMessageBus>>,
    executeur_thread_data_bus: Rc<RefCell<ExecuteurThreadDataBus>>,
    dynamic_shared_thread_state: Arc<Mutex<DynamicSharedThreadState>>,
}

impl TaskController {
    pub fn new(
        executeur_thread_message_bus: Rc<RefCell<ExecuteurThreadMessageBus>>,
        executeur_thread_data_bus: Rc<RefCell<ExecuteurThreadDataBus>>,
        dynamic_shared_thread_state: Arc<Mutex<DynamicSharedThreadState>>,
    ) -> Self {
        Self { 
            executeur_thread_message_bus: executeur_thread_message_bus,
            executeur_thread_data_bus: executeur_thread_data_bus,
            dynamic_shared_thread_state: dynamic_shared_thread_state,
        }
    }

    pub fn start(&mut self) {
        let mut schedule_ready = false;

        for message in self
            .executeur_thread_message_bus.borrow_mut()
            .drain_task_controller_message_buffer() {
            match message {
                ExecuteurThreadTaskControllerMessage::ScheduleReady => {
                    schedule_ready = true;
                }
            }
        }
        
        if !schedule_ready {
            return;
        }

        let mut dynamic_shared_thread_state_lock = self
            .dynamic_shared_thread_state
            .lock()
            .unwrap();
        
        let task_chunks: Vec<TaskChunk> = dynamic_shared_thread_state_lock.drain_schedule().collect();
        
        drop(dynamic_shared_thread_state_lock);
 
        self.executeur_thread_data_bus.borrow_mut().push_task_chunks(task_chunks.into_iter());

        self
        .executeur_thread_message_bus
        .borrow_mut()
        .push_time_controller_message_to_bus(ExecuteurThreadTimeControllerMessage::TaskChunksReady);
    }

    
}
