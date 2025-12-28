use std::{
    collections::{HashMap},
};

use crate::{
    enums::{
        scene_type_enum::SceneType,
        phase_enum::Phase,
        game_event_enum::GameEvent,
    },
};

#[derive(Debug, Clone)]
pub struct Scene {
    scene_type: SceneType,
    allowed_phase: Vec<Phase>,
    allowed_game_events: HashMap<Phase, Vec<GameEvent>>,
}

impl Scene {
    pub fn new(
        scene_type: SceneType, 
        allowed_phase: Vec<Phase>,
        allowed_game_events: HashMap<Phase, Vec<GameEvent>>
    ) -> Self {

        Self { 
            scene_type: scene_type, 
            allowed_phase: allowed_phase, 
            allowed_game_events: allowed_game_events, 
        }
    }

    pub fn get_allowed_phase(&self) -> Vec<Phase> {
        self.allowed_phase.clone()
    }

    pub fn get_allowed_game_events(&self, phase: &Phase) -> Option<Vec<GameEvent>> {
        self.allowed_game_events.get(phase).cloned()
    }
}
