use std::{
    collections::{HashMap,},
};
use crate::{
    aliases::{EntityID, VacantPlaceInChunk, ChunkArchetypeIndex},
    structures::{
        chunks::{
            object_archetype_chunk::ObjectArchetypeChunk,
        },
    },
    components::{
        size_component::SizeComponent,
        position_component::PositionComponent,
        sprite_component::SpriteComponent,
    },
    enums::{
        errors::ArchetypeError,
    },
};


pub struct ObjectArchetype {
    entities_map: HashMap<EntityID, ChunkArchetypeIndex>,
    object_chunks: Vec<ObjectArchetypeChunk>,
    vacant_places_in_chunk: Vec<VacantPlaceInChunk>,
    free_chunks: Vec<u64>, // bit mask 1 - free, 0 - occupied
    created_chunks: Vec<u64> // bit mask 1 - created, 0 - not
}

impl ObjectArchetype {
    pub fn new() -> Self {
        let free_chunks = vec![!0u64];
        let created_chunks = vec![0u64];

        Self {
            entities_map: HashMap::new(),
            object_chunks: Vec::new(),
            vacant_places_in_chunk: Vec::new(),
            free_chunks: free_chunks,
            created_chunks: created_chunks,
        }
    }

    pub fn add_entity_to_archetype(
        &mut self,
        entity_id: EntityID,
        size_component: SizeComponent,
        position_component: PositionComponent,
        sprite_component: SpriteComponent,
    ) -> Result<(), ArchetypeError> {
        for (cluster_index, created_chunks_current_bit_mask) in self.created_chunks.iter().enumerate() {
            // cluster indes is index of bit mask element in created chunks 
            // free_chunks is same for free chunks 
            let free_chunks_current_bit_maks = self.free_chunks[cluster_index];

            let cros_bit_mask = free_chunks_current_bit_maks & created_chunks_current_bit_mask;

            if cros_bit_mask.count_ones() != 0 {
                let leading_zeros_amount = cros_bit_mask.leading_zeros() as usize;
                let current_free_index = u64::BITS as usize - 1 - leading_zeros_amount;

                let chunk_index = cluster_index * (u64::BITS as usize) + current_free_index;

                match self.object_chunks[chunk_index].add_entity_to_chunk(
                    entity_id, 
                    position_component, 
                    size_component, 
                    sprite_component
                ) {
                    Ok(vacant_place_in_chunk) => {
                        if vacant_place_in_chunk == 0 {
                            self.free_chunks[cluster_index] &= !(1u64 << current_free_index);
                        }

                        self.vacant_places_in_chunk[chunk_index] = vacant_place_in_chunk;

                        self.entities_map.insert(entity_id, chunk_index);

                        return Ok(());
                    },
                    Err(_chunk_error) => {
                        self.free_chunks[cluster_index] &= !(1u64 << current_free_index);
                        continue;
                    }
                }
            }             
        } 

        let mut new_chunk = ObjectArchetypeChunk::new();

        let free_places_in_new_chunk = new_chunk.add_entity_to_chunk(
            entity_id, 
            position_component, 
            size_component, 
            sprite_component
        )
        .map_err(|_chunk_error| {
            ArchetypeError::EntityAddingErrorInChunk("Object Chunk".to_string())
        })?;
 
        let new_chunk_index = self.object_chunks.len();

        self.object_chunks.push(new_chunk); 

        self.entities_map.insert(entity_id, new_chunk_index);

        self.vacant_places_in_chunk.push(free_places_in_new_chunk);

        let cluster_index = new_chunk_index / (u64::BITS as usize);
        let bit_index = new_chunk_index % (u64::BITS as usize);

        if cluster_index >= self.created_chunks.len() {
            self.created_chunks.push(0);
            self.free_chunks.push(!0u64);
        }

        self.created_chunks[cluster_index] |= 1u64 << bit_index;

        if free_places_in_new_chunk > 0 {
            self.free_chunks[cluster_index] |= 1u64 << bit_index;
        }
        else {
            self.free_chunks[cluster_index] &= !(1u64 << bit_index);
        }

        return Ok(());
    }
}
