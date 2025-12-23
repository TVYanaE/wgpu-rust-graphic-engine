mod traits;
mod enums;
mod structures;
mod functions;
mod aliases;
mod consts;

use std::{
    sync::{Arc, Mutex},
};
use rayon::{ThreadPoolBuilder};
use flume::{unbounded, Sender,};
use winit::{
    application::ApplicationHandler,
    event::{WindowEvent, StartCause, },
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};
use crate::{
    structures::{
        buses::{
            io_bus::IOBus,
        },
        states::{
            app_state::AppState,
            static_shared_thread_state::StaticSharedThreadState,
            dynamic_shared_thread_state::DynamicSharedThreadState,
        },
        timer::Timer, 
        recorders::{
            winit_event_recorder::WinitEventRecorder,
        },
        threads::{
            render_thread::RenderThread,
            control_thread::ControlThread,
            io_thread::IOThread,
        },
    },
    enums::{
        signals::{
            control_thread_signal_enums::ControlThreadInputSignal,
            io_thread_signal_enums::IOThreadInputSignal,
        },
    },  
};

#[derive(Default)]
struct App {
    app_state: Option<AppState>,
    // Threads 
    render_thread: Option<RenderThread>,
    control_thread: Option<ControlThread>,
    io_thread: Option<IOThread>,

    // Shared Thread States 
    static_shared_thread_state: Option<Arc<StaticSharedThreadState>>,
    dynamic_shared_thread_state: Option<Arc<Mutex<DynamicSharedThreadState>>>,

    // Buses 
    io_bus: Option<Arc<Mutex<IOBus>>>,

    // timer
    timer: Option<Timer>,

    // recorder of winit events
    winit_event_recorder: Option<WinitEventRecorder>,

    // channel for ControlThread
    control_thread_input_channel_sender: Option<Sender<ControlThreadInputSignal>> 
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

        // init rayon thread pool for Shipyard multithread 
        let logical_cpu_number = num_cpus::get();

        // Reserved for OS and GPU Thread 
        let reserved_thread_number = 2;

        let rayon_threads_number = logical_cpu_number.saturating_sub(reserved_thread_number).max(1);

        ThreadPoolBuilder::new()
            .num_threads(rayon_threads_number)
            .build_global()
            .expect("Rayon thread pool init error");

        // Init channels  

        let (
            control_thread_input_channel_sender,
            control_thread_input_channel_receiver
        ) = unbounded::<ControlThreadInputSignal>();

        let (
            io_thread_input_channel_sender,
            io_thread_input_channel_receiver,
        ) = unbounded::<IOThreadInputSignal>();

        // Init exchange thread buffers and buses 
        let io_bus = Arc::new(Mutex::new(IOBus::new()));

        // Init winit event recorder  
        let winit_event_recorder = WinitEventRecorder::new(io_thread_input_channel_sender.clone()); 

        // Init Timer 
        let timer = Timer::new(control_thread_input_channel_sender.clone()); 

        // Init dynamic shader thread state 
        let dynamic_shared_thread_state = Arc::new(
            Mutex::new(DynamicSharedThreadState::new())
        );

        // Init threads
        let render_thead = pollster::block_on(RenderThread::new(window)); 
        let io_thread = IOThread::start_thread(io_thread_input_channel_receiver, io_bus.clone()); 
        let control_thread = ControlThread::start_thread(
            control_thread_input_channel_receiver,
            io_bus.clone(),
            dynamic_shared_thread_state.clone(),
        );

        // Init static Shader Thread state 
        let static_shared_thread_state = StaticSharedThreadState::new(render_thead.get_material_manager());

        // Save into App
        self.render_thread = Some(render_thead);
        self.control_thread = Some(control_thread);
        self.io_thread = Some(io_thread);

        self.static_shared_thread_state = Some(Arc::new(static_shared_thread_state));
        self.dynamic_shared_thread_state = Some(dynamic_shared_thread_state); 

        self.io_bus = Some(io_bus);
        
        self.winit_event_recorder = Some(winit_event_recorder);
        self.timer = Some(timer); 
        self.control_thread_input_channel_sender = Some(control_thread_input_channel_sender);

        self.control_thread_input_channel_sender.as_ref().unwrap().send(ControlThreadInputSignal::Init);
         
    }

    /* fn suspended(&mut self, event_loop: &ActiveEventLoop) {
        
    }

    fn exiting(&mut self, event_loop: &ActiveEventLoop) {
        
    } */

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {  
        match event {
            WindowEvent::CloseRequested => {
                 
                self.control_thread_input_channel_sender
                    .as_ref()
                    .unwrap()
                    .send(ControlThreadInputSignal::Shutdown);

                if let Some(control_thread) = self.control_thread.take() {
                    control_thread.handle.join();
                }
                else {
                    return;
                } 
                
                // TODO!: for another threads 
                
                event_loop.exit();
            },
            _ => {
                self.winit_event_recorder.as_mut().unwrap().register_input_event(event);
            }
        }

         
    }

    fn new_events(&mut self, _event_loop: &ActiveEventLoop, cause: StartCause) {
        match cause { 
            StartCause::Poll => {
                let timer = self.timer.as_mut().unwrap();
                timer.update();
                timer.check_logic_threshold();
                timer.check_frame_threshold();

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
