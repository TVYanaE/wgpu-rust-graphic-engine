use super::{
    super::{
        states::{
            scene_state::SceneState,
        },
        managers::{
            world_manager::WorldManager,
        },
    },
    Phase
};

const PHASE: Phase = Phase::PostlogicPhase;

pub fn postlogic_phase(
    scene_state: &SceneState,
    world_manager: &WorldManager,
) {
    if !scene_state.is_allowed_phase(&PHASE) {
        return;
    } 

    world_manager.postlogic_phase(); 
}
