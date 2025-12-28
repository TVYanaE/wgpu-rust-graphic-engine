use crate::{
    enums::phase_enum::Phase,
};

pub struct ControlThreadPhaseState {
    prev_phase: Phase,
    current_phase: Phase,
}

impl ControlThreadPhaseState {
    pub fn new() -> Self {
        Self {
            prev_phase: Phase::Idle,
            current_phase: Phase::Idle,
        }
    }

    pub fn set_current_phase(&mut self, current_phase: Phase) {
        self.prev_phase = self.current_phase;
        self.current_phase = current_phase;
    }

    pub fn get_current_phase(&self) -> Phase {
        self.current_phase
    }

    pub fn get_prev_phase(&self) -> Phase {
        self.prev_phase
    }
}
