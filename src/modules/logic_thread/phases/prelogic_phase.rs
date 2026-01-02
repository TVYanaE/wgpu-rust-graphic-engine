
use super::{
    super::{
        states::{
            scene_state::SceneState,
        },
        managers::{
            world_manager::WorldManager,
        },
    },
    Phase,
};

const PHASE: Phase = Phase::PrelogicPhase;

pub fn prelogic_phase(
    world_manager: &WorldManager,
    scene_state: &SceneState,
) {
    if !scene_state.is_allowed_phase(&PHASE) {
        return;
    }
    
    world_manager.start_prelogic();
    
} 
