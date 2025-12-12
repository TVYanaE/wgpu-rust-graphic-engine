use std::{
    sync::{Arc, RwLock},
};
use crate::{
    structures::{
        scheduler::Scheduler,
    },
};

pub struct GlobalOrchestrator {
    scheduler: Arc<RwLock<Scheduler>>
}

impl GlobalOrchestrator {
    pub fn new(scheduler: Arc<RwLock<Scheduler>>) -> Self {
        Self { 
            scheduler: scheduler 
        }
    }
}
