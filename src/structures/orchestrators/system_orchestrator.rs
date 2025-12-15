use std::{
    rc::Rc,
};
use crate::{
    structures::{
        systems::{
            render_system::RenderSystem,
            camera_system::CameraSystem,
        },
        managers::{
            material_manager::MaterialManager,
        }, 
    },
    enums::{
        task_name_enum::TaskName,
    },
};

pub struct SystemOrchestrator { 
    render_system: RenderSystem,
    camera_system: CameraSystem,
}

impl SystemOrchestrator {
    pub fn new(material_manager: Rc<MaterialManager>) -> Self {
        

        let camera_system = CameraSystem::new();
        let render_system = RenderSystem::new(material_manager);

        Self { render_system, camera_system }
    }
}
