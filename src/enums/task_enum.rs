use crate::{
    structures::{
        descriptors::{
            task_descriptor::TaskDescriptor,
        },
    },
    enums::{
        events::{
            internal_event_enum::InternalEvent,
        },
        task_name_enum::TaskName,
    },
};

#[derive(Debug, Clone)]
pub enum Task {
    RenderFrame(TaskDescriptor),
    PhysicsCalculation(TaskDescriptor),
    CameraReconfigurate(TaskDescriptor),
    Unknown,
}

impl Task {
    pub fn get_task_name(&self) -> Option<TaskName> {
        match self {
            Task::RenderFrame(task_descrptor) => { Some(task_descrptor.task_name) },
            Task::PhysicsCalculation(task_descriptor) => { Some(task_descriptor.task_name) },
            Task::CameraReconfigurate(task_descriptor) => { Some(task_descriptor.task_name) }
            Task::Unknown => {None}
        }
    }
    pub fn get_task_descriptor(&self) -> Option<&TaskDescriptor> {
        match self {
            Task::RenderFrame(task_descriptor) => { Some(task_descriptor) },
            Task::PhysicsCalculation(task_descriptor) => { Some(task_descriptor) },
            Task::CameraReconfigurate(task_decriptor) => { Some(task_decriptor) },
            Task::Unknown => { None },
        }
    }
}

impl From<InternalEvent> for Task {
    fn from(value: InternalEvent) -> Self {
        match value {
            InternalEvent::RequestRender(event_descriptor) => {
                let task_descriptor = TaskDescriptor {
                    task_name: TaskName::RenderFrame,
                    read_components: event_descriptor.read_components,
                    write_components: event_descriptor.write_components,
                };

                Self::RenderFrame(task_descriptor)
            },

            InternalEvent::RequestPhysicsCalculation(event_descriptor) => {
                let task_descriptor = TaskDescriptor {
                    task_name: TaskName::PhysicsCalculation,
                    read_components: event_descriptor.read_components,
                    write_components: event_descriptor.write_components,
                }; 

                Self::PhysicsCalculation(task_descriptor)
            },

            InternalEvent::ResizedRequest(event_descriptor) => {
                let task_descriptor = TaskDescriptor {
                    task_name: TaskName::CameraReconfigurate,
                    read_components: event_descriptor.read_components,
                    write_components: event_descriptor.write_components,
                };

                Self::CameraReconfigurate(task_descriptor) 
            }
        }
    }
}
