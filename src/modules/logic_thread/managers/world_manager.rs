use std::{
    sync::Arc,
};
use winit::{
    event::{
        KeyEvent,
    },
}; 
use shipyard::{ 
    World,
    UniqueViewMut,
    UniqueView,
    Workload,
};
use crate::{
    modules::{
        shared::{
            double_buffer_bus::DoubleBufferBus,
            render_state::RenderState,
        },
    },
};
use super::super::{
    user_input::{
        keyboard_state::KeyboardState,
        input_bindings::InputBindings,
    },
    uniques::{
        spawn_intention::SpawnIntention,
    },
    game_objects::{
        player::PlayerDescriptor, 
        enemy::EnemyDescriptor,
        map_object::MapObjectDescriptor,
    },
    workloads::{
        spawn_phase_workloads::get_spawn_phase_workloads,
        despawn_phase_workloads::get_despawn_phase_workloads,
        prelogic_phase_workloads::get_prelogic_phase_workloads,
        simulation_phase_workloads::{
            get_movement_sub_phase_workloads,
            get_combat_sub_phase_workloads, 
            get_finalisation_sub_phase_workloads,
        },
        prepare_render_state_phase_workloads::get_prepare_render_state_phase_workloads,
    },
    components::{
        camera_component::CameraComponent,
    },
    controllers::{
        IntentionQuery,
    },
};

pub struct WorldManager {
    world: World
} 

impl WorldManager {
    pub fn new() -> Self {
        let mut world = World::default();

        world.add_entity(CameraComponent::default());

        Self { 
            world: world,
        }
    }

    pub fn init_uniques(&self) {
        self.world.add_unique(KeyboardState::new());
        self.world.add_unique(InputBindings::default());
        self.world.add_unique(SpawnIntention::default());
        self.world.add_unique(IntentionQuery::default());
        self.world.add_unique(RenderState::default());
    } 

    pub fn init_workloads(&self) {
        let mut workloads: Vec<Workload> = Vec::with_capacity(32);

        let spawn_phase_workloads = get_spawn_phase_workloads();
        let despawn_phase_workloads = get_despawn_phase_workloads(); 
        let prelogic_phase_workloads = get_prelogic_phase_workloads();
        let mut simulation_phase_workloads: Vec<Workload> = Vec::new();
        let prepare_render_state_phase_workloads = get_prepare_render_state_phase_workloads();
        
        let movement_sub_phase_workloads = get_movement_sub_phase_workloads();
        let combat_sub_phase_workload = get_combat_sub_phase_workloads();
        let finalisation_sub_phase_workload = get_finalisation_sub_phase_workloads();

        simulation_phase_workloads.extend(movement_sub_phase_workloads.into_iter());
        simulation_phase_workloads.extend(combat_sub_phase_workload.into_iter());
        simulation_phase_workloads.extend(finalisation_sub_phase_workload.into_iter());
           
        workloads.extend(spawn_phase_workloads.into_iter());
        workloads.extend(despawn_phase_workloads.into_iter());
        workloads.extend(prelogic_phase_workloads.into_iter());
        workloads.extend(simulation_phase_workloads.into_iter());
        workloads.extend(prepare_render_state_phase_workloads.into_iter());

        for workload in workloads {
            workload.add_to_world(&self.world);
        }
    }

    pub fn key_event_handling(&self, key_event: KeyEvent) {
        let mut keyboard_state = self.world
        .borrow::<UniqueViewMut<KeyboardState>>()
        .unwrap();

        keyboard_state.clear_temp_sets();
        keyboard_state.register_key_event(key_event);
    }

    pub fn register_spawn_intention(
        &self,
        players_for_spawn: Vec<PlayerDescriptor>,
        enemies_for_spawn: Vec<EnemyDescriptor>,
        map_objects_for_spawn: Vec<MapObjectDescriptor>, 
    ) {
        let mut spawn_intention = self.world
        .borrow::<UniqueViewMut<SpawnIntention>>()
        .unwrap();

        spawn_intention.players_for_spawn.extend(players_for_spawn.into_iter());
        spawn_intention.enemies_for_spawn.extend(enemies_for_spawn.into_iter());
        spawn_intention.map_objects_for_spawn.extend(map_objects_for_spawn.into_iter());
    }

    pub fn start_spawn(&self) {
        if let Err(e) = self.world.run_workload("SpawnWorkload") {
            panic!("SpawnWorkload failed: {e:?}");
        }
        //self.world.run_workload("SpawnWorkload");
    }

    pub fn start_despawn(&self) {
        self.world.run_workload("DespawnWorkload");
    }

    pub fn start_prelogic(&self) {
        self.world.run_workload("PrelogicWorkload");
    }

    pub fn simulation_phase(&self) {
        // Movement sub phase
        self.world.run_workload("PlayerMovementWorkload"); 
        self.world.run_workload("NetworkMovementWorkload"); 
        self.world.run_workload("AIMovementWorkload");

        // Combat sub phase 
        self.world.run_workload("PlayerCombatWorkload"); 
        self.world.run_workload("NetworkCombatWorkload"); 
        self.world.run_workload("AICombatWorkload");

        // Finalisation 
        self.world.run_workload("FinalisationWorkload");
    }

    pub fn postlogic_phase(&self) {
        
    }

    pub fn prepare_render_state_phase(
        &self,  
        render_state_bus: Arc<DoubleBufferBus<RenderState>>,
    ) {
        self.world.run_workload("PrepareRenderState"); 

        let render_state = self.world.borrow::<UniqueView<RenderState>>().unwrap().clone();

        render_state_bus.push(render_state);
        render_state_bus.swap();
    }
}
