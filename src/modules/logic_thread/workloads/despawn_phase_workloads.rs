
use shipyard::{
    AllStoragesViewMut, View, Workload, IntoIter,
};
use super::super::{
    components::{
        dead_component::DeadComponent,
    },
};



pub fn get_despawn_phase_workloads() -> Vec<Workload> {
    let despawn_workload = Workload::new("DespawnWorkload")
    .with_system(despawn);
        
    return vec![despawn_workload];
}

fn despawn(
    mut all_storages: AllStoragesViewMut,
) { 

    let dead_entities: Vec<_> = {
        let dead_view = all_storages.borrow::<View<DeadComponent>>().unwrap();
        dead_view.iter().with_id().map(|(id, _)| id).collect()
    };

    for entity_id in dead_entities {
        all_storages.delete_entity(entity_id);
    }

/* 
    for (entity_id, _) in view_dead_component.iter().with_id(){
        all_storages.delete_entity(entity_id);
    }  */
}
