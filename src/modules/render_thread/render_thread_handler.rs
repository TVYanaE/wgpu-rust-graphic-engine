use std::{
    thread::{self, JoinHandle,},
    sync::{Arc},
    collections::{VecDeque},
};
use winit::{
    window::Window,
};
use flume::{
    Receiver, RecvError
};
use super::{
    states::{
        gpu_state::{
            state::{
                GPUState,
            },
            camera::ViewProjectionUniformMatrixCache,
        },
    },
    events::{
        external_event::ExternalEvent,
    },
    thread_pipelines::{
        start_pipeline::run_start_pipeline,
        render_pipeline::run_render_pipeline,
    },
    signals::RenderThreadInputSignal,
    render_batch::RenderBatch,
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


pub struct RenderThreadHandler {
    pub handle: JoinHandle<()>
}


impl RenderThreadHandler {
    pub async fn start_thread(
        window: Arc<Window>,
        render_thread_input_channel_receiver: Receiver<RenderThreadInputSignal>,
        winit_event_bus: Arc<DoubleBufferBus<WinitEvent>>,
        render_state_bus: Arc<DoubleBufferBus<RenderState>>,
    ) -> Self {
        let handle = thread::spawn(move ||{
            // Init GPU Resources
            let mut gpu_state = pollster::block_on(GPUState::new(window));

            // Init buffers for events 
            let mut external_event_queue: VecDeque<ExternalEvent> = VecDeque::with_capacity(8); 

            // Init cache for render batches
            let mut render_batches_cache: Vec<RenderBatch> = Vec::with_capacity(8); 

            // Init cache for ViewProject Matrix
            let mut view_projection_uniform_matrix_cache = 
            ViewProjectionUniformMatrixCache::default();

            loop {
                match render_thread_input_channel_receiver.recv() {
                    Ok(signal) => {
                        match signal {
                            RenderThreadInputSignal::FrameTick => {
                                run_render_pipeline(
                                    winit_event_bus.clone(),
                                    &mut external_event_queue,
                                    &mut gpu_state,
                                    render_state_bus.clone(),
                                    &mut render_batches_cache,
                                    &mut view_projection_uniform_matrix_cache,
                                );
                            },
                            RenderThreadInputSignal::Start => {
                                run_start_pipeline();
                            },
                            RenderThreadInputSignal::Shutdown => {
                                break;
                            },
                        }
                    },
                    Err(recv_error) => {
                        match recv_error {
                            RecvError::Disconnected => {
                                break;
                            },
                        } 
                    }
                }
            }
        });

        Self {
            handle: handle
        }
    } 
}
