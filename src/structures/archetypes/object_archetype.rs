use std::{
    collections::{HashMap,},
};
use crate::{
    aliases::{EntityID, VacantPlaceInChunk, ChunkArchetypeIndex, OccupancyChunkRate, FragmentationChunkRate},
    consts::{OBJECT_ARCHETYPE_CHUNK_SIZE},
    structures::{
        chunks::{
            object_archetype_chunk::ObjectArchetypeChunk,
        },
        components::{
            size_component::SizeComponent,
            position_component::PositionComponent,
            sprite_component::SpriteComponent,
        },
    }, 
    enums::{
        errors::ArchetypeError,
    },
};


pub struct ObjectArchetype {
    occupancy_chunk_rates: Vec<OccupancyChunkRate>,
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
            occupancy_chunk_rates: Vec::new(),
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
            // cluster index is index of bit mask element in created chunks 
            // free_chunks is same for free chunks 
            let free_chunks_current_bit_maks = self.free_chunks[cluster_index];

            // lookup free and created chunk 
            let cros_bit_mask = free_chunks_current_bit_maks & created_chunks_current_bit_mask;

            if cros_bit_mask.count_ones() != 0 {
                // if free and created chunk has been foind 
                // calculate bit index of chunk
                let leading_zeros_amount = cros_bit_mask.leading_zeros() as usize;
                let current_free_index = u64::BITS as usize - 1 - leading_zeros_amount;

                // and after that calculate index of chunk in chunk storage
                let chunk_index = cluster_index * (u64::BITS as usize) + current_free_index;

                match self.object_chunks[chunk_index].add_entity_to_chunk(
                    entity_id, 
                    position_component, 
                    size_component, 
                    sprite_component
                ) {
                    Ok(vacant_place_in_chunk) => {
                        // if there are not free place in chunk mark it like occupied
                        if vacant_place_in_chunk == 0 {
                            self.free_chunks[cluster_index] &= !(1u64 << current_free_index);
                        }
                        
                        // change amount of free place in chunk 
                        self.vacant_places_in_chunk[chunk_index] = vacant_place_in_chunk;

                        // mapping between chunk index and entity_id
                        self.entities_map.insert(entity_id, chunk_index);

                        return Ok(());
                    },
                    Err(_chunk_error) => {
                        // if chunk was full (this branch work only in execption in logic)
                        self.free_chunks[cluster_index] &= !(1u64 << current_free_index);
                        continue;
                    }
                }
            }             
        } 

        // if there are not free chunk create new chunk
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

    pub fn remove_entity_from_archetype(&mut self, entity_id: EntityID) -> Result<(), ArchetypeError> {
        let chunk_index = self.entities_map.get(&entity_id).ok_or_else(||{
            ArchetypeError::EntityWasNotFound("Object Archetype".to_string()) 
        })?
        .clone(); 
        
        let free_place = self.object_chunks[chunk_index]
            .remove_entity_from_chunk(entity_id)
            .map_err(|_error|{
                ArchetypeError::RemoveEntityFromChunkError("Object archetype".to_string()) 
            })?;

        self.vacant_places_in_chunk[chunk_index] = free_place;
        
        let cluster_index = chunk_index / (u64::BITS as usize);
        let bit_index = chunk_index % (u64::BITS as usize);

        self.free_chunks[cluster_index] |= 1u64 << bit_index;

        return Ok(());
    } 

    pub fn calc_metrics(&mut self) {
        let delta_occupance_rate_position_amount = self.object_chunks.len() - self.object_chunks.len();

        // sync of amount of metrics and amount of chunks 
        if delta_occupance_rate_position_amount != 0 {
            for _ in 0..delta_occupance_rate_position_amount {
                self.occupancy_chunk_rates.push(0.0);
            }
        }

        for chunk in self.object_chunks.iter_mut() {
            chunk.calc_fragmentation();
        }

        for (chunk_number, vacant_place_in_chunk_amount) in self.vacant_places_in_chunk.iter().enumerate() {
            let occupance_chunk_rate = (vacant_place_in_chunk_amount.clone() as f32) / (OBJECT_ARCHETYPE_CHUNK_SIZE as f32);
            self.occupancy_chunk_rates[chunk_number] = occupance_chunk_rate;      
        }

    }

    pub fn collect_archetype_metrics(&self) -> impl Iterator<Item = &OccupancyChunkRate> {
        self.occupancy_chunk_rates.iter()
    }

    pub fn collect_chunks_metrics(&self) -> impl Iterator<Item = &FragmentationChunkRate> {
        self.object_chunks.iter().map(|chunk|{
            chunk.get_fragmentation_rate()
        })
    }

    
}
