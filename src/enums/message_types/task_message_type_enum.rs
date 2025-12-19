use crate::{
    structures::{
        task::Task,
    },
};

#[derive(Debug, Clone)]
pub enum TaskMessageType {
    Light,
    Heavy(Task), 
}
