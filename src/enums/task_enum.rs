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
}

impl Task {
    pub fn get_task_name(&self) -> TaskName {
        match self {
            Task::RenderFrame(task_descrptor) => { task_descrptor.task_name },
            Task::PhysicsCalculation(task_descriptor) => { task_descriptor.task_name }
        }
    }
}

impl From<InternalEvent> for Task {
    fn from(value: InternalEvent) -> Self {
        match value {
            InternalEvent::RequestRender(event_descriptor) => {
                let task_descriptor = TaskDescriptor {
                    task_name: TaskName::RenderFrame,
                    frame_phase: event_descriptor.frame_phase,
                    read_components: event_descriptor.read_components,
                    write_components: event_descriptor.write_components,
                };

                Self::RenderFrame(task_descriptor)
            },

            InternalEvent::RequestPhysicsCalculation(event_descriptor) => {
                let task_descriptor = TaskDescriptor {
                    task_name: TaskName::PhysicsCalculation,
                    frame_phase: event_descriptor.frame_phase,
                    read_components: event_descriptor.read_components,
                    write_components: event_descriptor.write_components,
                }; 

                Self::PhysicsCalculation(task_descriptor)
            },
        }
    }
}
