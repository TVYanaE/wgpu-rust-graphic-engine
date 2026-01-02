use std::{
    collections::{VecDeque},
};
use super::super::{
    states::{
        scene_state::SceneState
    }, 
    phases::{
        Phase
    },
    events::{
        game_event::{GameEvent, GameEventData},
    },
    managers::{
        world_manager::WorldManager,
    },
    game_objects::{
        enemy::EnemyDescriptor,
        player::PlayerDescriptor,
        map_object::MapObjectDescriptor,
    },
};

const PHASE: Phase = Phase::SpawnPhase;

pub fn spawn_phase(
    scene_state: &SceneState,
    world_manager: &WorldManager,
    game_event_queue: &mut VecDeque<GameEvent>,
) {
    if !scene_state.is_allowed_phase(&PHASE) {
        return;
    } 
    
    let mut game_event_indices: Vec<usize> = Vec::new();

    for (game_event_index, game_event) in game_event_queue.iter().enumerate() {
        if game_event.phase == PHASE {
            game_event_indices.push(game_event_index);
        }
    }

    if game_event_indices.is_empty() {
        return;
    }

    let mut players_for_spawn: Vec<PlayerDescriptor> = Vec::new();
    let mut enemies_for_spawn: Vec<EnemyDescriptor> = Vec::new();
    let mut map_objects_for_spawn: Vec<MapObjectDescriptor> = Vec::new();
     

    for game_event_index in game_event_indices {
        if let Some(game_event) = game_event_queue.remove(game_event_index) {
            match game_event.event_data {
                GameEventData::SpawnPlayers(players) => {
                    players_for_spawn.extend(players.into_iter());
                },
                _ => {},
            }
        }
        else {
            panic!("Panic in Spawn Despawn Phase handler GAME EVENT");
        }
    }

    world_manager.register_spawn_intention(players_for_spawn, enemies_for_spawn, map_objects_for_spawn);
    world_manager.start_spawn(); 
}

