use std::{
    collections::{HashMap},
};
use crate::{
    aliases::ComponentStorageID,
    traits::{any_storage_trait::AnyStorage},
    managers::{
        entity_manager::EntityManager,
    },
    structures::{
        entity::Entity,
        storages::component_storage::ComponentStorage,
    },  
};

pub struct ECSManager {
    entity_manager: EntityManager,
    entity_storage: Vec<Entity>,
    component_storages: HashMap<ComponentStorageID, Box<dyn AnyStorage>>
}

impl ECSManager {
    pub fn new(entity_manager: EntityManager ) -> Self {
        Self { 
            entity_manager: entity_manager,
            entity_storage: Vec::new(), 
            component_storages: HashMap::new(),
        }
    } 

    pub fn create_entity(&mut self) -> Entity {
        let creating_entity = self.entity_manager.create_entity();
        self.entity_storage.push(creating_entity);
        creating_entity
    }
    
    pub fn add_component_to_entity<T: 'static>(&mut self, entity: Entity, component: T) {
        let component_storage_id = ComponentStorageID::of::<T>();
       
        if !self.component_storages.contains_key(&component_storage_id) {
            self.component_storages.insert(
                component_storage_id,
                Box::new(ComponentStorage::<T>::new()) as Box<dyn AnyStorage>
            );
        }

        let component_storage = self.component_storages
            .get_mut(&component_storage_id)
            .unwrap()
            .as_any_mut()
            .downcast_mut::<ComponentStorage<T>>()
            .expect("downd cast error in ECSManager");

        component_storage.insert(entity, component);
    }

    pub fn get_storage_mut<T: 'static>(&mut self) -> &mut ComponentStorage<T> {
        let component_storage_id = ComponentStorageID::of::<T>();

        if !self.component_storages.contains_key(&component_storage_id) {
            self.component_storages.insert(
                component_storage_id, 
                Box::new(ComponentStorage::<T>::new()) as Box<dyn AnyStorage>
            );
        }

        self.component_storages
            .get_mut(&component_storage_id)
            .unwrap()
            .as_any_mut()
            .downcast_mut::<ComponentStorage<T>>()
            .unwrap()
    } 
}
