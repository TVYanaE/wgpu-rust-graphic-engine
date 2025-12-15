use std::{
    sync::{Arc, RwLock},
};
use winit::{
    event::WindowEvent,
};
use crate::{
    structures::{
        event_buffer_recorder::EventBufferRecorder,
        descriptors::{
            event_descriptor::EventDescriptor,
        },
        winit_data_buffer::WinitDataBuffer,
        winit_event_recorder::WinitEventRecorder,
    },
    enums::{
        events::{
            winit_event_enum::WinitEvent,
            internal_event_enum::InternalEvent,
        },
        component_name_enum::ComponentName,
    },
};

pub struct ExternalEventQuarts {
    event_buffer_recorder: Arc<RwLock<EventBufferRecorder>>,
    winit_event_recorder: Arc<RwLock<WinitEventRecorder>>,
    winit_data_buffer: Arc<RwLock<WinitDataBuffer>>,
}

impl ExternalEventQuarts {
    pub fn new(
        event_buffer_recorder: Arc<RwLock<EventBufferRecorder>>,
        winit_event_recorder: Arc<RwLock<WinitEventRecorder>>,
        winit_data_buffer: Arc<RwLock<WinitDataBuffer>>,
    ) -> Self {
        Self { 
            event_buffer_recorder: event_buffer_recorder,
            winit_event_recorder: winit_event_recorder,
            winit_data_buffer: winit_data_buffer,
        }
    } 
    pub fn run_tact(&mut self) {
        let mut winit_event_buffer = Vec::new();

        let mut winit_event_guard = self.winit_event_recorder.write().unwrap();

        for winit_event in winit_event_guard.drain_winit_event_buffer() {
            winit_event_buffer.push(winit_event);
        }
         
        drop(winit_event_guard);

        let mut winit_data_buffer_guard = self.winit_data_buffer.write().unwrap();
        let mut internal_event_buffer = Vec::new();

        for winit_event in winit_event_buffer {
            match winit_event {
                WinitEvent::WindowEvent(window_event) => {
                    match window_event {
                        WindowEvent::Resized(physical_size) => { 
                            winit_data_buffer_guard.set_physical(physical_size);
                            let internal_event_descriptor = EventDescriptor {
                                read_components: Vec::new(),
                                write_components: vec![ComponentName::Camera],
                            };

                            internal_event_buffer.push(InternalEvent::ResizedRequest(internal_event_descriptor));
                        },
                        _ => {},
                    }
                }
            }
        }

        drop(winit_data_buffer_guard);

        let mut event_recorder_guard = self.event_buffer_recorder.write().unwrap();

        for internal_event in internal_event_buffer {
            event_recorder_guard.register_internal_event(internal_event);
        }
    }
}
