use std::{
    collections::{VecDeque},
    sync::{Arc},
};
use crate::{
    modules::{
        shared::{
            double_buffer_bus::DoubleBufferBus,
            render_state::RenderState,
        },
        main_thread::{
            winit_event::WinitEvent,
        },
    },
};
use super::{
    super::{
        phases::{
            external_event_collecting_phase::external_event_collecting_phase,
            external_event_handling_phase::external_event_handling_phase,
            obtaining_render_state_phase::obtaining_render_state_phase,
            render_prepare_phase::render_prepare_phase,
            render_phase::render_phase,
        },
        events::{
            external_event::ExternalEvent,
        },
        states::{
            gpu_state::{
                state::GPUState,
                camera::ViewProjectionUniformMatrixCache, 
            },
            render_state_snapshot::RenderStateSnapshot,
        },
        render_batch::RenderBatch,
    },
};

pub fn run_render_pipeline(
    winit_event_bus: Arc<DoubleBufferBus<WinitEvent>>,
    external_event_queue: &mut VecDeque<ExternalEvent>, 
    gpu_state: &mut GPUState,
    render_state_bus: Arc<DoubleBufferBus<RenderState>>,
    render_batches_cache: &mut Vec<RenderBatch>,
    view_projection_uniform_matrix_cache: &mut ViewProjectionUniformMatrixCache,
    render_state_snapshot: &mut RenderStateSnapshot,
) {
    external_event_collecting_phase(winit_event_bus, external_event_queue);
    external_event_handling_phase(external_event_queue, gpu_state);
    
    let size = gpu_state.window.inner_size();

    if size.width != gpu_state.surface_configuration.width
        || size.height != gpu_state.surface_configuration.height
    {
        gpu_state.surface_configuration.width = size.width;
        gpu_state.surface_configuration.height = size.height;
        gpu_state.surface.configure(&gpu_state.device, &gpu_state.surface_configuration);
    }

    obtaining_render_state_phase(render_state_bus, render_state_snapshot);
    render_prepare_phase(
        &gpu_state.device, 
        render_batches_cache, 
        &gpu_state.material_manager,
        view_projection_uniform_matrix_cache,
        gpu_state.surface_configuration.width as f32,
        gpu_state.surface_configuration.height as f32,
        render_state_snapshot,
    );
    render_phase(gpu_state, render_batches_cache, view_projection_uniform_matrix_cache);
}


