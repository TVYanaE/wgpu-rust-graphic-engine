use super::super::{
    scene::{
        ScenePhase, Scene,
    },
    phases::{
        Phase,
    },
};

#[derive(Debug, Default)]
pub struct SceneState {
    current_scene: Option<Scene>,
    current_scene_phase: Option<ScenePhase>,
}

impl SceneState {
    pub fn set_current_scene(&mut self, current_scene: Scene) {
        self.current_scene = Some(current_scene);
    }

    pub fn set_current_scene_phase(&mut self, current_scene_phase: ScenePhase) {
        self.current_scene_phase = Some(current_scene_phase);
    }

    pub fn is_allowed_phase(&self, phase: &Phase) -> bool {
        match self.current_scene_phase.unwrap() {
            ScenePhase::BeginScene => {
                self.current_scene.as_ref().unwrap().begin_scene_allowed_phase.contains(&phase)
            },
            ScenePhase::DuringScene => {
                self.current_scene.as_ref().unwrap().during_scene_allowed_phase.contains(&phase)
            },
            ScenePhase::EndScene => {
                self.current_scene.as_ref().unwrap().end_scene_allowed_phase.contains(&phase)
            },
        }
    } 
}
