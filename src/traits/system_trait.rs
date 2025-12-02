use crate::{managers::ecs_manager::ECSManager};


pub trait System {
    fn run(&mut self, ecs_manager: &mut ECSManager);
}
