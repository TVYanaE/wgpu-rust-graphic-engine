use std::{
    time::Duration,
};
use crate::{
    structures::{
        task_time_cost::TaskTimeCost,
    },
    enums::{
        task_type_enum::TaskType,
        phase_enum::Phase,
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Task {
    pub task_type: TaskType,
    pub phase: Phase,  
    pub task_time_cost: TaskTimeCost,
}
