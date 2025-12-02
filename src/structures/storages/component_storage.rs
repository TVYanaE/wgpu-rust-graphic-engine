use std::{
    any::Any,
    collections::HashMap
};
use crate::{
    structures::{
        entity::Entity,
    },
    traits::{
        any_storage_trait::AnyStorage,
    },
};


pub struct ComponentStorage<T> {
    dense_storage: Vec<T>,
    entities: Vec<Entity>,
    indices: HashMap<Entity, usize>,
}

impl<T> ComponentStorage<T> {
    pub fn new() -> Self {
        Self { 
            dense_storage: Vec::new(), 
            entities: Vec::new(), 
            indices: HashMap::new() 
        }
    }

    pub fn insert(&mut self, entity: Entity, component: T) {
        if let Some(&index) = self.indices.get(&entity) {
            self.dense_storage[index] = component;
            return;
        }
        
        let index = self.dense_storage.len();
        self.dense_storage.push(component);
        self.entities.push(entity);
        self.indices.insert(entity, index);
    }

    pub fn get(&self, entity: Entity) -> Option<&T> {
        self.indices
            .get(&entity)
            .map(|&index| &self.dense_storage[index])
    }

    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut T> {
        self.indices
            .get(&entity)
            .map(|&index| &mut self.dense_storage[index] )
    }

    pub fn has(&self, entity: Entity) -> bool {
        self.indices.contains_key(&entity)
    }

    pub fn remove(&mut self, entity: Entity) -> Option<T> {
        let index = self.indices.remove(&entity)?;

        let last_index = self.dense_storage.len() - 1;

        self.dense_storage.swap(index, last_index);
        self.entities.swap(index, last_index);

        let moved_entity = self.entities[index];

        self.indices.insert(moved_entity, index);

        self.entities.pop();
        Some(self.dense_storage.pop().unwrap())
    }

    pub fn iter(&self) -> impl Iterator<Item = (Entity, &T)> {
        self.entities
            .iter()
            .cloned()
            .zip(self.dense_storage.iter())
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (Entity, &mut T)> {
        let entities = &self.entities;
        self.dense_storage
            .iter_mut()
            .enumerate()
            .map(move |(i, component)| (entities[i], component))
    }
}

impl<T: 'static> AnyStorage for ComponentStorage<T> {
    fn remove_entity(&mut self, entity: Entity) {
        self.remove(entity);
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self        
    }
}
