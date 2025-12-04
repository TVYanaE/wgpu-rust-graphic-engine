
use crate::{
    traits::system_trait::System,
    managers::{
        ecs_manager::ECSManager,
    },
};

pub struct RenderSystem {

}

impl System for RenderSystem {
    fn run(&mut self, ecs_manager: &mut ECSManager) {
        let sprite_storage = ecs_manager.get_storage_mut::<Sprite>();

        for (entity, sprite) in sprite_storage.iter() {
             
        }
    }
}


