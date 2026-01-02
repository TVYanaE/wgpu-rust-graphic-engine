
use shipyard::{
    Workload
};
use super::super::{
    controllers::{
        player_controllers::{
            player_movement_controller::player_movement_controller,
        },
        network_controller::network_controller,
        ai_controller::ai_controller
    },
};

pub fn get_prelogic_phase_workloads() -> Vec<Workload> {
    let controllers_workload = Workload::new("PrelogicWorkload")
    .with_system(player_movement_controller)
    .with_system(network_controller)
    .with_system(ai_controller);
        
    return vec![controllers_workload];
}


