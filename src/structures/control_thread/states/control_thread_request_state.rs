
use crate::{
    enums::{
        control_thread_request_enum::ControlThreadRequest, 
    },
};

pub struct ControlThreadRequestState {
    priority_request: Option<ControlThreadRequest>,
}

impl ControlThreadRequestState {
    pub fn new() -> Self {
        Self { priority_request: None }
    }

    pub fn set_priority_request(&mut self, request: Option<ControlThreadRequest>) {
        self.priority_request = request;
    }

    pub fn get_priority_request(&self) -> Option<ControlThreadRequest> {
        self.priority_request
    }
}
