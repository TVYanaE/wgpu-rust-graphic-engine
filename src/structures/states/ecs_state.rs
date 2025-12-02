use winit::{
    event_loop::ActiveEventLoop,
    keyboard::KeyCode
};
use crate::{
    managers::{
        ecs_manager::ECSManager,
        entity_manager::EntityManager,
        system_manager::SystemManager,
    },
};

pub struct ECSState {
    pub ecs_manager: ECSManager,
}

impl ECSState {
    pub fn new() -> Self {
        let entity_manager = EntityManager::new();
        let system_manager = SystemManager::new();
        let ecs_manager = ECSManager::new(entity_manager, system_manager);

        Self { ecs_manager: ecs_manager }
    }

    pub fn handle_key(&mut self, event_loop: &ActiveEventLoop, code: KeyCode, is_pressed: bool) {
        if code == KeyCode::Escape && is_pressed {
            event_loop.exit();
        }
    } 
    
}


