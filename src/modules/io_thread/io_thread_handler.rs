use std::{
    thread::{
        self,
        JoinHandle,
    },
};


pub struct IOThread {
    pub handle: JoinHandle<()>
}

impl IOThread {
    pub fn start_thread( 
    ) -> Self {
        let handle = thread::spawn(move ||{

        
        });
        Self { handle }
    } 
}
