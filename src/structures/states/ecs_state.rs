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
    structures::entity_factory::EntityFactory,
};

pub struct ECSState {
    pub ecs_manager: ECSManager,
    pub entity_factory: EntityFactory,
}

impl ECSState {
    pub fn new() -> Self {
        let entity_manager = EntityManager::new();

        let system_manager = SystemManager::new();

        let  

        let ecs_manager = ECSManager::new(entity_manager, system_manager);
        let entity_factory = EntityFactory::new();

        Self { ecs_manager: ecs_manager, entity_factory: entity_factory }
    }

    pub fn handle_key(&mut self, event_loop: &ActiveEventLoop, code: KeyCode, is_pressed: bool) {
        if code == KeyCode::Escape && is_pressed {
            event_loop.exit();
        }
    }

    pub fn render_prepared(&mut self) {
        self.entity_factory.create_triagle_tree(&mut self.ecs_manager);
    }
    
}


