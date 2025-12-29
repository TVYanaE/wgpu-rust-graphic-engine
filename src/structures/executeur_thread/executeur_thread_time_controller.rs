use std::{
    rc::Rc,
    cell::RefCell,
    collections::{VecDeque},
    time::Duration,
};
use crate::{
    structures::{
        executeur_thread::{
            buses::{
                executeur_thread_data_bus::ExecuteurThreadDataBus,
                executeur_thread_message_bus::ExecuteurThreadMessageBus,
            },
        states::{
                executeur_thread_time_state::ExecuteurThreadTimeState,
            },
        },
        common_structures::{
            task_chunk::TaskChunk,
        },
    },
    enums::{
        execute_thread_message_enums::{
            ExecuteurThreadTimeControllerMessage,
            ExecuteurThreadGlobalExecuteurMessage,
        },
        time_cost_type_enum::TimeCostType,
    },
};

pub struct ExecuteurThreadTimeController {
    executeur_thread_message_bus: Rc<RefCell<ExecuteurThreadMessageBus>>,
    executeur_thread_data_bus: Rc<RefCell<ExecuteurThreadDataBus>>,
    executeur_thread_time_state: Rc<RefCell<ExecuteurThreadTimeState>>,
    task_chunk_debt: VecDeque<TaskChunk>,
    avaiable_render_time_budget: Duration,
    avaiable_logic_time_budget: Duration,
}

impl ExecuteurThreadTimeController {
    pub fn new(
        executeur_thread_message_bus: Rc<RefCell<ExecuteurThreadMessageBus>>,
        executeur_thread_data_bus: Rc<RefCell<ExecuteurThreadDataBus>>,
        executeur_thread_time_state: Rc<RefCell<ExecuteurThreadTimeState>>,
        
    ) -> Self {
        Self { 
            executeur_thread_message_bus: executeur_thread_message_bus, 
            executeur_thread_data_bus: executeur_thread_data_bus,
            executeur_thread_time_state: executeur_thread_time_state,
            task_chunk_debt: VecDeque::new(),
            avaiable_render_time_budget: Duration::ZERO,
            avaiable_logic_time_budget: Duration::ZERO,
        }
    }

    pub fn start(&mut self) {
        let mut task_chunks_ready = false;  

        for message in self
            .executeur_thread_message_bus
            .borrow_mut()
            .drain_time_controller_message_buffer() {
            match message {
                ExecuteurThreadTimeControllerMessage::TaskChunksReady => {
                    task_chunks_ready = true;
                },
            }
        }

        if !task_chunks_ready {
            return;
        }

        for task_chunk in self.executeur_thread_data_bus.borrow_mut().drain_task_chunk_buffer() {
            self.task_chunk_debt.push_back(task_chunk);
        }

        if let Some(new_render_time_budget) = self
            .executeur_thread_time_state
            .borrow_mut()
            .render_time_budget
            .get_avaiable_budget() {
            self.avaiable_render_time_budget = new_render_time_budget;
        }

        if let Some(new_logic_time_budget) = self
            .executeur_thread_time_state
            .borrow_mut()
            .logic_time_budget
            .get_avaiable_budget() {
            self.avaiable_logic_time_budget = new_logic_time_budget;
        }
        
        let mut job_list = Vec::new();

        let mut debt_over = true;
        let mut reserv_task_chunk = TaskChunk::new(); 

        while let Some(task_chunk) = self.task_chunk_debt.pop_front() {
            let task_chunk_time_cost = task_chunk.get_time_cost().unwrap();

            match task_chunk_time_cost.time_cost_type {
                TimeCostType::LogicTimeCost => {
                    if task_chunk_time_cost.time_cost < self.avaiable_logic_time_budget {
                        job_list.push(task_chunk);
                        self.avaiable_logic_time_budget -= task_chunk_time_cost.time_cost;
                    }
                    else {
                        reserv_task_chunk = task_chunk;
                        debt_over = false;
                        break;
                    }
                },
                TimeCostType::RenderTimeCost => {
                    if task_chunk_time_cost.time_cost < self.avaiable_render_time_budget {
                        job_list.push(task_chunk);
                        self.avaiable_logic_time_budget -= task_chunk_time_cost.time_cost;
                    }
                    else {
                        reserv_task_chunk = task_chunk;
                        debt_over = false;
                        break;
                    }
                },
            }
        }

        if !debt_over {
            self.task_chunk_debt.push_front(reserv_task_chunk);
        }
         
        self
        .executeur_thread_message_bus
        .borrow_mut()
        .push_global_executeur_message_to_bus(ExecuteurThreadGlobalExecuteurMessage::JobListReady); 
        
        self
        .executeur_thread_data_bus
        .borrow_mut()
        .push_job_list(job_list.into_iter());
    }
}
