use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
};
use crate::{
    structures::{
        control_thread::{
            buses::{
                control_thread_data_bus::ControlThreadDataBus,
            }, 
        },
        main_thread::{
            states::{
                dynamic_shared_thread_state::DynamicSharedThreadState,
            },
        },
        common_structures::{
            time_menu::TimeMenu,
            task::Task,
            task_time_cost::TaskTimeCost,
        },
    },
    enums::{
        event_enums::{
            Event,
        },
        task_type_enum::TaskType,
        time_cost_type_enum::TimeCostType,
    },
};


pub struct ControlThreadTaskGenerator {
    control_thread_data_bus: Rc<RefCell<ControlThreadDataBus>>,
    dynamic_shared_thread_state: Arc<Mutex<DynamicSharedThreadState>>,
    time_menu: Arc<Mutex<TimeMenu>>,
}

impl ControlThreadTaskGenerator {
    pub fn new(
        control_thread_data_bus: Rc<RefCell<ControlThreadDataBus>>,
        dynamic_shared_thread_state: Arc<Mutex<DynamicSharedThreadState>>,
        time_menu: Arc<Mutex<TimeMenu>>,
    ) -> Self {
        Self { 
            control_thread_data_bus: control_thread_data_bus,
            dynamic_shared_thread_state: dynamic_shared_thread_state,
            time_menu: time_menu,
        }
    }

    pub fn start(&self) {
        let mut data_bus = self.control_thread_data_bus.borrow_mut();

        let mut tasks: Vec<Task> = Vec::new();
      
        let time_menu = self.time_menu.lock().unwrap().get_time_menu();
        
        for event in data_bus.drain_event_buffer() {
            match event {
                Event::LogicCalculation => {
                    let time_cost = *time_menu.get(&TaskType::LogicCalculation).unwrap();

                    let task = Task {
                        task_type: TaskType::LogicCalculation, 
                        task_time_cost: TaskTimeCost { 
                            time_cost_type: TimeCostType::LogicTimeCost, 
                            time_cost: time_cost
                        }
                    };

                    tasks.push(task);
                },
                Event::PrepareRenderState => {
                    let time_cost = *time_menu.get(&TaskType::PrepareRenderState).unwrap();

                    let task = Task {
                        task_type: TaskType::PrepareRenderState, 
                        task_time_cost: TaskTimeCost { 
                            time_cost_type: TimeCostType::LogicTimeCost, 
                            time_cost: time_cost
                        }
                    };

                    tasks.push(task);
                },
                Event::DrawRenderState => {
                    let time_cost = *time_menu.get(&TaskType::DrawRenderState).unwrap();

                    let task = Task {
                        task_type: TaskType::DrawRenderState, 
                        task_time_cost: TaskTimeCost { 
                            time_cost_type: TimeCostType::LogicTimeCost, 
                            time_cost: time_cost
                        }
                    };

                    tasks.push(task);
                },
                Event::Resize(physical_size) => {
                    let time_cost = *time_menu.get(&TaskType::Resize).unwrap();

                    let task = Task {
                        task_type: TaskType::Resize, 
                        task_time_cost: TaskTimeCost { 
                            time_cost_type: TimeCostType::LogicTimeCost, 
                            time_cost: time_cost
                        }
                    };

                    tasks.push(task);
                    self.dynamic_shared_thread_state.lock().unwrap().set_physical_size(physical_size);
                },
                Event::UnknownEvent => {},
            }
        } 
        data_bus.push_tasks(tasks.into_iter()); 
    }
}
