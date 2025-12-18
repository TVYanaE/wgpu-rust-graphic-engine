
use crate::{
    enums::{
        event_enum::Event, 
    },
};

pub enum ControlThreadInputSignal {
    EventBuffer(Vec<Event>),
    LogicTick,
    FrameTick,
}
