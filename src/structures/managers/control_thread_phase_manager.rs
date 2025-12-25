use std::{
    rc::Rc,
    cell::RefCell,
};
use crate::{
    enums::{
        signals::{
            control_thread_signal_enums::ControlThreadInputSignal,
        },
        phase_enum::Phase, 
    },
    structures::{ 
        states::{
            phase_state::PhaseState,
        },
        control_thread_signal_storage::ControlThreadSignalStorage,
        managers::{
            control_thread_io_event_manager::ControlThreadIOEventManager,
        },
    },
};

pub struct ControlThreadPhaseManager {
    phase_state: Rc<RefCell<PhaseState>>,
    control_thread_io_event_manager: Rc<RefCell<ControlThreadIOEventManager>>,
    control_thread_signal_storage: Rc<RefCell<ControlThreadSignalStorage>>,
}

impl ControlThreadPhaseManager {
    pub fn new(
        phase_state: Rc<RefCell<PhaseState>>, 
        control_thread_io_event_manager: Rc<RefCell<ControlThreadIOEventManager>>,
        control_thread_signal_storage: Rc<RefCell<ControlThreadSignalStorage>>,
    ) -> Self {
        Self { 
            phase_state: phase_state,
            control_thread_io_event_manager: control_thread_io_event_manager,
            control_thread_signal_storage: control_thread_signal_storage,
        }
    } 

    pub fn start(&mut self) {
        let mut phase_state = self.phase_state.borrow_mut();     
        let mut control_thread_signal_storage = self.control_thread_signal_storage.borrow_mut();
        
        if let Some(priority_signal) = control_thread_signal_storage.get_priority_signal() {
            match priority_signal {
                ControlThreadInputSignal::Shutdown => {
                    phase_state.set_current_phase(Phase::ShutdownPhase);
                    control_thread_signal_storage.consume_signal_for_phase(Phase::ShutdownPhase);
                },
                ControlThreadInputSignal::Init => {
                    phase_state.set_current_phase(Phase::InitPhase);
                    control_thread_signal_storage.consume_signal_for_phase(Phase::InitPhase);
                },
                ControlThreadInputSignal::LogicStart => {
                    phase_state.set_current_phase(Phase::UpdatePhase);
                    control_thread_signal_storage.consume_signal_for_phase(Phase::UpdatePhase);
                },
                ControlThreadInputSignal::FrameStart => {
                    phase_state.set_current_phase(Phase::RenderPhase);
                    control_thread_signal_storage.consume_signal_for_phase(Phase::RenderPhase);
                },
            } 
        }
        else {
            if self.control_thread_io_event_manager.borrow_mut().check_io_events() {
                phase_state.set_current_phase(Phase::ExternalEventsPhase);
            }
            else {
                phase_state.set_current_phase(Phase::Idle);
            }
        }
    }
}
