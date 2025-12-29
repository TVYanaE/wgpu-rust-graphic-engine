
use crate::{
    structures::{
        common_structures::{
            scene::Scene,
        },
    }, 
    enums::{
        event_enums::{
            GameEvent
        },
        phase_enum::Phase,
    },
};

pub struct ControlThreadSceneState {
    active_scene: Option<Scene>,
}

impl ControlThreadSceneState {
    pub fn new() -> Self {
        Self { active_scene: None }
    }

    pub fn set_active_scene(&mut self, scene: Option<Scene>) {
        self.active_scene = scene;
    }

    pub fn get_allowed_phase(&self) -> Vec<Phase> {
        self.active_scene.as_ref().unwrap().get_allowed_phase()
    }

    pub fn get_allowed_game_events(&self, phase: &Phase) -> Option<Vec<GameEvent>> {
        self.active_scene.as_ref().unwrap().get_allowed_game_events(phase)
    } 
}
