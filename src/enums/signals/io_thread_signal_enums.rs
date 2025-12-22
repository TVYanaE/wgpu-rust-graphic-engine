#![allow(dead_code)]

use crate::{
    enums::{
        winit_event_enum::WinitEvent,
    },
};

#[derive(Debug, Clone)]
pub enum IOThreadInputSignal {
    WinitEvent(WinitEvent),
}
