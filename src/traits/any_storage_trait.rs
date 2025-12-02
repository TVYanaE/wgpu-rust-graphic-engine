use std::any::Any;
use crate::{
    structures::entity::Entity,
};

pub trait AnyStorage {
    fn remove_entity(&mut self, entity: Entity);
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
