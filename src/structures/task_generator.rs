use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
    time::Duration,
};
use winit::{
    event::WindowEvent,
};
use crate::{
    structures::{
        buses::{
            control_thread_data_bus::ControlThreadDataBus,
        },
        states::{
            phase_state::PhaseState, 
            dynamic_shared_thread_state::DynamicSharedThreadState,
        },
        task::Task,
        task_time_cost::TaskTimeCost,
        time_menu::TimeMenu,
    },
    enums::{
        phase_enum::Phase,
        task_type_enum::TaskType,
        time_cost_type_enum::TimeCostType,
        task_priority_enum::TaskPriority,
        io_event_enum::IOEvent,
        winit_event_enum::WinitEvent,
    },
};


pub struct TaskGenerator {
    control_thread_data_bus: Rc<RefCell<ControlThreadDataBus>>,
    phase_state: Rc<RefCell<PhaseState>>,
    dynamic_shared_thread_state: Arc<Mutex<DynamicSharedThreadState>>,
    time_menu: Arc<Mutex<TimeMenu>>,
}

impl TaskGenerator {
    pub fn new(
        control_thread_data_bus: Rc<RefCell<ControlThreadDataBus>>,
        phase_state: Rc<RefCell<PhaseState>>,
        dynamic_shared_thread_state: Arc<Mutex<DynamicSharedThreadState>>,
        time_menu: Arc<Mutex<TimeMenu>>,
    ) -> Self {
        Self { 
            control_thread_data_bus: control_thread_data_bus,
            phase_state: phase_state,
            dynamic_shared_thread_state: dynamic_shared_thread_state,
            time_menu: time_menu,
        }
    }

    pub fn start(&self) {
        let phase_state = self.phase_state.borrow_mut();
        let mut data_bus = self.control_thread_data_bus.borrow_mut();

        let current_phase = phase_state.get_current_phase();

        let mut tasks: Vec<Task> = Vec::new();
      
        let time_menu = self.time_menu.lock().unwrap().get_time_menu();

        match current_phase { 
            Phase::InitPhase => {
                let task_time_cost = *time_menu.get(&TaskType::Init).unwrap();

                let task = Task {
                    task_type: TaskType::Init,
                    task_time_cost: TaskTimeCost { 
                        time_cost_type: TimeCostType::LogicTimeCost,
                        time_cost: task_time_cost, 
                    },
                    phase: Phase::InitPhase,
                    task_priority: TaskPriority::FirstPriority, 
                };

                tasks.push(task); 
            },
            Phase::ShutdownPhase => {
                let task_time_cost = *time_menu.get(&TaskType::Shutdown).unwrap();

                let task = Task {
                    task_type: TaskType::Shutdown,
                    task_time_cost: TaskTimeCost { 
                        time_cost_type: TimeCostType::LogicTimeCost, 
                        time_cost: task_time_cost, 
                    },
                    phase: Phase::ShutdownPhase,
                    task_priority: TaskPriority::FirstPriority,
                };

                tasks.push(task);
            },
            Phase::UpdatePhase => {
                let task1_time_cost = *time_menu.get(&TaskType::LogicCalculation).unwrap();
                let task2_time_cost = *time_menu.get(&TaskType::PrepareRenderState).unwrap();

                let task1 = Task {
                    task_type: TaskType::LogicCalculation,
                    task_time_cost: TaskTimeCost { 
                        time_cost_type: TimeCostType::LogicTimeCost,
                        time_cost: task1_time_cost, 
                    },
                    phase: Phase::UpdatePhase,
                    task_priority: TaskPriority::FirstPriority,
                };

                let task2 = Task {
                    task_type: TaskType::PrepareRenderState,
                    task_time_cost: TaskTimeCost { 
                        time_cost_type: TimeCostType::LogicTimeCost,
                        time_cost:task2_time_cost, 
                    },
                    phase: Phase::UpdatePhase,
                    task_priority: TaskPriority::FirstPriority,
                };

                tasks.push(task1);
                tasks.push(task2);
            },
            Phase::RenderPhase => {
                let task_time_cost = *time_menu.get(&TaskType::DrawRenderState).unwrap();

                let task = Task {
                    task_type: TaskType::DrawRenderState,
                    task_time_cost: TaskTimeCost { 
                        time_cost_type: TimeCostType::RenderTimeCost,
                        time_cost: task_time_cost, 
                    },
                    phase: Phase::RenderPhase,
                    task_priority: TaskPriority::FirstPriority,
                };

                tasks.push(task); 
            },
            Phase::ExternalEventsPhase => {
                let mut data_bus = self.control_thread_data_bus.borrow_mut();
                let mut dynamic_shared_thread_state_lock = self
                    .dynamic_shared_thread_state
                    .lock()
                    .unwrap();

                let io_events: Vec<IOEvent> = data_bus.drain_io_event_queue().collect();

                for io_event in io_events {
                    match io_event {
                        IOEvent::WinitEvent(winit_event) => {
                            match winit_event {
                                WinitEvent::WindowEvent(window_event) => {
                                    match window_event {
                                        WindowEvent::Resized(physical_size) => {
                                            let task_time_cost = *time_menu.get(&TaskType::Resize).unwrap();

                                            let task = Task {
                                                task_type: TaskType::Resize,
                                                task_time_cost: TaskTimeCost { 
                                                    time_cost_type: TimeCostType::LogicTimeCost, 
                                                    time_cost: task_time_cost, 
                                                },
                                                phase: Phase::ExternalEventsPhase,
                                                task_priority: TaskPriority::FirstPriority,
                                            };

                                            tasks.push(task);
                                            dynamic_shared_thread_state_lock.set_physical_size(physical_size);
                                        },
                                        _ => {},
                                    }
                                },
                            }
                        },
                    }
                } 
            },
            Phase::Idle => {
                // Do nothing 
            },
        }

        data_bus.push_tasks(tasks.into_iter()); 
    }
}
