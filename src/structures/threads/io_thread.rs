use std::{
    thread::{
        self,
        JoinHandle,
    },
    sync::{Mutex, Arc},
};
use flume::{
    Receiver, 
};
use crate::{
    enums::{
        signals::{
            io_thread_signal_enums::IOThreadInputSignal,
        },
        event_enum::Event,
    },
    structures::{
        buses::{
            io_bus::IOBus,
        }, 
    },
};

pub struct IOThread {
    pub handle: JoinHandle<()>
}

impl IOThread {
    pub fn start_thread(
        io_thread_input_channel_receiver: Receiver<IOThreadInputSignal>,
        io_bus: Arc<Mutex<IOBus>>,
    ) -> Self {
        let handle = thread::spawn(move ||{

        let io_bus = io_bus;

            loop {
                match io_thread_input_channel_receiver.recv() {
                    Ok(io_thread_input_signal) => {
                        match io_thread_input_signal {
                            IOThreadInputSignal::WinitEvent(winit_event) => {
                                let event = Event::WinitEvent(winit_event);

                                io_bus.lock().unwrap().push(event);

                            }
                        }
                    },
                    Err(_) => { break; }
                }    
            }
        });
        Self { handle }
    } 
}
