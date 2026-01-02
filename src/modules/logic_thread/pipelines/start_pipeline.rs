use std::{
    collections::VecDeque,
};
use glam::{
    vec3,
};
use super::{
    super::{
        managers::{
            world_manager::WorldManager,
            scene_manager::SceneChangeRequest,
        },
        scene::{SceneName},
        events::{
            game_event::{GameEvent, GameEventData},
        },
        game_objects::{
            player::PlayerDescriptor,
        },
        components::{
            position_component::PositionComponent,
            size_component::SizeComponent,
            sprite_component::SpriteComponent,
            player_component::PlayerComponent
        },
        phases::{
            Phase,
        },
    },
};

pub fn run_start_pipeline(
    world_manager: &WorldManager,
    scene_change_requests: &mut VecDeque<SceneChangeRequest>,
    game_event_queue: &mut VecDeque<GameEvent>,
) {
    world_manager.init_uniques();
    world_manager.init_workloads();

    scene_change_requests.push_back(SceneChangeRequest::StartScene(SceneName::Gameplay));

    let player_descriptors = vec![PlayerDescriptor{
        position_component: PositionComponent { position: vec3(0.0, 0.0, 1.0) },
        size_component: SizeComponent { size_x: 1.0, size_y: 1.0, size_z: 0.0 },
        sprite_component: SpriteComponent { material_id: 0 },
        player_component: PlayerComponent, 
    }]; 

    game_event_queue.push_back(GameEvent { 
        event_data: GameEventData::SpawnPlayers(player_descriptors), 
        phase: Phase::SpawnPhase,
    });

}
