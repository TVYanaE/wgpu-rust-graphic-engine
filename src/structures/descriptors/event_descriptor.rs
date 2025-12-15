use crate::{
    enums::{
        frame_phase_name_enum::FramePhaseName,
        component_name_enum::ComponentName,
    },
};

#[derive(Debug, Clone)]
pub struct EventDescriptor {
    pub read_components: Vec<ComponentName>,
    pub write_components: Vec<ComponentName>
}
