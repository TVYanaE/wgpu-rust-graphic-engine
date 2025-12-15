use crate::{
    structures::{
        descriptors::event_descriptor::EventDescriptor,
    },
};

#[derive(Debug, Clone,)]
pub enum InternalEvent {
    RequestRender(EventDescriptor),
    RequestPhysicsCalculation(EventDescriptor),
    ResizedRequest(EventDescriptor),
}
