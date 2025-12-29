use std::{
    rc::Rc,
    cell::RefCell,
};

use crate::{
    structures::{
        control_thread::{
            states::{
                control_thread_phase_state::ControlThreadPhaseState,
                control_thread_scene_state::ControlThreadSceneState,
            },
            buses::{
                control_thread_data_bus::ControlThreadDataBus,
            },
        },
    },
    enums::{
        event_enums::{
            Event,
            IOEvent,
        },
        phase_enum::Phase,
    },
};


pub struct ControlThreadEventEmitter {
    control_thread_phase_state: Rc<RefCell<ControlThreadPhaseState>>,
    control_thread_data_bus: Rc<RefCell<ControlThreadDataBus>>,
    control_thread_scene_state: Rc<RefCell<ControlThreadSceneState>>,
}

impl ControlThreadEventEmitter {
    pub fn new(
        control_thread_phase_state: Rc<RefCell<ControlThreadPhaseState>>,
        control_thread_data_bus: Rc<RefCell<ControlThreadDataBus>>,
        control_thread_scene_state: Rc<RefCell<ControlThreadSceneState>>,
    ) -> Self {
        Self { 
            control_thread_phase_state: control_thread_phase_state, 
            control_thread_data_bus: control_thread_data_bus,
            control_thread_scene_state: control_thread_scene_state,
        }
    }

    pub fn start(&self) {
        let current_phase = self.control_thread_phase_state.borrow().get_current_phase();
       
        let mut events: Vec<Event> = Vec::new();

        match current_phase {
            Phase::UpdatePhase => {
                let game_events = self
                    .control_thread_scene_state
                    .borrow()
                    .get_allowed_game_events(&current_phase)
                    .unwrap();
                
                events.extend(
                    game_events.into_iter().map(|game_event|{
                        Event::from(game_event)
                    })
                );
            },
            Phase::RenderPhase => {
                let game_events = self
                    .control_thread_scene_state
                    .borrow()
                    .get_allowed_game_events(&current_phase)
                    .unwrap();

                events.extend(
                    game_events.into_iter().map(|game_event|{
                        Event::from(game_event)
                    })
                );
            },
            Phase::ExternalEventsPhase => {
                let io_events: Vec<IOEvent> = self
                .control_thread_data_bus
                .borrow_mut()
                .drain_io_event_queue()
                .collect();

                events.extend(
                    io_events.into_iter().map(|io_event|{
                        Event::from(io_event)
                    })
                );
            },
            Phase::Idle => {},
        }

        self.control_thread_data_bus.borrow_mut().push_events(events.into_iter());
    }
}
