use std::{
    collections::{VecDeque},
    sync::{Arc}
};
use crate::{
    modules::{
        shared::{
            double_buffer_bus::DoubleBufferBus,
            render_state::RenderState,
        },
        main_thread::{
            winit_event::WinitEvent
        },
    },
};
use super::super::{
    events::{
        external_event::{ExternalEvent}  
    },
    phases::{
        external_event_collecting_phase::external_event_collecting_phase,
        external_event_handling_phase::external_event_handling_phase,
        scene_change_phase::scene_change_phase,
        spawn_phase::spawn_phase,
        despawn_phase::despawn_phase,
        prelogic_phase::prelogic_phase,
        simulation_phase::simulation_phase,
        postlogic_phase::postlogic_phase,
        prepare_render_state_phase::prepare_render_state_phase,
    },
    states::{
        scene_state::SceneState
    },
    managers::{
        scene_manager::{SceneManager, SceneChangeRequest},
        world_manager::{WorldManager},
    },
    events::{
        game_event::{GameEvent}
    },
};


pub fn run_logic_pipeline(
    winit_event_bus: Arc<DoubleBufferBus<WinitEvent>>,
    external_event_queue: &mut VecDeque<ExternalEvent>,
    scene_change_requests: &mut VecDeque<SceneChangeRequest>,
    scene_state: &mut SceneState,
    scene_manager: &SceneManager,
    world_manager: &WorldManager, 
    game_event_queue: &mut VecDeque<GameEvent>,
    render_state_bus: Arc<DoubleBufferBus<RenderState>>,
) {
    external_event_collecting_phase(winit_event_bus, external_event_queue);
    scene_change_phase(
        scene_change_requests,
        scene_state,
        scene_manager,
    );
    external_event_handling_phase(external_event_queue, world_manager);
    spawn_phase(scene_state, world_manager, game_event_queue);
    despawn_phase(world_manager, game_event_queue, scene_state);
    prelogic_phase(world_manager, scene_state);
    simulation_phase(world_manager, scene_state);
    postlogic_phase(scene_state, world_manager);
    prepare_render_state_phase(scene_state, world_manager, render_state_bus);
}
