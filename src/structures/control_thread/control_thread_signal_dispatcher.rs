use std::{
    rc::Rc,
    cell::RefCell,
};
use flume::{
    Receiver, TryRecvError
};
use crate::{
    structures::{
        control_thread::{
            buses::{
                control_thread_message_bus::ControlThreadMessagesBus,
            },
        },
    },
    enums::{
        signals::{
            control_thread_signal_enums::{
                ControlThreadInputSignal,
                ControlThreadInputEngineSignal,
                ControlThreadInputGameSignal,
            },
        },
        messages::{
            control_thread_message_enums::{
                ControlThreadRequestManagerMessage,
                ControlThreadSceneManagerMessage,
            },
        },
    },
};

pub struct ControlThreadSignalDispatcher {
    control_thread_input_channel_receiver: Receiver<ControlThreadInputSignal>,
    control_thread_message_bus: Rc<RefCell<ControlThreadMessagesBus>>,
}

impl ControlThreadSignalDispatcher {
    pub fn new(
        control_thread_input_channel_receiver: Receiver<ControlThreadInputSignal>,
        control_thread_message_bus: Rc<RefCell<ControlThreadMessagesBus>>,
    ) -> Self {
        Self {
            control_thread_input_channel_receiver: control_thread_input_channel_receiver, 
            control_thread_message_bus: control_thread_message_bus, 
        }
    }

    pub fn start(&self) {
        loop {
            match self.control_thread_input_channel_receiver.try_recv() {
                Ok(control_thread_input_signal) => {
                    match control_thread_input_signal {
                        ControlThreadInputSignal::EngineSignal(engine_signal) => {
                            match engine_signal {
                                ControlThreadInputEngineSignal::Shutdown => {
                                    self
                                    .control_thread_message_bus
                                    .borrow_mut()
                                    .push_request_manager_message(ControlThreadRequestManagerMessage::ShutdownSignal);
                                },
                                ControlThreadInputEngineSignal::Init => {
                                    self
                                    .control_thread_message_bus
                                    .borrow_mut()
                                    .push_request_manager_message(ControlThreadRequestManagerMessage::InitSignal);
                                },
                                ControlThreadInputEngineSignal::LogicStart => {
                                    self
                                    .control_thread_message_bus
                                    .borrow_mut()
                                    .push_request_manager_message(ControlThreadRequestManagerMessage::LogicStart);
                                },
                                ControlThreadInputEngineSignal::FrameStart => {
                                    self
                                    .control_thread_message_bus
                                    .borrow_mut()
                                    .push_request_manager_message(ControlThreadRequestManagerMessage::FrameStart);
                                },     
                            }
                        },
                        ControlThreadInputSignal::GameSignal(game_signal) => {
                            match game_signal {
                                ControlThreadInputGameSignal::GameplayStarted => {
                                    self
                                    .control_thread_message_bus
                                    .borrow_mut()
                                    .push_scene_manager_message(ControlThreadSceneManagerMessage::GameplayStarted);
                                },
                            }
                        }
                    } 
                },
                Err(try_recv_error) => {
                    match try_recv_error {
                        TryRecvError::Empty => { break; },
                        TryRecvError::Disconnected => {
                            break;
                        }
                    }
                },
            }
        } 
    }
}
