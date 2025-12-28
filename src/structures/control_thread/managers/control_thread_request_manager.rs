use std::{
    rc::Rc,
    cell::RefCell,
};
use crate::{
    structures::{
        control_thread::{
            buses::{
                control_thread_message_bus::ControlThreadMessagesBus,
            },
            states::{
                control_thread_request_state::ControlThreadRequestState,
            },
        },
    },
    enums::{ 
        messages::{
            control_thread_message_enums::{
                ControlThreadRequestManagerMessage,
            },
        },
        control_thread_request_enum::ControlThreadRequest,
    },
};

pub struct ControlThreadRequestManager {
    control_thread_message_bus: Rc<RefCell<ControlThreadMessagesBus>>,
    control_thread_request_state: Rc<RefCell<ControlThreadRequestState>>,
    logic_signal: u16,
    frame_signal: bool,
    init_signal: bool,
    shutdown_signal: bool,
}

impl ControlThreadRequestManager {
    pub fn new(
        control_thread_message_bus: Rc<RefCell<ControlThreadMessagesBus>>,
        control_thread_request_state: Rc<RefCell<ControlThreadRequestState>>,
    ) -> Self {
        Self {
            control_thread_message_bus: control_thread_message_bus,
            control_thread_request_state: control_thread_request_state,
            logic_signal: 0, 
            frame_signal: false, 
            init_signal: false, 
            shutdown_signal: false, 
        }
    }

    pub fn start(&mut self)  {
        for message in self
            .control_thread_message_bus
            .borrow_mut()
            .drain_request_manager_message_buffer() {
            match message {
                ControlThreadRequestManagerMessage::InitSignal => {
                    self.init_signal = true; 
                },
                ControlThreadRequestManagerMessage::ShutdownSignal => {
                    self.shutdown_signal = true;
                },
                ControlThreadRequestManagerMessage::LogicStart => {
                    self.logic_signal += 1;
                },
                ControlThreadRequestManagerMessage::FrameStart => {
                    self.frame_signal = true;
                },
            }
        }

        if self.shutdown_signal == true {
            self
            .control_thread_request_state
            .borrow_mut()
            .set_priority_request(Some(ControlThreadRequest::ShutdownRequest)); 

            self.shutdown_signal = false;
        } 
        else if self.init_signal == true {
            self
            .control_thread_request_state
            .borrow_mut()
            .set_priority_request(Some(ControlThreadRequest::InitRequest));

            self.init_signal = false;
        }
        else if self.frame_signal == true {
            self
            .control_thread_request_state
            .borrow_mut()
            .set_priority_request(Some(ControlThreadRequest::RenderRequest));

            self.frame_signal = false;
        }
        else if self.logic_signal > 0 {
            self
            .control_thread_request_state
            .borrow_mut()
            .set_priority_request(Some(ControlThreadRequest::LogicCalculationRequest)); 

            self.logic_signal -= 1;
        }
        else {
            self
            .control_thread_request_state
            .borrow_mut()
            .set_priority_request(None);
        }
    }  
}
