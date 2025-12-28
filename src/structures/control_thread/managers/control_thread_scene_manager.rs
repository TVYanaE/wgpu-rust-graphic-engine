use std::{
    rc::Rc,
    cell::RefCell,
    collections::{HashMap,}
};

use crate::{
    structures::{
        control_thread::{
            buses::{
                control_thread_message_bus::ControlThreadMessagesBus,
            },
            states::{
                control_thread_scene_state::ControlThreadSceneState,
            },
        },
        common_structures::{
            scene::Scene,
        },
    },
    enums::{
        messages::{
            control_thread_message_enums::{
                ControlThreadSceneManagerMessage
            },
        },
        scene_type_enum::SceneType,
        phase_enum::Phase,
        game_event_enum::GameEvent,
    },
};

pub struct ControlThreadSceneManager {
    control_thread_message_bus: Rc<RefCell<ControlThreadMessagesBus>>,
    scene_storage: HashMap<SceneType, Scene>,
    control_thread_scene_state: Rc<RefCell<ControlThreadSceneState>>,
}

impl ControlThreadSceneManager {
    pub fn new(
        control_thread_message_bus: Rc<RefCell<ControlThreadMessagesBus>>,
        control_thread_scene_state: Rc<RefCell<ControlThreadSceneState>>,
    ) -> Self {
        let mut scene_storage: HashMap<SceneType, Scene> = HashMap::new();

        // GameplayScene 
        
        let mut gameplay_scene_phases = Vec::new();

        gameplay_scene_phases.push(Phase::Idle);
        gameplay_scene_phases.push(Phase::UpdatePhase);
        gameplay_scene_phases.push(Phase::RenderPhase);
        gameplay_scene_phases.push(Phase::ExternalEventsPhase);

        let mut gameplay_scene_allowed_game_events = HashMap::new(); 

        let update_phase_game_events = vec![GameEvent::LogicCalculation, GameEvent::PrepareRenderState];

        gameplay_scene_allowed_game_events.insert(Phase::UpdatePhase, update_phase_game_events);
        gameplay_scene_allowed_game_events.insert(Phase::RenderPhase, vec![GameEvent::DrawRenderState]);

        scene_storage.insert(
            SceneType::GameplayScene, 
            Scene::new(
                SceneType::GameplayScene,
                gameplay_scene_phases,
                gameplay_scene_allowed_game_events
            )
        );

        Self { 
            control_thread_message_bus: control_thread_message_bus, 
            scene_storage: scene_storage,
            control_thread_scene_state: control_thread_scene_state,
        }
    }
    
    pub fn start(&self) {
        for message in self
            .control_thread_message_bus
            .borrow_mut()
            .drain_scene_manager_message_buffer() {
            match message {
                ControlThreadSceneManagerMessage::GameplayStarted => {
                    self
                    .control_thread_scene_state
                    .borrow_mut()
                    .set_active_scene(
                    self
                        .scene_storage
                        .get(&SceneType::GameplayScene)
                        .cloned()
                    ); 
                },
            }
        }
    }
}
