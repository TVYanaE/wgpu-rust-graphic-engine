use std::{
    time::{Duration},
};
use crate::{
    structures::{
        time_budget::TimeBudget,
        time_menu::TimeMenu,
    },
    enums::{
        time_budget_type_enum::TimeBudgetType,
    },
};

pub struct TimeState {
    pub time_menu: TimeMenu,
    pub render_time_budget: TimeBudget,
    pub logic_time_budget: TimeBudget,
}

impl Default for TimeState {
    fn default() -> Self {
        let time_menu = TimeMenu::new();

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
            time_menu: time_menu, 
            render_time_budget: render_time_budget, 
            logic_time_budget: logic_time_budget, 
        } 
    }
}
