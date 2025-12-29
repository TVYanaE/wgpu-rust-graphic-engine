use crate::{
    structures::{
        common_structures::{
            task_time_cost::TaskTimeCost,
        },
    },
    enums::{
        task_type_enum::TaskType,
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Task {
    pub task_type: TaskType,
    pub task_time_cost: TaskTimeCost,
}
