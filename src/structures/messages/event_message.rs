use crate::{
    enums::{
        message_types::{
            event_message_type_enum::EventMessageType,
        },
    },
};

#[derive(Debug, Clone)]
pub struct EventMessage {
    pub message_type: EventMessageType,
    pub event_index: Option<usize>,
}
