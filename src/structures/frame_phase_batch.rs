use crate::{
    enums::{
        task_enum::Task,
        component_name_enum::ComponentName,
    },
};

pub struct FramePhaseBatch {
    tasks: Vec<Task>,
    component_write: Vec<ComponentName>
}

impl FramePhaseBatch {
    pub fn new() -> Self {
        Self { 
            tasks: Vec::with_capacity(4),
            component_write: Vec::with_capacity(2)
        }
    }
   
    // if true - there is component that will be writing 
    pub fn has_write_component(&self, write_component: &ComponentName) -> bool {
        self.component_write.contains(write_component)
    }

    pub fn conflict_with_write_components(&self, write_components: &[ComponentName]) -> bool {
        let mut logical_buffer = false;

        for write_component in write_components {
            let contain_component = self.component_write.contains(write_component);
            logical_buffer = logical_buffer || contain_component;
        }

        return logical_buffer;
    }

    pub fn add_task(&mut self, task: Task) {
        let task_descriptor = task.get_task_descriptor().unwrap().clone();

        for write_component in task_descriptor.write_components {
            self.component_write.push(write_component);
        }

        self.tasks.push(task);
    }

    pub fn drain_batch(&mut self) -> impl Iterator<Item = Task> {
        self.component_write.clear();
        self.tasks.drain(..)        
    }
}
