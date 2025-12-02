
use crate::{
    structures::{
        entity::Entity,
    },
    aliases::{EntityID, GenerationID},
};

pub struct EntityManager {
    generations: Vec<GenerationID>,
    free_entity_id_list: Vec<EntityID>, 
}

impl EntityManager {
    pub fn new() -> Self {
        Self { 
            generations: Vec::new(), 
            free_entity_id_list: Vec::new() 
        }
    }

    pub fn create_entity(&mut self) -> Entity {
        if let Some(entity_id) = self.free_entity_id_list.pop() {
            let generation = self.generations[entity_id as usize];
            Entity { entity_id, generation }
        }
        else {
            let entity_id = self.generations.len() as EntityID;
            self.generations.push(0);
            Entity { entity_id, generation: 0 }
        }
    }

    pub fn destroy_entity(&mut self, entity: Entity) {
        let entity_id = entity.entity_id as usize;
        self.generations[entity_id] += 1;
        self.free_entity_id_list.push(entity.entity_id);
    }

    pub fn is_alive(&self, entity: &Entity) -> bool {
        self.generations[entity.entity_id as usize] == entity.generation
    }
}
