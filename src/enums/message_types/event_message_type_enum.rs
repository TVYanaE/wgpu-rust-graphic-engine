use crate::{
    enums::{
        event_enum::Event,
    },
};

#[derive(Debug, Clone)]
pub enum EventMessageType {
    Light,
    Heavy(Event),
}
