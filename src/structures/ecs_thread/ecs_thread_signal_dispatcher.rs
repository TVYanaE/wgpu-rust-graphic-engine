use std::{
    rc::Rc,
    cell::RefCell,
};
use flume::{
    Receiver, TryRecvError,
};
use crate::{
    structures::{
        buses::{
            ecs_thread_data_bus::ECSThreadDataBus,
            ecs_thread_message_bus::ECSThreadMessageBus,
        },
    },
    enums::{
        signals::{
            ecs_thread_signal_enums::ECSThreadInputSignal,
        },
        ecs_thread_message_enums::{
            ECSThreadWorldManagerMessage,
        },
    },
};

pub struct ECSThreadSignalStorage {
    ecs_thread_input_channel_receiver: Receiver<ECSThreadInputSignal>,
    ecs_thread_data_bus: Rc<RefCell<ECSThreadDataBus>>,
    ecs_thread_message_bus: Rc<RefCell<ECSThreadMessageBus>>,
}

impl ECSThreadSignalStorage {
    pub fn new(
        ecs_thread_input_channel_receiver: Receiver<ECSThreadInputSignal>,
        ecs_thread_data_bus: Rc<RefCell<ECSThreadDataBus>>,
        ecs_thread_message_bus: Rc<RefCell<ECSThreadMessageBus>>,
    ) -> Self {
        Self { 
            ecs_thread_input_channel_receiver: ecs_thread_input_channel_receiver,
            ecs_thread_data_bus: ecs_thread_data_bus, 
            ecs_thread_message_bus: ecs_thread_message_bus, 
        }
    }

    pub fn start(&self) {
        loop {
            match self.ecs_thread_input_channel_receiver.try_recv() {
                Ok(ecs_thread_input_signal) => {
                    match ecs_thread_input_signal {
                        ECSThreadInputSignal::Init => {
                            self
                            .ecs_thread_message_bus
                            .borrow_mut()
                            .push_world_manager_message_to_bus(ECSThreadWorldManagerMessage::Init);
                        },
                        ECSThreadInputSignal::Shutdown => {
                            self
                            .ecs_thread_message_bus
                            .borrow_mut()
                            .push_world_manager_message_to_bus(ECSThreadWorldManagerMessage::Shutdown);
                        },
                        ECSThreadInputSignal::LogicCalculation => {
                            self
                            .ecs_thread_message_bus
                            .borrow_mut()
                            .push_world_manager_message_to_bus(ECSThreadWorldManagerMessage::LogicCalculation);
                        },
                        ECSThreadInputSignal::PrepareRenderState => {
                            self
                            .ecs_thread_message_bus
                            .borrow_mut()
                            .push_world_manager_message_to_bus(ECSThreadWorldManagerMessage::PrepareRenderState);
                        },
                        ECSThreadInputSignal::Resize => {
                            self
                            .ecs_thread_message_bus
                            .borrow_mut()
                            .push_world_manager_message_to_bus(ECSThreadWorldManagerMessage::Resize);
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
