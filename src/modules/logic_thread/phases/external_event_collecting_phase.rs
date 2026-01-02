use std::{
    collections::{VecDeque},
    sync::{Arc},
};
use winit::{
    event::{
        WindowEvent,
    },
};
use super::super::{
    events::external_event::ExternalEvent, 
};
use crate::{
    modules::{
        shared::{
            double_buffer_bus::DoubleBufferBus,
        },
        main_thread::{
            winit_event::WinitEvent,
        },
    },
};


pub fn external_event_collecting_phase(
    winit_event_bus: Arc<DoubleBufferBus<WinitEvent>>,
    external_event_queue: &mut VecDeque<ExternalEvent>,
) {
    let winit_events = winit_event_bus.get_read_buffer();

    for winit_event in winit_events.iter() {
        match winit_event {
            WinitEvent::WindowEvent(window_event) => {
                match window_event { 
                    WindowEvent::KeyboardInput { 
                        event,
                        ..
                    } => {
                        let external_event = ExternalEvent::KeyEvent((*event).clone());

                        external_event_queue.push_back(external_event);
                    },
                    _ => {}
                }
            },
        }
    }
} 
