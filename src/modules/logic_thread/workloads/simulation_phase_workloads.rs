use glam::{
    vec3,
};
use shipyard::{
    IntoIter, UniqueViewMut, UniqueView, View, ViewMut, Workload
};
use super::{
    super::{
        controllers::{
            IntentionQuery,
            IntentionKind,
            player_controllers::{
                PlayerIntention
            },
        },
        components::{
            player_component::PlayerComponent,
            position_component::PositionComponent,
        },
    },
};

// Movement sub phase 
// 1) player -> network -> AI 
pub fn get_movement_sub_phase_workloads() -> Vec<Workload> {
    let player_movement_workload = Workload::new("PlayerMovementWorkload")
    .with_system(player_movement);
    
    let network_movement_workload = Workload::new("NetworkMovementWorkload")
    .with_system(network_movement); 
    
    let ai_movement_workload = Workload::new("AIMovementWorkload")
    .with_system(ai_movement);

    return vec![player_movement_workload, network_movement_workload, ai_movement_workload];
}


fn player_movement(
    intentions: UniqueView<IntentionQuery>,
    view_player: View<PlayerComponent>,
    mut view_position: ViewMut<PositionComponent>,
) {
    for intention in intentions.0.iter() {
        match &intention.kind {
            IntentionKind::Player(player_intention) => {
                match player_intention {
                    PlayerIntention::Move(move_descriptor) => {
                        for (
                            _player, 
                            position_component) in (&view_player, &mut view_position).iter() {
                            position_component.position += vec3(
                                move_descriptor.movement_vector.x,
                                move_descriptor.movement_vector.y, 0.0
                            );
                        }
                    }
                }
            }
            IntentionKind::AI(_ai_intention) => {},
            IntentionKind::Network(_network_intention) => {},
        }
    }
}

fn network_movement () {

}

fn ai_movement () {

}

pub fn get_combat_sub_phase_workloads() -> Vec<Workload> {
    let player_combat_workload = Workload::new("PlayerCombatWorkload")
    .with_system(player_combat);
   
    let network_combat_workload = Workload::new("NetworkCombatWorkload")
    .with_system(network_combat);
   
    let ai_combat_workload = Workload::new("AICombatWorkload")
    .with_system(ai_combat);
    
    return vec![player_combat_workload, network_combat_workload, ai_combat_workload];
} 

fn player_combat(

) {

}

fn network_combat(

) {

}

fn ai_combat(

) {

}

pub fn get_finalisation_sub_phase_workloads() -> Vec<Workload> {
    let finalisation_workload = Workload::new("FinalisationWorkload")
    .with_system(finalisation);

    return vec![finalisation_workload];
}

fn finalisation(
    mut intentions: UniqueViewMut<IntentionQuery>,
) {
    intentions.0.clear(); 
}
