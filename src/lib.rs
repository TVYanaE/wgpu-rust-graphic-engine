mod traits;
mod enums;
mod structures;
mod functions;
mod aliases;
mod consts;

use std::{
    sync::{Arc},
};
use flume::{unbounded, Sender,};
use winit::{
    application::ApplicationHandler,
    event::{WindowEvent, StartCause, },
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};
use crate::{
    structures::{
        states::{
            app_state::AppState,
            shared_thread_state::SharedThreadState,
        },
        timer::Timer, 
        thread_buffer_recorders::{
            input_event_thread_buffer_recorder::InputEventThreadBufferRecorder,
        },
        threads::{
            render_thread::RenderThread,
            control_thread::ControlThread,
        },
    },
    enums::{
        input_event_enum::InputEvent, 
    },
};

#[derive(Default)]
struct App {
    app_state: Option<AppState>,
    // Threads 
    render_thread: Option<RenderThread>,
    control_thread: Option<ControlThread>,

    // Shared Thread State 
    shared_thread_state: Option<Arc<SharedThreadState>>,
    //
    timer: Option<Timer>,
    input_event_thread_buffer_recorder: Option<InputEventThreadBufferRecorder>,
    input_event_channel_sender: Option<Sender<InputEvent>>,
}

impl ApplicationHandler for App {

    // This event is triggered after StartCase::Init 
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // Get the window from winit
        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
            );   

        // Init channels 
        let (input_event_channel_sender, input_event_channel_receiver) = unbounded::<InputEvent>();

        // Init exchange thread buffers

        // Init exchande thread buffers recorders
        let input_event_thread_buffer_recorder = 
            InputEventThreadBufferRecorder::new(input_event_channel_sender.clone());

        // Init Timer 
        let timer = Timer::new(input_event_channel_sender.clone()); 

        // Init threads
        let render_thead = pollster::block_on(RenderThread::new(window));
        let control_thread = ControlThread::start_thread(input_event_channel_receiver);

        // Init Shader Thread state 
        let shared_thread_state = SharedThreadState::new(render_thead.get_material_manager());

        // Save into App
        self.render_thread = Some(render_thead);
        self.control_thread = Some(control_thread);

        self.shared_thread_state = Some(Arc::new(shared_thread_state));

        self.input_event_thread_buffer_recorder = Some(input_event_thread_buffer_recorder);

        self.input_event_channel_sender = Some(input_event_channel_sender);
        self.timer = Some(timer);  
        
        
    }

    /* fn suspended(&mut self, event_loop: &ActiveEventLoop) {
        
    }

    fn exiting(&mut self, event_loop: &ActiveEventLoop) {
        
    } */

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {  
        match event {
            WindowEvent::CloseRequested => {
                if let Some(input_event_channel_sender) = self.input_event_channel_sender.as_ref() {
                    input_event_channel_sender.send(InputEvent::Shutdown);
                }
                else {
                    return;
                }

                if let Some(control_thread) = self.control_thread.take() {
                    control_thread.handle.join();
                }
                else {
                    return;
                }
                
                event_loop.exit();
            },
            _ => {
                self.input_event_thread_buffer_recorder.as_mut().unwrap().register_input_event(event);
            }
        }

         
    }

    fn new_events(&mut self, _event_loop: &ActiveEventLoop, cause: StartCause) {
        match cause { 
            StartCause::Poll => {
                let timer =self.timer.as_mut().unwrap();
                timer.check_step_fixed(); 
            },
            // There are two another type of StartCause for another type of ControlFlow
            _ => {},    
        }
    } 
}


pub fn start_app() {
    
    env_logger::init();

    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Poll);
 
    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}
