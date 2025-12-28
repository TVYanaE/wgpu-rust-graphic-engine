use std::{
    rc::Rc,
    cell::RefCell,
};
use crate::{
    structures::{ 
        control_thread::{
            states::{
                control_thread_request_state::ControlThreadRequestState,
                control_thread_scene_state::ControlThreadSceneState,
                control_thread_phase_state::ControlThreadPhaseState,
            },
            managers::{
                control_thread_io_event_manager::ControlThreadIOEventManager,
            },
        },  
    },
    enums::{
        control_thread_request_enum::ControlThreadRequest, 
        phase_enum::Phase, 
    }, 
};

pub struct ControlThreadPhaseManager {
    control_thread_request_state: Rc<RefCell<ControlThreadRequestState>>,
    control_thread_scene_state: Rc<RefCell<ControlThreadSceneState>>,
    control_thread_phase_state: Rc<RefCell<ControlThreadPhaseState>>,
    control_thread_io_event_manager: Rc<RefCell<ControlThreadIOEventManager>>,
}

impl ControlThreadPhaseManager {
    pub fn new(
        control_thread_request_state: Rc<RefCell<ControlThreadRequestState>>,
        control_thread_scene_state: Rc<RefCell<ControlThreadSceneState>>, 
        control_thread_phase_state: Rc<RefCell<ControlThreadPhaseState>>,
        control_thread_io_event_manager: Rc<RefCell<ControlThreadIOEventManager>>,
    ) -> Self {
        Self {
            control_thread_request_state: control_thread_request_state,
            control_thread_scene_state: control_thread_scene_state,
            control_thread_phase_state: control_thread_phase_state,
            control_thread_io_event_manager: control_thread_io_event_manager,
        }
    } 

    pub fn start(&mut self) {
        let control_thread_request_state = self.control_thread_request_state.borrow();
        let allowed_phases = self.control_thread_scene_state.borrow().get_allowed_phase();

        if let Some(priority_request) = control_thread_request_state.get_priority_request() {
            match priority_request {
                ControlThreadRequest::ShutdownRequest => {
                     
                },
                ControlThreadRequest::InitRequest => {
                    
                },
                ControlThreadRequest::LogicCalculationRequest => {
                    if allowed_phases.contains(&Phase::UpdatePhase) {
                        self.control_thread_phase_state.borrow_mut().set_current_phase(Phase::UpdatePhase);
                    }
                },
                ControlThreadRequest::RenderRequest => {
                    if allowed_phases.contains(&Phase::RenderPhase) {
                        self.control_thread_phase_state.borrow_mut().set_current_phase(Phase::RenderPhase);
                    } 
                },
            } 
        }
        else {
            if self.control_thread_io_event_manager.borrow_mut().check_io_events() {
                if allowed_phases.contains(&Phase::ExternalEventsPhase) {
                        self.control_thread_phase_state.borrow_mut().set_current_phase(Phase::ExternalEventsPhase);
                }
            }
            else {
                if allowed_phases.contains(&Phase::Idle) {
                        self.control_thread_phase_state.borrow_mut().set_current_phase(Phase::Idle);
                }
            }
        }
    }
}
