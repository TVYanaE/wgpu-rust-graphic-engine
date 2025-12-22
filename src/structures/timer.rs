use std::time::{Instant, Duration};
use flume::{Sender};
use crate::{
    enums::{
        signals::{
            control_thread_signal_enums::ControlThreadInputSignal,
        },
        event_enum::Event,
    },
};

pub struct Timer {
    last_time_check: Instant,
    delta: Duration,
    frame_threshold: Duration,
    logic_threshold: Duration,
    frame_time_accumulator: Duration,
    logic_time_accumulator: Duration,
    control_thread_input_channel_sender: Sender<ControlThreadInputSignal>,
}


impl Timer {
    pub fn new(control_thread_input_channel_sender: Sender<ControlThreadInputSignal>) -> Self {
        Self { 
            last_time_check: Instant::now(), 
            delta: Duration::ZERO, 
            frame_threshold: Duration::from_millis(17),
            logic_threshold: Duration::from_millis(4),
            logic_time_accumulator: Duration::ZERO,
            frame_time_accumulator: Duration::ZERO,
            control_thread_input_channel_sender
        }
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        self.delta = now - self.last_time_check;
        self.last_time_check = now;
 
        self.frame_time_accumulator += self.delta;
        self.logic_time_accumulator += self.delta;
    }
        
    pub fn check_logic_threshold(&mut self) {
        if self.logic_time_accumulator >= self.logic_threshold {
            self.logic_time_accumulator -= self.logic_threshold;
            self.control_thread_input_channel_sender.send(ControlThreadInputSignal::LogicStart);
        }
        else {
            return;
        }
    }

    pub fn check_frame_threshold(&mut self) {
        if self.frame_time_accumulator >= self.frame_threshold {
            self.frame_time_accumulator = Duration::ZERO;
            self.control_thread_input_channel_sender.send(ControlThreadInputSignal::FrameStart);
        }
        else {
            return;
        }
    }
}
