use std::{
    rc::Rc,
};
use winit::{
    event_loop::ActiveEventLoop,
    keyboard::KeyCode
};
use glam::Mat4;
use crate::{
    managers::{
        entity_manager::EntityManager,
        material_manager::MaterialManager,
    },
    structures::{
        render_items::RenderItem,
        systems::{
            render_system::RenderSystem,
            camera_system::CameraSystem,
        },
    },
    
};

pub struct LogicState {
    pub entity_manager: EntityManager,
    pub render_system: Option<RenderSystem>,
    pub camera_system: Option<CameraSystem>,
}

impl LogicState {
    pub fn new() -> Self {
        let entity_manager = EntityManager::new();

        Self { 
            entity_manager: entity_manager,
            render_system: None,
            camera_system: None,
        }
    }

    pub fn handle_key(&mut self, event_loop: &ActiveEventLoop, code: KeyCode, is_pressed: bool) {
        if code == KeyCode::Escape && is_pressed {
            event_loop.exit();
        }
    }
    
    pub fn init_systems(&mut self, 
        material_manager: Rc<MaterialManager>,
    ) {
        let render_system = RenderSystem::new(material_manager);
        let camera_system = CameraSystem::new();

        self.render_system = Some(render_system);
        self.camera_system = Some(camera_system);
    }

    pub fn render_prepare(&mut self, window_width: f32, window_height: f32) {
        self.entity_manager.create_test_object(); 

        self.render_system.as_mut().unwrap().run(&mut self.);
        self.camera_system.as_mut().unwrap().run(&mut self.en);
    }
   
    pub fn get_render_items(&self) -> &[RenderItem] {
        self.render_system.as_ref().unwrap().get_render_item_cache()
    }

    pub fn get_view_project_matrix(&self) -> &Mat4 {
        self.camera_system.as_ref().unwrap().get_view_project_matrix()
    } 

    pub fn resize_handle(&mut self, window_width: f32, window_height: f32) {
        self.camera_system.as_ref().unwrap().window_resize_handle(&mut self.ecs_manager, window_width, window_height);
    }

    pub fn run_real_time_tasks(&mut self) {
        
    }
}


