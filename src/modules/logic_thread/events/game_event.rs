use super::super::{
    phases::{Phase},
    game_objects::{
        enemy::EnemyDescriptor,
        player::PlayerDescriptor,
        map_object::MapObjectDescriptor,
    },
};


#[derive(Debug)]
pub enum GameEventData {
    SpawnEnemies(Vec<EnemyDescriptor>),
    SpawnPlayers(Vec<PlayerDescriptor>),
    SpawnMapObjects(Vec<MapObjectDescriptor>),
    DespawnGameObjects,
}

#[derive(Debug)]
pub struct GameEvent {
    pub event_data: GameEventData,
    pub phase: Phase, 
}
