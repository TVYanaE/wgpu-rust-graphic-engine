
use crate::{
    enums::{
        message_types::{
            task_message_type_enum::TaskMessageType,
        },
    },
};

#[derive(Debug, Clone)]
pub struct TaskMessage {
    message_type: TaskMessageType,
    task_index: Option<usize>,
} 
