use std::{
    collections::{VecDeque},
};
use super::{
    super::{
        events::{
            external_event::ExternalEvent,
        },
        states::{
            gpu_state::{
                state::GPUState,
            },
        },
    },
};

pub fn external_event_handling_phase(
    external_event_queue: &mut VecDeque<ExternalEvent>, 
    gpu_state: &mut GPUState,
) {
    for external_event in external_event_queue.drain(..) {
        match external_event {
            ExternalEvent::Resize(physical_size) => {
                gpu_state.reconfigure_surface(physical_size);
            },
            ExternalEvent::RedrawRequested => {
                
                let size = gpu_state.window.inner_size();

                if size.width != gpu_state.surface_configuration.width
                    || size.height != gpu_state.surface_configuration.height
                {
                    gpu_state.surface_configuration.width = size.width;
                    gpu_state.surface_configuration.height = size.height;
                    gpu_state.surface.configure(&gpu_state.device, &gpu_state.surface_configuration);
                }
            },
        } 
    }
}
