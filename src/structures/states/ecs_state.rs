use std::{
    rc::Rc,
};
use winit::{
    event_loop::ActiveEventLoop,
    keyboard::KeyCode
};
use crate::{
    managers::{
        ecs_manager::ECSManager,
        entity_manager::EntityManager,
        system_manager::SystemManager,
        material_manager::MaterialManager,
    },
    structures::{
        entity_factory::EntityFactory,
        render_items::RenderItem,
    },
    systems::{
        render_system::RenderSystem,
    },
};

pub struct ECSState {
    pub ecs_manager: ECSManager,
    pub system_manager: SystemManager, 
    pub entity_factory: EntityFactory,
    pub render_system: Option<RenderSystem>,
}

impl ECSState {
    pub fn new() -> Self {
        let entity_manager = EntityManager::new();

        let system_manager = SystemManager::new(); 

        let ecs_manager = ECSManager::new(entity_manager);
        let entity_factory = EntityFactory::new();

        Self { 
            system_manager: system_manager, 
            ecs_manager: ecs_manager, 
            entity_factory: entity_factory,
            render_system: None,
        }
    }

    pub fn handle_key(&mut self, event_loop: &ActiveEventLoop, code: KeyCode, is_pressed: bool) {
        if code == KeyCode::Escape && is_pressed {
            event_loop.exit();
        }
    }
    
    pub fn init_systems(&mut self, material_manager: Rc<MaterialManager>) {
        let render_system = RenderSystem::new(material_manager);
        self.render_system = Some(render_system);
    }

    pub fn render_prepare(&mut self) {
        self.entity_factory.create_test_object(&mut self.ecs_manager);
        self.render_system.as_mut().unwrap().run(&mut self.ecs_manager);
    }
   
    pub fn get_render_items(&self) -> &[RenderItem] {
        self.render_system.as_ref().unwrap().get_render_item_cache()
    }

    pub fn run_real_time_tasks(&mut self) {

    }
}


