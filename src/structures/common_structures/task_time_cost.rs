use std::{
    time::Duration,
};
use crate::{
    enums::{
        time_cost_type_enum::TimeCostType,
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TaskTimeCost {
    pub time_cost_type: TimeCostType,
    pub time_cost: Duration,
}
