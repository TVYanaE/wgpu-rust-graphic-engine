use std::{
    rc::Rc,
    cell::RefCell,
};
use flume::{
    Receiver, Sender, TryRecvError,
};
use crate::{
    enums::{
        signals::{
            control_thread_signal_enums::ControlThreadInputSignal,
        },
        message_types::{
            event_message_type_enum::EventMessageType,
        },
        event_enum::Event,
    },
    structures::{
        buses::{
            control_thread_message_bus::ControlThreadMessagesBus,
            control_thread_data_bus::ControlThreadDataBus,
        },
        messages::{
            event_message::EventMessage,
        },
    },
};

pub struct ControlThreadRecorder {
    control_thread_input_channel_receiver: Receiver<ControlThreadInputSignal>,
    control_thread_message_bus_ref: Rc<RefCell<ControlThreadMessagesBus>>,
    control_thread_data_bus_ref: Rc<RefCell<ControlThreadDataBus>>,
}

impl ControlThreadRecorder {
    pub fn new(
        control_thread_input_channel_receiver: Receiver<ControlThreadInputSignal>,
        control_thread_message_bus_ref: Rc<RefCell<ControlThreadMessagesBus>>,
        control_thread_data_bus_ref: Rc<RefCell<ControlThreadDataBus>>,
    ) -> Self {
        Self { 
            control_thread_input_channel_receiver: control_thread_input_channel_receiver, 
            control_thread_message_bus_ref: control_thread_message_bus_ref,
            control_thread_data_bus_ref: control_thread_data_bus_ref,
        }
    }

    pub fn listen_input_channel(&mut self) -> Option<()> {
        let mut data_bus = self.control_thread_data_bus_ref.borrow_mut();
        let mut message_bus = self.control_thread_message_bus_ref.borrow_mut();
        loop {
            match self.control_thread_input_channel_receiver.try_recv() {
                Ok(control_thread_input_signal) => {
                    match control_thread_input_signal {
                        ControlThreadInputSignal::Event(event) => {
                            match event {
                                Event::Init => {
                                    let event_message = EventMessage {
                                        message_type: EventMessageType::Heavy(Event::Init),
                                        event_index: None,
                                    };

                                    message_bus.push_event_message_to_bus(event_message);
                                },
                                Event::Shutdown => {
                                    let event_message = EventMessage {
                                        message_type: EventMessageType::Heavy(Event::Shutdown),
                                        event_index: None,
                                    };

                                    message_bus.push_event_message_to_bus(event_message);
                                    return None;
                                },
                                Event::WinitEvent(winit) => {
                                    let event_index = data_bus.push_event_to_bus(Event::WinitEvent(winit));

                                    let event_message = EventMessage {
                                        message_type: EventMessageType::Light,
                                        event_index: Some(event_index),
                                    };

                                    message_bus.push_event_message_to_bus(event_message);
                                },
                                Event::LogicTick => {
                                    let event_message = EventMessage {
                                        message_type: EventMessageType::Heavy(Event::LogicTick),
                                        event_index: None,
                                    };

                                    message_bus.push_event_message_to_bus(event_message);
                                },
                                Event::FrameTick => {
                                    let event_message = EventMessage {
                                        message_type: EventMessageType::Heavy(Event::FrameTick),
                                        event_index: None,
                                    };

                                    message_bus.push_event_message_to_bus(event_message);
                                },
                            }
                        }
                    }
                },
                Err(try_recv_error) => {
                    match try_recv_error {
                        TryRecvError::Empty => { return Some(()); },
                        TryRecvError::Disconnected => { return None; },
                    }
                }
            }
            return Some(());
        } 
    }
}
