use std::time::{Instant, Duration};
use flume::{Sender};
use crate::{
    enums::{
        input_event_enum::InputEvent,
    },
};

pub struct Timer {
    last_instant: Instant,
    delta: Duration,
    fixed_step: Duration,
    accumulator: Duration,
    input_event_channel_sender: Sender<InputEvent>,
}


impl Timer {
    pub fn new(input_event_channel_sender: Sender<InputEvent>) -> Self {
        Self { 
            last_instant: Instant::now(), 
            delta: Duration::ZERO, 
            fixed_step: Duration::from_millis(17), 
            accumulator: Duration::ZERO, 
            input_event_channel_sender: input_event_channel_sender
        }
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        self.delta = now - self.last_instant;
        self.last_instant = now;
 
        self.accumulator += self.delta;
    }
        
    pub fn check_step_fixed(&mut self) {
        if self.accumulator >= self.fixed_step {
            self.accumulator -= self.fixed_step;
            self.input_event_channel_sender.send(InputEvent::FrameStart);
        }
        else {
            return;
        }
    } 
}
