
use crate::{ 
    enums::{
        messages::{
            control_thread_message_enums::{
                ControlThreadRequestManagerMessage,
                ControlThreadSceneManagerMessage,
            },
        },
    },    
};

pub struct ControlThreadMessagesBus { 
    request_manager_messages: Vec<ControlThreadRequestManagerMessage>,
    scene_manager_messages: Vec<ControlThreadSceneManagerMessage>,
}

impl ControlThreadMessagesBus {
    pub fn new() -> Self {
        Self { 
            request_manager_messages: Vec::new(),
            scene_manager_messages: Vec::new(),
        }
    } 

    pub fn push_request_manager_message(
        &mut self, 
        message: ControlThreadRequestManagerMessage
    ) {
        self.request_manager_messages.push(message);
    }

    pub fn push_scene_manager_message(&mut self, message: ControlThreadSceneManagerMessage) {
        self.scene_manager_messages.push(message);
    }

    pub fn drain_request_manager_message_buffer(
        &mut self
    ) -> impl Iterator<Item = ControlThreadRequestManagerMessage> {
        self.request_manager_messages.drain(..)
    }

    pub fn drain_scene_manager_message_buffer(
        &mut self
    ) -> impl Iterator<Item = ControlThreadSceneManagerMessage> {
        self.scene_manager_messages.drain(..)
    }
}
