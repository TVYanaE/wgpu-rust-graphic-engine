
use crate::{
    enums::{
        frame_phase_enum::FramePhaseName,
        component_name_enum::ComponentName,
    },
};

#[derive(Debug, Clone)]
pub struct EventDescriptor {
    pub frame_phase: FramePhaseName,
    pub read_component: Vec<ComponentName>,
    pub write_component: Vec<ComponentName>
}
