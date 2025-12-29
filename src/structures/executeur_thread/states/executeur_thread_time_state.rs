use std::{
    time::{Duration},
};
use crate::{
    structures::{
        common_structures::{
            time_budget::TimeBudget,
        },
    },
    enums::{
        time_budget_type_enum::TimeBudgetType,
    },
};

pub struct ExecuteurThreadTimeState {
    pub render_time_budget: TimeBudget,
    pub logic_time_budget: TimeBudget,
}

impl Default for ExecuteurThreadTimeState {
    fn default() -> Self {
        let render_time_budget = TimeBudget::new(
            TimeBudgetType::RenderTimeBudget, 
            Duration::from_millis(17), 
            Duration::from_millis(17)
        );

        let logic_time_budget = TimeBudget::new(
            TimeBudgetType::LogicTimeBudget, 
            Duration::from_millis(34), 
            Duration::from_millis(34)
        );
        
        Self { 
            render_time_budget: render_time_budget, 
            logic_time_budget: logic_time_budget, 
        } 
    }
}
