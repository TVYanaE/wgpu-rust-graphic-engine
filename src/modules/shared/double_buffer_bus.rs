use std::{
    mem::{take},
    sync::{Arc, Mutex},
};

pub type Snapshot<T> = Arc<Vec<T>>;

pub struct DoubleBufferBus<T> {
    write_buffer: Mutex<Vec<T>>,
    read_buffer: Mutex<Snapshot<T>>,
}

impl<T> DoubleBufferBus<T> {
    pub fn new() -> Self {
        Self {
            write_buffer: Mutex::new(Vec::new()),
            read_buffer: Mutex::new(Arc::new(Vec::new())), 
        }
    }

    pub fn push(&self, value: T) {
        self.write_buffer.lock().unwrap().push(value);
    }

    pub fn swap(&self) {
        let mut write_buffer = self.write_buffer.lock().unwrap();
        let mut read_buffer = self.read_buffer.lock().unwrap();
        
        let snapshot = Arc::new(take(&mut *write_buffer));

        *read_buffer = snapshot;
    }

    pub fn get_read_buffer(&self) -> Arc<Vec<T>> {
        self.read_buffer.lock().unwrap().clone()
    }
}
