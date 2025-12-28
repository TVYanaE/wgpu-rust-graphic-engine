use std::{
    time::Duration,
};
use crate::{
    enums::{
        time_cost_type_enum::TimeCostType,
    }
};

#[derive(Debug, Clone, Copy)]
pub struct TaskChunkTimeCost {
    pub time_cost_type: TimeCostType,
    pub time_cost: Duration,
}
