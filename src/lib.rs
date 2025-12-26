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
    enums::{
        signals::{
            control_thread_signal_enums::ControlThreadInputSignal, 
            ecs_thread_signal_enums::ECSThreadInputSignal,
            executeur_thread_signal_enums::ExecuteurThreadInputSignal,
            io_thread_signal_enums::IOThreadInputSignal,
            render_thread_signal_enums::RenderThreadInputSignal,
        },
    },
    structures::{
        buses::{
            io_bus::IOBus
        }, 
        recorders::{
            winit_event_recorder::WinitEventRecorder,
        }, 
        states::{
            dynamic_shared_thread_state::DynamicSharedThreadState, 
            static_shared_thread_state::StaticSharedThreadState
        }, 
        threads::{
            control_thread::ControlThread, 
            executeur_thread::ExecuteurThread, 
            io_thread::IOThread, 
            render_thread::RenderThread,
            ecs_thread::ECSThread,
        }, 
        time_menu::{
            TimeMenu
        }, 
        timer::Timer
    }  
};

#[derive(Default)]
struct App {
    // Threads 
    render_thread: Option<RenderThread>,
    control_thread: Option<ControlThread>,
    io_thread: Option<IOThread>,
    executeur_thread: Option<ExecuteurThread>,
    ecs_thread: Option<ECSThread>,

    // Shared Thread States 
    static_shared_thread_state: Option<Arc<StaticSharedThreadState>>,
    dynamic_shared_thread_state: Option<Arc<Mutex<DynamicSharedThreadState>>>,
    time_menu: Option<Arc<Mutex<TimeMenu>>>,

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

        let (
            executeur_thread_input_channel_sender,
            executeur_thread_input_channel_receiver
        ) = unbounded::<ExecuteurThreadInputSignal>();

        let (
            ecs_thread_input_channel_sender,
            ecs_thread_input_channel_receiver
        ) = unbounded::<ECSThreadInputSignal>();
       
        let (
            render_thread_input_channel_sender,
            render_thread_input_channel_receiver
        ) = unbounded::<RenderThreadInputSignal>();

        // Init exchange thread buffers and buses 
        let io_bus = Arc::new(Mutex::new(IOBus::new()));

        // Init winit event recorder  
        let winit_event_recorder = WinitEventRecorder::new(io_thread_input_channel_sender.clone()); 

        // Init Timer 
        let timer = Timer::new(
            control_thread_input_channel_sender.clone(),
            executeur_thread_input_channel_sender.clone(),
        ); 

        // Init dynamic shader thread state 
        let dynamic_shared_thread_state = Arc::new(
            Mutex::new(DynamicSharedThreadState::new())
        );

        // Init time menu 
        let time_menu = Arc::new(Mutex::new(TimeMenu::new()));

        // Init threads
        let render_thread = pollster::block_on(RenderThread::new(window)); 
        let io_thread = IOThread::start_thread(io_thread_input_channel_receiver, io_bus.clone()); 
        let control_thread = ControlThread::start_thread(
            control_thread_input_channel_receiver,
            io_bus.clone(),
            dynamic_shared_thread_state.clone(),
            executeur_thread_input_channel_sender,
            time_menu.clone(),
        );
        let executeur_thread = ExecuteurThread::start_thread(
            executeur_thread_input_channel_receiver, 
            dynamic_shared_thread_state.clone(),
            ecs_thread_input_channel_sender.clone(),
            render_thread_input_channel_sender,
        );
        let ecs_thread = ECSThread::start_thread(ecs_thread_input_channel_receiver);

        // Init static Shader Thread state 
        let static_shared_thread_state = StaticSharedThreadState::new(render_thread.get_material_manager());

        // Save into App
        self.render_thread = Some(render_thread);
        self.control_thread = Some(control_thread);
        self.io_thread = Some(io_thread);
        self.executeur_thread = Some(executeur_thread);
        self.ecs_thread = Some(ecs_thread);

        self.static_shared_thread_state = Some(Arc::new(static_shared_thread_state));
        self.dynamic_shared_thread_state = Some(dynamic_shared_thread_state); 
        self.time_menu = Some(time_menu);

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
