use std::{
    time::{Instant, Duration},
};

pub struct Timer {
    last_time_check: Instant,
    delta: Duration,
    frame_threshold: Duration,
    logic_threshold: Duration,
    frame_time_accumulator: Duration,
    logic_time_accumulator: Duration,
}


impl Timer {
    pub fn new() -> Self {
        Self { 
            last_time_check: Instant::now(), 
            delta: Duration::ZERO, 
            frame_threshold: Duration::from_millis(17),
            logic_threshold: Duration::from_millis(34),
            logic_time_accumulator: Duration::ZERO,
            frame_time_accumulator: Duration::ZERO,
        }
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        self.delta = now - self.last_time_check;
        self.last_time_check = now;
 
        self.frame_time_accumulator += self.delta;
        self.logic_time_accumulator += self.delta;
    }
        
    pub fn check_logic_threshold(&mut self) -> bool {
        if self.logic_time_accumulator >= self.logic_threshold {
            self.logic_time_accumulator -= self.logic_threshold;
            
            return true;
        }
        else {
            return false;
        }
    }

    pub fn check_frame_threshold(&mut self) -> bool {
        if self.frame_time_accumulator >= self.frame_threshold {
            self.frame_time_accumulator = Duration::ZERO;

            return true;
        }
        else {
            return false;
        }
    }
}
