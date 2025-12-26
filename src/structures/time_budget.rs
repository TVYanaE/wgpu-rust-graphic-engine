use std::{
    time::Duration,
};
use crate::{
    enums::{
        time_budget_type_enum::TimeBudgetType,
    },
};

#[derive(Debug, Clone, Copy)]
pub struct TimeBudget {
    time_budget_type: TimeBudgetType,
    current_budget: Duration,
    max_budget: Duration,
    avaiable_budget: Duration,
}

impl TimeBudget {
    pub fn new(
        time_budget_type: TimeBudgetType,
        max_budget: Duration, 
        current_budget: Duration,
    ) -> Self {
        Self { 
            time_budget_type: time_budget_type,
            current_budget: current_budget,
            max_budget: max_budget,
            avaiable_budget: current_budget,
        }
    }

    pub fn set_current_budget(&mut self, current_budget: Duration) {
        if current_budget < self.max_budget {
            self.current_budget = current_budget;
        } 
        else {
            self.current_budget = self.max_budget;
        } 
    }

    pub fn get_avaiable_budget(&mut self) -> Option<Duration> {
        if self.avaiable_budget == Duration::ZERO {
            return None;
        }
        else {
            let avaiable_budget = self.avaiable_budget;
            self.avaiable_budget = Duration::ZERO;
            return Some(avaiable_budget);
        }
    }

    pub fn refresh_avaiable_budget(&mut self) {
        self.avaiable_budget = self.current_budget;
    }
}

