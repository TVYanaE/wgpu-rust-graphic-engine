use shipyard::{
    Workload,
    EntitiesViewMut,
    UniqueViewMut,
    UniqueView,
    ViewMut,
};

use super::super::{
    uniques::{
        spawn_intention::SpawnIntention,
    },
    components::{
        size_component::SizeComponent,
        position_component::PositionComponent,
        sprite_component::SpriteComponent,
        player_component::PlayerComponent,
    }, 
};

pub fn get_spawn_phase_workloads() -> Vec<Workload> {
    let spawn_workload = Workload::new("SpawnWorkload")
    .with_system(spawn_entities)
    .with_system(clean);
    

    return vec![spawn_workload];
}


fn spawn_entities(
    mut entities: EntitiesViewMut,
    view_spawn_intention: UniqueView<SpawnIntention>,
    mut vm_pos: ViewMut<PositionComponent>,
    mut vm_size: ViewMut<SizeComponent>,
    mut vm_sprite: ViewMut<SpriteComponent>,
    mut vm_player: ViewMut<PlayerComponent>,
) {
    
    view_spawn_intention.players_for_spawn.iter().for_each(|player_descriptor|{
        entities.add_entity(
            (
                &mut vm_pos, 
                &mut vm_size, 
                &mut vm_sprite, 
                &mut vm_player
            ), 
            (
                player_descriptor.position_component, 
                player_descriptor.size_component, 
                player_descriptor.sprite_component, 
                player_descriptor.player_component
            )
        );
    });

    view_spawn_intention.enemies_for_spawn.iter().for_each(|enemy_descriptor|{
        entities.add_entity(
            (
                &mut vm_pos, 
                &mut vm_size, 
                &mut vm_sprite, 
            ), 
            (
                enemy_descriptor.position_component, 
                enemy_descriptor.size_component, 
                enemy_descriptor.sprite_component, 
            )
        );
    });
    
    view_spawn_intention.map_objects_for_spawn.iter().for_each(|map_object_descriptor|{
        entities.add_entity(
            (
                &mut vm_pos, 
                &mut vm_size, 
                &mut vm_sprite, 
            ), 
            (
                map_object_descriptor.position_component, 
                map_object_descriptor.size_component, 
                map_object_descriptor.sprite_component, 
            )
        );
    });
}

fn clean(
    mut spawn_intention: UniqueViewMut<SpawnIntention>
) {
    spawn_intention.players_for_spawn.clear();
    spawn_intention.enemies_for_spawn.clear();
    spawn_intention.map_objects_for_spawn.clear();
}
