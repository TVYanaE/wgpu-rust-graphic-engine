use std::{
    collections::{VecDeque},
};

use super::super::{
    managers::{
        world_manager::WorldManager,
    },
    events::{
        game_event::{GameEvent, GameEventData},
    },
    states::{
        scene_state::SceneState,
    },
    phases::{
        Phase
    },
};

const PHASE: Phase = Phase::DespawnPhase;

pub fn despawn_phase(
    world_manager: &WorldManager,
    game_event_queue: &mut VecDeque<GameEvent>,
    scene_state: &SceneState,
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
    
    for game_event_index in game_event_indices {
        if let Some(game_event) = game_event_queue.remove(game_event_index) {
            match game_event.event_data {
                GameEventData::DespawnGameObjects => {world_manager.start_despawn();},
                _ => {},
            } 
        }
        else {
            panic!("Panic in Despawn Phase handler");
        }
    }

}
