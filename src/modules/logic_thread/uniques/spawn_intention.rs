use std::{
    collections::{VecDeque},
};
use shipyard::{
    Unique
};
use super::super::{
    game_objects::{
        enemy::EnemyDescriptor,
        player::PlayerDescriptor,
        map_object::MapObjectDescriptor,
    },
};

#[derive(Debug, Unique)]
pub struct SpawnIntention {
    pub players_for_spawn: VecDeque<PlayerDescriptor>,
    pub enemies_for_spawn: VecDeque<EnemyDescriptor>,
    pub map_objects_for_spawn: VecDeque<MapObjectDescriptor>,
}

impl Default for SpawnIntention {
    fn default() -> Self {
        Self { 
            players_for_spawn: VecDeque::with_capacity(64), 
            enemies_for_spawn: VecDeque::with_capacity(64), 
            map_objects_for_spawn: VecDeque::with_capacity(64), 
        }
    }
}
