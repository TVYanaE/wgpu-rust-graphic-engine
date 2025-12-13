use crate::{
    enums::{
        frame_phase_name_enum::FramePhaseName,
        component_name_enum::ComponentName,
        task_name_enum::TaskName,
    },
};

#[derive(Debug, Clone)]
pub struct TaskDescriptor {
    pub task_name: TaskName,
    pub frame_phase: FramePhaseName,
    pub read_components: Vec<ComponentName>,
    pub write_components: Vec<ComponentName>
}
