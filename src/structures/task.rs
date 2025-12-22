
use crate::{
    enums::{
        phase_enum::Phase,
        task_type_enum::TaskType,
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Task {
    pub task_type: TaskType,
    pub work_phase: Phase, 
}
