use std::{
    sync::Arc,
};
use crate::{
    modules::{
        shared::{
            double_buffer_bus::DoubleBufferBus,
            render_state::RenderState,
        },
    },
};
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

const PHASE: Phase = Phase::PrepareRenderStatePhase;

pub fn prepare_render_state_phase(
    scene_state: &SceneState,
    world_manager: &WorldManager,
    render_state_bus: Arc<DoubleBufferBus<RenderState>>,
) {
    if !scene_state.is_allowed_phase(&PHASE) {
        return;
    }

    world_manager.prepare_render_state_phase(render_state_bus);
}
