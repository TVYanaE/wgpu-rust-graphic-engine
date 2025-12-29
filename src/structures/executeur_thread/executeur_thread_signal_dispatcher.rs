use std::{
    cell::RefCell,
    rc::Rc,
};
use flume::{
    Receiver, TryRecvError,
};
use crate::{
    structures::{
        executeur_thread::{
            buses::{
                executeur_thread_message_bus::ExecuteurThreadMessageBus,
            },
        },
    },
    enums::{
        signals::{
            executeur_thread_signal_enums::ExecuteurThreadInputSignal,
        },
        execute_thread_message_enums::{
            ExecuteurThreadTaskControllerMessage,
            ExecuteurThreadTimeManagerMessage,
        },
    },
    
};


pub struct ExecuteurThreadSignalDispatcher {
    executeur_thread_input_channel_receiver: Receiver<ExecuteurThreadInputSignal>,
    executeur_thread_message_bus: Rc<RefCell<ExecuteurThreadMessageBus>>,
}

impl ExecuteurThreadSignalDispatcher {
    pub fn new(
        executeur_thread_input_channel_receiver: Receiver<ExecuteurThreadInputSignal>,
        executeur_thread_message_bus: Rc<RefCell<ExecuteurThreadMessageBus>>,
    ) -> Self {

        Self { 
            executeur_thread_input_channel_receiver: executeur_thread_input_channel_receiver,
            executeur_thread_message_bus: executeur_thread_message_bus,
        }
    }

    pub fn start(&self) {
        loop {
            match self.executeur_thread_input_channel_receiver.try_recv() {
                Ok(executeur_thread_input_signal) => {
                    match executeur_thread_input_signal {
                        ExecuteurThreadInputSignal::ScheduleReady => {
                            self
                            .executeur_thread_message_bus
                            .borrow_mut()
                            .push_task_controller_message_to_bus(ExecuteurThreadTaskControllerMessage::ScheduleReady);
                        },
                        ExecuteurThreadInputSignal::LogicStart => {
                            self
                            .executeur_thread_message_bus
                            .borrow_mut()
                            .push_time_manager_message_to_bus(ExecuteurThreadTimeManagerMessage::LogicStart);
                        },
                        ExecuteurThreadInputSignal::FrameStart => {
                            self
                            .executeur_thread_message_bus
                            .borrow_mut()
                            .push_time_manager_message_to_bus(ExecuteurThreadTimeManagerMessage::LogicStart);
                        },
                    }
                },
                Err(try_recv_error) => {
                    match try_recv_error {
                        TryRecvError::Empty => {
                            break;
                        },
                        TryRecvError::Disconnected => {
                            break;
                        }
                    }
                },
            } 
        }
    } 
}
