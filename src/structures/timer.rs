use std::time::{Instant, Duration};

pub struct Timer {
    last_instant: Instant,
    delta: Duration,
    fixed_step: Duration,
    accumulator: Duration,
}


impl Timer {
    pub fn new() -> Self {
        Self { 
            last_instant: Instant::now(), 
            delta: Duration::ZERO, 
            fixed_step: Duration::from_millis(16), 
            accumulator: Duration::ZERO, 
        }
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        self.delta = now - self.last_instant;
        self.last_instant = now;
 
        self.accumulator += self.delta;
    }
        
    pub fn should_step_fixed(&mut self) -> bool {
        if self.accumulator >= self.fixed_step {
            self.accumulator -= self.fixed_step;
            return true;
        }
        else {
            return false;
        }
    }

    pub fn dt(&self) -> f32 {
        self.delta.as_secs_f32()
    }
}
