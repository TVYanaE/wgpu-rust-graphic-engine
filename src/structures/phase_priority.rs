
use crate::{
    enums::{
        phase_enum::Phase,
    },
};


pub struct PhasePriority {
    pub phase: Phase,
    pub priority: u8,
}

pub const PHASE_PRIORITIES: [PhasePriority; 5] = [
    PhasePriority { phase: Phase::RenderPhase, priority: 4 },
    PhasePriority { phase: Phase::UpdatePhase, priority: 3 },
    PhasePriority { phase: Phase::ExternalEventsPhase, priority: 2 },
    PhasePriority { phase: Phase::InitPhase, priority: 1 },
    PhasePriority { phase: Phase::ShutdownPhase, priority: 0 },
];
