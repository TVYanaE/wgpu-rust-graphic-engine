use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
    collections::{VecDeque},
};
use crate::{
    structures::{
        buses::{
            executeur_thread_message_bus::ExecuteurThreadMessageBus,
        },
        states::{
            dynamic_shared_thread_state::DynamicSharedThreadState,
        },
        task_chunk::TaskChunk,
        task::Task,
    },
    enums::{
        execute_thread_message_enum::ExecuteThreadMessage,
        task_priority_enum::TaskPriority,
    },
};

pub struct TimeController {
    executeur_thread_message_bus: Rc<RefCell<ExecuteurThreadMessageBus>>,
    dynamic_shared_thread_state: Arc<Mutex<DynamicSharedThreadState>>,
    first_priority_task_debt: VecDeque<Task>, 
    second_priority_task_debt: VecDeque<Task>,
}

impl TimeController {
    pub fn new(
        executeur_thread_message_bus: Rc<RefCell<ExecuteurThreadMessageBus>>,
        dynamic_shared_thread_state: Arc<Mutex<DynamicSharedThreadState>>,
    ) -> Self {
        Self { 
            executeur_thread_message_bus: executeur_thread_message_bus,
            dynamic_shared_thread_state: dynamic_shared_thread_state,
            first_priority_task_debt: VecDeque::new(),
            second_priority_task_debt: VecDeque::new(),
        }
    }

    pub fn start(&mut self) {
        let messages: Vec<ExecuteThreadMessage> = self
            .executeur_thread_message_bus
            .borrow_mut()
            .drain_message_buffer()
            .collect();

        let mut task_chunks: Vec<TaskChunk> = Vec::new();

        let mut dynamic_shared_thread_state_lock = self
            .dynamic_shared_thread_state
            .lock()
            .unwrap();

        let time_menu = dynamic_shared_thread_state_lock.time_menu.get_time_menu();

        for message in messages {
            match message {
                ExecuteThreadMessage::ScheduleReady => {
                    task_chunks.extend(dynamic_shared_thread_state_lock.drain_schedule());
                },
            }
        }

        for mut task_chunk in task_chunks {
            for task in task_chunk.drain_chunk() {
                match task.task_priority {
                    TaskPriority::FirstPriority => {
                        self.first_priority_task_debt.push_back(task);
                    },
                    TaskPriority::SecondPriority => {
                        self.second_priority_task_debt.push_back(task);
                    },
                }
            } 
        }

        
        
         
    }

    
}
