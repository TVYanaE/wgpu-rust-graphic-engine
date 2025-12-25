use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex},
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
    },
    enums::{
        phase_enum::Phase,
        task_type_enum::TaskType,
        task_priority_enum::TaskPriority,
        io_event_enum::IOEvent,
        winit_event_enum::WinitEvent,
    },
};


pub struct TaskGenerator {
    control_thread_data_bus: Rc<RefCell<ControlThreadDataBus>>,
    phase_state: Rc<RefCell<PhaseState>>,
    dynamic_shared_thread_state: Arc<Mutex<DynamicSharedThreadState>>,
}

impl TaskGenerator {
    pub fn new(
        control_thread_data_bus: Rc<RefCell<ControlThreadDataBus>>,
        phase_state: Rc<RefCell<PhaseState>>,
        dynamic_shared_thread_state: Arc<Mutex<DynamicSharedThreadState>>
    ) -> Self {
        Self { 
            control_thread_data_bus: control_thread_data_bus,
            phase_state: phase_state,
            dynamic_shared_thread_state: dynamic_shared_thread_state,
        }
    }

    pub fn start(&self) {
        let phase_state = self.phase_state.borrow_mut();
        let mut data_bus = self.control_thread_data_bus.borrow_mut();

        let current_phase = phase_state.get_current_phase();

        let mut tasks: Vec<Task> = Vec::new();
       
        match current_phase { 
            Phase::InitPhase => {
                let task = Task {
                    task_type: TaskType::Init,
                    phase: Phase::InitPhase,
                    task_priority: TaskPriority::FirstPriority, 
                };

                tasks.push(task); 
            },
            Phase::ShutdownPhase => {
                let task = Task {
                    task_type: TaskType::Shutdown,
                    phase: Phase::ShutdownPhase,
                    task_priority: TaskPriority::FirstPriority,
                };

                tasks.push(task);
            },
            Phase::UpdatePhase => {
                let task_1 = Task {
                    task_type: TaskType::LogicCalculation,
                    phase: Phase::UpdatePhase,
                    task_priority: TaskPriority::FirstPriority,
                };

                let task_2 = Task {
                    task_type: TaskType::PrepareRenderState,
                    phase: Phase::UpdatePhase,
                    task_priority: TaskPriority::FirstPriority,
                };

                tasks.push(task_1);
                tasks.push(task_2);
            },
            Phase::RenderPhase => {
                let task = Task {
                    task_type: TaskType::DrawRenderState,
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
                                            let task = Task {
                                                task_type: TaskType::Resize,
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
