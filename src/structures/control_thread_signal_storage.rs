
use flume::{
    Receiver, TryRecvError
};
use crate::{
    enums::{
        signals::{
            control_thread_signal_enums::ControlThreadInputSignal,
        },
        phase_enum::Phase,
        errors::ControlThreadSignalStorageError,
    },
};

pub struct ControlThreadSignalStorage {
    control_thread_input_channel_receiver: Receiver<ControlThreadInputSignal>,
    logic_signal: u16,
    frame_signal: bool,
    init_signal: bool,
    shutdown_signal: bool,
}

impl ControlThreadSignalStorage {
    pub fn new(
        control_thread_input_channel_receiver: Receiver<ControlThreadInputSignal>,
    ) -> Self {
        Self {
            control_thread_input_channel_receiver: control_thread_input_channel_receiver,
            logic_signal: 0, 
            frame_signal: false, 
            init_signal: false, 
            shutdown_signal: false, 
        }
    }

    pub fn start(&mut self) -> Result<(), ControlThreadSignalStorageError> {
        loop {
            match self.control_thread_input_channel_receiver.try_recv() {
                Ok(control_thread_input_signal) => {
                    match control_thread_input_signal {
                        ControlThreadInputSignal::Shutdown => {
                            self.shutdown_signal = true;
                        },
                        ControlThreadInputSignal::Init => {
                            self.init_signal = true;
                        },
                        ControlThreadInputSignal::LogicStart => {
                            self.logic_signal += 1;
                        },
                        ControlThreadInputSignal::FrameStart => {
                            self.frame_signal = true;
                        },
                    } 
                },
                Err(try_recv_error) => {
                    match try_recv_error {
                        TryRecvError::Empty => { break; },
                        TryRecvError::Disconnected => { 
                            return Err(ControlThreadSignalStorageError::ChannelClosedError(try_recv_error)); 
                        }
                    }
                },
            }
        }

        return Ok(()); 
    }

    pub fn get_priority_signal(&self) -> Option<ControlThreadInputSignal> {
        if self.shutdown_signal == true {
            Some(ControlThreadInputSignal::Shutdown)
        } 
        else if self.init_signal == true {
            Some(ControlThreadInputSignal::Init)
        }
        else if self.frame_signal == true {
            Some(ControlThreadInputSignal::FrameStart)
        }
        else if self.logic_signal > 0 {
            Some(ControlThreadInputSignal::LogicStart)
        }
        else {
            None
        }
    }

    pub fn consume_signal_for_phase(&mut self, phase: Phase) {
        match phase {
            Phase::ShutdownPhase => {
                self.shutdown_signal = false;
            },
            Phase::InitPhase => {
                self.init_signal = false
            },
            Phase::UpdatePhase => {
                self.logic_signal -= 1;
            },
            Phase::RenderPhase => {
                self.frame_signal = false;
            },
            Phase::ExternalEventsPhase => {},
            Phase::Idle => {}, 
        }
    }
}
