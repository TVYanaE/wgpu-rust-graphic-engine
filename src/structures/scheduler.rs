use std::{
    cell::RefCell,
    rc::Rc,
    collections::{HashMap, VecDeque},
};
use crate::{  
    structures::{
        buses::{
            control_thread_data_bus::ControlThreadDataBus,
        },
        states::{
            phase_state::PhaseState,
        },
        task::Task,
    }, 
    enums::{
        phase_enum::Phase,
    },
};

pub struct Scheduler {  
    control_thread_data_bus: Rc<RefCell<ControlThreadDataBus>>,
    task_debds: HashMap<Phase, VecDeque<Task>>,
    phase_state: Rc<RefCell<PhaseState>>,
}

impl Scheduler {
    pub fn new(
        control_thread_data_bus: Rc<RefCell<ControlThreadDataBus>>,
        phase_state: Rc<RefCell<PhaseState>>,
    ) -> Self {
        let mut task_debds: HashMap<Phase, VecDeque<Task>> = HashMap::new();

        task_debds.insert(Phase::InitPhase, VecDeque::new());
        task_debds.insert(Phase::ShutdownPhase, VecDeque::new());
        task_debds.insert(Phase::UpdatePhase, VecDeque::new());
        task_debds.insert(Phase::RenderPhase, VecDeque::new());
        task_debds.insert(Phase::ExternalEventsPhase, VecDeque::new());

        Self {  
            control_thread_data_bus: control_thread_data_bus,
            task_debds: task_debds,
            phase_state,
        }
    } 

    pub fn start(&mut self){
        let mut data_bus = self.control_thread_data_bus.borrow_mut();

        for task in data_bus.drain_task_queue() {
            let task_debd = self.task_debds.get_mut(&task.phase).unwrap();

            task_debd.push_back(task);
        }

        // TODO: Time budget will be there
        let current_phase = self.phase_state.borrow().get_current_phase();

        let task_debd = self.task_debds.get_mut(&current_phase).unwrap();
        
    }
    
    
}
