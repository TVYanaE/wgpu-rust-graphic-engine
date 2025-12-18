use crate::{
    structures::{
        scheduler::Scheduler,
        executeurs::global_executeur::GlobalExecuter,
    },
    enums::{ 
        event_enum::Event,
        task_enum::Task,
    },
};

pub struct ControlThreadState {
    scheduler: Scheduler, 
    global_executeur: GlobalExecuter,
}

impl ControlThreadState {
    pub fn new() -> Self {
        Self { 
            scheduler: Scheduler::new(),
            global_executeur: GlobalExecuter::new(),
        }
    } 

    pub fn run_logic(&mut self, event_buffer: impl Iterator<Item = Event>) {
        let tasks = Task::events_to_tasks(event_buffer);

        self.scheduler.create_schedule(tasks);

        self.global_executeur.execute_schedule(self.scheduler.drain_schedule());
    }
    
    pub fn run_drawing(&mut self) {
        let mut tasks = Vec::new();

        tasks.push(Task::DrawRenderState);

        self.scheduler.create_schedule(tasks);

        self.global_executeur.execute_schedule(self.scheduler.drain_schedule());
    }

    pub fn init(&mut self) {
        let mut tasks = Vec::new();

        tasks.push(Task::Init);

        self.scheduler.create_schedule(tasks);

        self.global_executeur.execute_schedule(self.scheduler.drain_schedule());
    }
}
