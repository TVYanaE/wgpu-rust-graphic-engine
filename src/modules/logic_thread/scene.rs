use super::{
    phases::{
        Phase
    }, 
};

#[derive(Debug, Clone, Copy)]
pub enum ScenePhase {
    BeginScene,
    DuringScene,
    EndScene,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SceneName {
    Gameplay,
}

#[derive(Debug, Clone,)]
pub struct Scene {
    pub name: SceneName,
    pub begin_scene_allowed_phase: Vec<Phase>,
    pub during_scene_allowed_phase: Vec<Phase>,
    pub end_scene_allowed_phase: Vec<Phase>,
}
