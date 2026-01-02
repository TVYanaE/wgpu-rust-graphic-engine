use std::{
    sync::Arc, 
    thread::{self, JoinHandle},
    collections::{VecDeque},
};
use flume::{
    Receiver, RecvError
};
use super::{
    signals::{
        LogicThreadInputSignal
    },
    pipelines::{
        logic_pipeline::run_logic_pipeline,
        start_pipeline::run_start_pipeline,
    }, 
    states::{
        scene_state::SceneState,
    },
    managers::{
        scene_manager::{SceneChangeRequest, SceneManager},
        world_manager::WorldManager,
    },
    events::{
        external_event::ExternalEvent,
        game_event::GameEvent,
    },
};
use crate::{
    modules::{
        main_thread::{
            winit_event::WinitEvent,
        },
        shared::{
            double_buffer_bus::DoubleBufferBus,
            render_state::RenderState,
        },
    },
};


pub struct LogicThreadHandler {
    pub handle: JoinHandle<()>,
}

impl LogicThreadHandler {
    pub fn start_thread(
        logic_thread_input_channel_receiver: Receiver<LogicThreadInputSignal>,
        winit_event_bus: Arc<DoubleBufferBus<WinitEvent>>,
        render_state_bus: Arc<DoubleBufferBus<RenderState>>,
    ) -> Self {
        let handle = thread::spawn(move ||{
            let winit_event_bus = winit_event_bus.clone();
            // Init World manager 
            let world_manager = WorldManager::new();

            // Init queues for Event and Event Data 
            let mut external_event_queue: VecDeque<ExternalEvent> = VecDeque::with_capacity(8);
            let mut game_event_queue: VecDeque<GameEvent> = VecDeque::with_capacity(8);

            // Init Scene Manager Requests
            let mut scene_change_requests: VecDeque<SceneChangeRequest> = VecDeque::with_capacity(4); 

            // Init Scene Manager 
            let scene_manager = SceneManager::new();

            // Init Scene State
            let mut scene_state = SceneState::default();

            loop {
                match logic_thread_input_channel_receiver.recv() {
                    Ok(signal) => {
                        match signal {
                            LogicThreadInputSignal::Start => {
                                run_start_pipeline(
                                    &world_manager,
                                    &mut scene_change_requests,
                                    &mut game_event_queue,
                                );
                            },
                            LogicThreadInputSignal::LogicTick => {
                                run_logic_pipeline(
                                    winit_event_bus.clone(),
                                    &mut external_event_queue,
                                    &mut scene_change_requests,
                                    &mut scene_state,
                                    &scene_manager,
                                    &world_manager,
                                    &mut game_event_queue,
                                    render_state_bus.clone(),
                                ); 
                            }
                            LogicThreadInputSignal::Shutdown => {
                                // Graceful shutdown 
                                //
                                break;
                            }
                        }
                    },
                    Err(recv_error) => {
                        match recv_error {
                            RecvError::Disconnected => {
                                break;
                            },
                        }
                    },
                }
            }

        });

        Self { 
            handle: handle 
        }
    }
}
