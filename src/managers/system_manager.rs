use crate::{
    traits::{system_trait::System},
    managers::{ecs_manager::ECSManager},
};

pub struct SystemManager {
    system_storage: Vec<Box<dyn System>>
}

impl SystemManager {
    pub fn new() -> Self {
        Self { system_storage: Vec::new() }
    }

    pub fn add_system<T: System + 'static>(&mut self, system: T) {
        self.system_storage.push(Box::new(system));
    }

    pub fn run(&mut self, ecs_manager: &mut ECSManager) {
        for system in self.system_storage.iter_mut() {
            system.run(ecs_manager);
        }
    }
}
