use crate::{
    aliases::EntityID,
    consts::MAX_ENTITIES
};



pub struct EntityIDManager {
    free_ids: Vec<EntityID>,
    next_free: usize
}

impl EntityIDManager {
    pub fn new() -> Self {
        let mut free_ids = Vec::with_capacity(MAX_ENTITIES);

        for i in 0..MAX_ENTITIES {
            free_ids.push(i as EntityID);
        }

        Self { 
            free_ids: free_ids,
            next_free: MAX_ENTITIES,
        }
    }

    pub fn create_entity(&mut self) -> Option<EntityID> {
        if self.next_free == 0 {
            return None;
        }

        self.next_free -= 1; 

        let entity_id = self.free_ids[self.next_free];
        Some(entity_id)
    }

    pub fn destroy_entity(&mut self, entiti_id: EntityID) {
        self.next_free += 1;
        
        self.free_ids[self.next_free] = entiti_id;
    }

}
