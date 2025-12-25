use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
};
use flume::{
    Sender
};
use crate::{
    structures::{
        buses::{
            control_thread_data_bus::ControlThreadDataBus,
        },
        task::Task,
        task_chunk::{TaskChunk},
        states::{
            dynamic_shared_thread_state::DynamicSharedThreadState,
        },
    },
    enums::{
        signals::{
            executeur_thread_signal_enums::ExecuteurThreadInputSignal,
        },
    },
};

pub struct Scheduler {  
    control_thread_data_bus: Rc<RefCell<ControlThreadDataBus>>, 
    dynamic_shared_thread_state: Arc<Mutex<DynamicSharedThreadState>>,
    executeur_thread_input_channel_sender: Sender<ExecuteurThreadInputSignal>,
}

impl Scheduler {
    pub fn new(
        control_thread_data_bus: Rc<RefCell<ControlThreadDataBus>>,
        dynamic_shared_thread_state: Arc<Mutex<DynamicSharedThreadState>>,
        executeur_thread_input_channel_sender: Sender<ExecuteurThreadInputSignal>
    ) -> Self {
        Self {  
            control_thread_data_bus: control_thread_data_bus,
            dynamic_shared_thread_state: dynamic_shared_thread_state,
            executeur_thread_input_channel_sender: executeur_thread_input_channel_sender,
        }
    } 

    pub fn start(&mut self){
        let mut data_bus = self.control_thread_data_bus.borrow_mut();

        let tasks: Vec<Task> = data_bus.drain_task_queue().collect();

        let mut schedule: Vec<TaskChunk> = Vec::new(); 

        for task in tasks {
            let mut inserted = false;

            for task_chunk in schedule.iter_mut() {
                if task_chunk.try_insert_task(task) {
                    inserted = true;
                    break;
                } 
            }

            if !inserted {
                let mut new_task_chunk = TaskChunk::new();
                new_task_chunk.try_insert_task(task);

                schedule.push(new_task_chunk);
            }
        }
        
        let mut lock_dynamic_shared_thread_state = self
            .dynamic_shared_thread_state
            .lock()
            .unwrap();

        lock_dynamic_shared_thread_state.set_schedule(schedule.into_iter());
        self.executeur_thread_input_channel_sender.send(ExecuteurThreadInputSignal::ScheduleReady);
    }
    
    
}
