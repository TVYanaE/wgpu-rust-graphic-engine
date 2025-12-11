use crate::{
    aliases::{EntityID, VacantPlaceInChunk, EntityChunkIndex},
    components::{
        size_component::SizeComponent,
        position_component::PositionComponent,
        sprite_component::SpriteComponent,
    },
    consts::OBJECT_ARCHETYPE_CHUNK_SIZE,
    enums::errors::ChunkError,
};

pub struct ObjectArchetypeChunk {
    entities_map: [EntityID; OBJECT_ARCHETYPE_CHUNK_SIZE],
    positions: [PositionComponent; OBJECT_ARCHETYPE_CHUNK_SIZE],
    sizes: [SizeComponent; OBJECT_ARCHETYPE_CHUNK_SIZE],
    sprites: [SpriteComponent; OBJECT_ARCHETYPE_CHUNK_SIZE],
    activities: u16, // bit mask for activiti element
    free_places: u16 // bit maks for free places 
}

impl ObjectArchetypeChunk {
    pub fn new() -> Self {
        let positions = [PositionComponent::default(); OBJECT_ARCHETYPE_CHUNK_SIZE];
        let sizes = [SizeComponent::default(); OBJECT_ARCHETYPE_CHUNK_SIZE];
        let sprites = [SpriteComponent::default(); OBJECT_ARCHETYPE_CHUNK_SIZE];
        let entites_map = [EntityID::MAX; OBJECT_ARCHETYPE_CHUNK_SIZE];
        let activities = 0;
        let free_places = !0;

        Self { 
            entities_map: entites_map,
            positions: positions, 
            sizes: sizes, 
            sprites: sprites,
            activities: activities,
            free_places: free_places,
        }
    }

    // 0 1 2 3 4 5 6 7 
    // 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0

    fn entity_index_to_bit_index(&self, entity_index: EntityChunkIndex) -> u16 {
        (u16::BITS as u16) - 1 - (entity_index as u16)
    }
    
    // true - render, false - no render  
    fn set_render_activity_by_index(&mut self, entity_index: EntityChunkIndex, activity: bool) {
        let bit_index_delta = self.entity_index_to_bit_index(entity_index); 
        match activity {
            true => { self.activities |= 1u16 << bit_index_delta; },
            false => { self.activities &= !(1u16 << bit_index_delta) }
        };
    }

    // true - place free, false - place is ocupied
    fn set_free_place_bit(&mut self, entity_index: EntityChunkIndex, free: bool) {
        let bit_index_delta = self.entity_index_to_bit_index(entity_index); 
        match free {
            true => { self.free_places |= 1u16 << bit_index_delta; },
            false => { self.free_places &= !(1u16 << bit_index_delta) }
        };
    }
   
    fn is_active_by_index(&self, entity_index: EntityChunkIndex) -> bool {
        let bit_index_delta = self.entity_index_to_bit_index(entity_index);  
        (self.activities & (1u16 << bit_index_delta)) != 0
    }

    // 0 1 2 3 4 5 6 7 8
    // 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 0
    // 0  0  0  1  0  0  0 0 0 0 0 0 0 0 0 0

    pub fn add_entity_to_chunk(
        &mut self,
        entity_id: EntityID,
        position_component: PositionComponent,
        size_component: SizeComponent,
        sprite_component: SpriteComponent,
    ) -> Result<VacantPlaceInChunk, ChunkError> {
        if self.free_places.count_ones() == 0 {
            return Err(ChunkError::NotEnoughSpaceInChunk("Object Archetype Chunk".to_string()));
        }
      
        let leading_zeros_amount = self.free_places.leading_zeros() as usize;
        let current_free_index = u16::BITS as usize - 1 - leading_zeros_amount;

        self.positions[current_free_index] = position_component;
        self.sizes[current_free_index] = size_component;
        self.sprites[current_free_index] = sprite_component;
        self.entities_map[current_free_index] = entity_id; 
        self.set_render_activity_by_index(current_free_index, true);

        self.set_free_place_bit(current_free_index, false); 

        return Ok(self.free_places.count_ones());
    }

    pub fn remove_entity_from_chunk(&mut self, entity_id: EntityID) -> Result<VacantPlaceInChunk, ChunkError> {
        let entity_index = self.entities_map.iter().position(|current_entity| {
            *current_entity == entity_id
        })
        .ok_or_else(|| {
            ChunkError::ChunkDoesntContainEntity("Object archetype Chunk".to_string())
        })?;

        self.set_render_activity_by_index(entity_index, false);
        self.set_free_place_bit(entity_index, true);
        
        return Ok(self.free_places.count_ones())
    }

    /// Hot method
    pub fn apply_hot_function<F>(&self, function: &F) 
    where 
        F: Fn(
            &PositionComponent,
            &SizeComponent,
            &SpriteComponent,
            EntityID
        )
    {
        for entity_index in 0..OBJECT_ARCHETYPE_CHUNK_SIZE {
            if self.is_active_by_index(entity_index) {
                function(
                    &self.positions[entity_index], 
                    &self.sizes[entity_index], 
                    &self.sprites[entity_index],
                    self.entities_map[entity_index]
                );
            }
        }
        
    }

    /// Hot method 
    pub fn apply_hot_mut_function<F>(&mut self, function: &mut F) 
    where 
        F: FnMut(
            &mut PositionComponent,
            &mut SizeComponent,
            &mut SpriteComponent,
            EntityID
        )
    {
        for entity_index in 0..OBJECT_ARCHETYPE_CHUNK_SIZE {
            if self.is_active_by_index(entity_index) {
                function(
                    &mut self.positions[entity_index], 
                    &mut self.sizes[entity_index], 
                    &mut self.sprites[entity_index],
                    self.entities_map[entity_index],
                );
            }
        }
        
    }
}
