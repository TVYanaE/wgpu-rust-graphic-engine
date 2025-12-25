use crate::{
    enums::{
        task_type_enum::TaskType,
        phase_enum::Phase,
        task_priority_enum::TaskPriority,
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Task {
    pub task_type: TaskType,
    pub phase: Phase,  
    pub task_priority: TaskPriority,
}
