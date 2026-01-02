use std::{
    collections::{VecDeque},
};
use super::super::{
    managers::{
        scene_manager::{
            SceneChangeRequest,
            SceneManager,
        },
    },
    scene::{
        ScenePhase
    },
    states::{
        scene_state::SceneState,
    },
};

pub fn scene_change_phase(
    scene_change_requests: &mut VecDeque<SceneChangeRequest>,
    scene_state: &mut SceneState,
    scene_manager: &SceneManager,
) {
    if let Some(scene_change_request) = scene_change_requests.pop_front() {
        match scene_change_request {
            SceneChangeRequest::StartScene(scene_name) => {
                let current_scene = scene_manager.get_scene(&scene_name).unwrap();

                scene_state.set_current_scene(current_scene);
                scene_state.set_current_scene_phase(ScenePhase::BeginScene);
            },
            SceneChangeRequest::EndScene => {
                scene_state.set_current_scene_phase(ScenePhase::EndScene);
            }
        }
    }
    else {
        scene_state.set_current_scene_phase(ScenePhase::DuringScene);
    }
}
