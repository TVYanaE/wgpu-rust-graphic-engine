use std::{
    collections::{HashMap}
};
use super::super::{
    scene::{SceneName, Scene},
    phases::{Phase},
};

pub enum SceneChangeRequest {
    StartScene(SceneName),
    EndScene,
}

pub struct SceneManager {
    scene_storage: HashMap<SceneName, Scene>,
}

impl SceneManager {
    pub fn new() -> Self {
        let mut scene_storage: HashMap<SceneName, Scene> = HashMap::new();        

        let gameplay_begin_allowed_phase = vec![
            Phase::SpawnPhase,
            Phase::DespawnPhase,
            Phase::PrelogicPhase,
            Phase::SimulationPhase,
            Phase::PostlogicPhase,
            Phase::PrepareRenderStatePhase,
        ];
        let gameplay_during_allowed_phase = vec![
            Phase::SpawnPhase,
            Phase::DespawnPhase,
            Phase::PrelogicPhase,
            Phase::SimulationPhase,
            Phase::PostlogicPhase,
            Phase::PrepareRenderStatePhase,
        ];
        let gameplay_end_allowed_phase = vec![
            Phase::SpawnPhase,
            Phase::DespawnPhase,
            Phase::PrelogicPhase,
            Phase::SimulationPhase,
            Phase::PostlogicPhase,
            Phase::PrepareRenderStatePhase,
        ];
 
        let gameplay_scene = Scene {
            name: SceneName::Gameplay,
            begin_scene_allowed_phase: gameplay_begin_allowed_phase,
            during_scene_allowed_phase: gameplay_during_allowed_phase,
            end_scene_allowed_phase: gameplay_end_allowed_phase,
        };

        scene_storage.insert(SceneName::Gameplay, gameplay_scene);        

        Self { 
            scene_storage: scene_storage
        }
    }

    pub fn get_scene(&self, scene_name: &SceneName) -> Option<Scene> {
        self.scene_storage.get(scene_name).cloned()
    }
}

