use crate::{
    structures::{
        task_chunk::TaskChunk,
    },
};

pub struct ExecuteurThreadDataBus {
    task_chunks: Vec<TaskChunk>,
    job_list: Vec<TaskChunk>,
}

impl ExecuteurThreadDataBus {
    pub fn new() -> Self {

        Self { 
            task_chunks: Vec::new(),
            job_list: Vec::new(),
        }
    }
    
    pub fn push_task_chunk(&mut self, task_chunk: TaskChunk) {
        self.task_chunks.push(task_chunk);
    }
    
    pub fn push_task_chunks(&mut self, task_chunks: impl Iterator<Item = TaskChunk>) {
        self.task_chunks.extend(task_chunks);
    }

    pub fn push_job_list(&mut self, job_list: impl Iterator<Item = TaskChunk>) {
        self.job_list.extend(job_list);
    }

    pub fn drain_task_chunk_buffer(&mut self) -> impl Iterator<Item = TaskChunk> {
        self.task_chunks.drain(..)
    }

    pub fn drain_job_list(&mut self) -> impl Iterator<Item = TaskChunk> {
        self.job_list.drain(..)
    } 
}
