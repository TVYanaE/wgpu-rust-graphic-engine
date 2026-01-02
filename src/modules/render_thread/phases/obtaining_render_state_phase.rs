use std::{
    sync::{Arc},
    time::{
        Instant,
    },
};
use crate::{
    modules::{
        shared::{
            render_state::{RenderState},
            double_buffer_bus::DoubleBufferBus
        },
    },
};
use super::{
    super::{
        states::{
            render_state_snapshot::RenderStateSnapshot,
        },
    },
};

pub fn obtaining_render_state_phase(
    render_state_bus: Arc<DoubleBufferBus<RenderState>>,
    render_state_snapshot: &mut RenderStateSnapshot,
) {
    let read_buffer = render_state_bus.get_read_buffer().as_ref().clone();

    let new_render_state = read_buffer[0].clone();
    
    if let Some(current_render_state) = &render_state_snapshot.curr_render_state_snapshot {
        if current_render_state == &new_render_state {
            return;
        }
        else {
            render_state_snapshot.prev_render_state_snapshot = render_state_snapshot.curr_render_state_snapshot.clone();
            render_state_snapshot.curr_render_state_snapshot = Some(new_render_state);
            render_state_snapshot.t_curr = Some(Instant::now());
        }
    }
    else {
        render_state_snapshot.curr_render_state_snapshot = Some(new_render_state);
        render_state_snapshot.t_curr = Some(Instant::now());
    }

}
