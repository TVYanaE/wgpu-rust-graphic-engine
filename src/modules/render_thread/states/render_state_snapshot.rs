use std::{
    time::{
        Instant,
        Duration,
    },
};
use crate::{
    modules::{
        shared::{
            render_state::RenderState,
        },
    },
};

#[derive(Debug)]
pub struct RenderStateSnapshot {
    pub prev_render_state_snapshot: Option<RenderState>,
    pub curr_render_state_snapshot: Option<RenderState>,
    pub t_curr: Option<Instant>,
    pub delta_time_logic: Option<Duration>, 
}

impl Default for RenderStateSnapshot {
    fn default() -> Self {
        Self { 
            prev_render_state_snapshot: None, 
            curr_render_state_snapshot: None, 
            t_curr: None, 
            delta_time_logic: Some(Duration::from_millis(34)) 
        }
    }
}
