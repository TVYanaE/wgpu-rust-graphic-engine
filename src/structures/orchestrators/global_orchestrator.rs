use std::{
    sync::{Arc, RwLock},
};
use crate::{
    structures::{
        scheduler::Scheduler,
        orchestrators::{
            archetype_orchestrator::ArcherypeOrchestrator,
            system_orchestrator::SystemOrchestrator,
        },
    },
    enums::{
        task_name_enum::TaskName,
    },
};

pub struct GlobalOrchestrator {
    archetype_orchestrator: Arc<RwLock<ArcherypeOrchestrator>>,
    system_orchestrator: Arc<RwLock<SystemOrchestrator>>,
    scheduler: Arc<RwLock<Scheduler>>,
}

impl GlobalOrchestrator {
    pub fn new(
        scheduler: Arc<RwLock<Scheduler>>,
        system_orchestrator: Arc<RwLock<SystemOrchestrator>>,
        archetype_orchestrator: Arc<RwLock<ArcherypeOrchestrator>>
    ) -> Self {
        Self { 
            scheduler: scheduler,
            system_orchestrator: system_orchestrator,
            archetype_orchestrator: archetype_orchestrator,
        }
    }

    pub fn run_tact(&self) {
        let mut frame_phase_buffer = Vec::new();

        let mut scheduler_guard = self.scheduler.write().unwrap();

        for frame_phase in scheduler_guard.drain_schedule() {
            frame_phase_buffer.push(frame_phase);
        }

        drop(scheduler_guard);

        for frame_phase in frame_phase_buffer.iter_mut() {
            for mut frame_phase_batch in frame_phase.drain_frame_phase() {
                for task in frame_phase_batch.drain_batch() {
                     
                }
            }
        }
    }
}
