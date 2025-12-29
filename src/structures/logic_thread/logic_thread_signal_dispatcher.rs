use std::{
    rc::Rc,
    cell::RefCell,
};
use flume::{
    Receiver, TryRecvError,
};
use crate::{
    structures::{
        logic_thread::{
            buses::{
                logic_thread_data_bus::LogicThreadDataBus,
                logic_thread_message_bus::LogicThreadMessageBus,
            },
        },
    },
    enums::{
        signals::{
            logic_thread_signal_enums::LogicThreadInputSignal,
        },
        
    },
};

pub struct LogicThreadSignalDispatcher {
    logic_thread_input_channel_receiver: Receiver<LogicThreadInputSignal>,
    logic_thread_data_bus: Rc<RefCell<LogicThreadDataBus>>,
    logic_thread_message_bus: Rc<RefCell<LogicThreadMessageBus>>,
}

impl LogicThreadSignalDispatcher {
    pub fn new(
        logic_thread_input_channel_receiver: Receiver<LogicThreadInputSignal>,
        logic_thread_data_bus: Rc<RefCell<LogicThreadDataBus>>,
        logic_thread_message_bus: Rc<RefCell<LogicThreadMessageBus>>,
    ) -> Self {
        Self { 
            logic_thread_input_channel_receiver: logic_thread_input_channel_receiver,
            logic_thread_data_bus: logic_thread_data_bus, 
            logic_thread_message_bus: logic_thread_message_bus, 
        }
    }

    pub fn start(&self) {
        loop {
            match self.logic_thread_input_channel_receiver.try_recv() {
                Ok(logic_thread_input_signal) => {
                    match logic_thread_input_signal {
                        LogicThreadInputSignal::Init => {
                            
                        },
                        LogicThreadInputSignal::Shutdown => {

                        },
                        LogicThreadInputSignal::LogicCalculation => {
                             
                        },
                        LogicThreadInputSignal::PrepareRenderState => {
                            
                        },
                        LogicThreadInputSignal::Resize => {
                            
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
                        },
                    }
                }
            }
        } 
    }
}
