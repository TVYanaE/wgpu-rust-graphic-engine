mod aliases;
mod consts;
mod modules;

use std::{
    sync::{Arc},
};
use rayon::{ThreadPoolBuilder};
use winit::{
    application::ApplicationHandler,
    event::{WindowEvent, StartCause, },
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};
use flume::{
    Sender, unbounded,
}; 
use crate::{ 
    modules::{
        main_thread::{ 
            timer::Timer,
            winit_event::WinitEvent,
        },
        logic_thread::{
            logic_thread_handler::LogicThreadHandler,
            signals::LogicThreadInputSignal,
        },
        render_thread::{
            render_thread_handler::RenderThreadHandler,
            signals::RenderThreadInputSignal,
        },
        shared::{
            double_buffer_bus::DoubleBufferBus,
            render_state::RenderState,
        },
    },  
};

#[derive(Default)]
struct App {
    // Thread Handlers 
    render_thread_handler: Option<RenderThreadHandler>,
    logic_thread_handler: Option<LogicThreadHandler>,
   
    // Buses 
    winit_event_bus: Option<Arc<DoubleBufferBus<WinitEvent>>>,
    render_state_bus: Option<Arc<DoubleBufferBus<RenderState>>>,

    // Thread Channel Senders
    render_thread_input_channel_sender: Option<Sender<RenderThreadInputSignal>>,
    logic_thread_input_channel_sender: Option<Sender<LogicThreadInputSignal>>,

    // Timer
    timer: Option<Timer>,
 
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
 
        // Init rayon thread pool for Shipyard multithread 
        let logical_cpu_number = num_cpus::get();

        // Reserved for OS and GPU Thread 
        let reserved_thread_number = 2;

        let rayon_threads_number = logical_cpu_number.saturating_sub(reserved_thread_number).max(1);

        ThreadPoolBuilder::new()
            .num_threads(rayon_threads_number)
            .build_global()
            .expect("Rayon thread pool init error"); 

        // Init Buses
        let winit_event_bus = Arc::new(DoubleBufferBus::<WinitEvent>::new());
        let render_state_bus = Arc::new(DoubleBufferBus::<RenderState>::new());

        // Init Thread Channels 
        // Render Thread Channel
        let (
            render_thread_input_channel_sender,
            render_thread_input_channel_receiver
        ) = unbounded::<RenderThreadInputSignal>();

        // Logic Thread Channel
        let (
            logic_thread_input_channel_sender,
            logic_thread_input_channel_receiver
        ) = unbounded::<LogicThreadInputSignal>();

        // Init Timer 
        let timer = Timer::new();  
      
        // Init Threads 
        // Render Thread
        let render_thread_handler = pollster::block_on(RenderThreadHandler::start_thread(
            window, 
            render_thread_input_channel_receiver,
            winit_event_bus.clone(),
            render_state_bus.clone()
        ));

        // Logic Thread 
        let logic_thread_handler = LogicThreadHandler::start_thread(
            logic_thread_input_channel_receiver, 
            winit_event_bus.clone(),
            render_state_bus.clone()
        );

        // Save to App
        // Timer
        self.timer = Some(timer); 
        
        // Buses
        self.winit_event_bus = Some(winit_event_bus);
        self.render_state_bus = Some(render_state_bus);

        // Thread Channel Senders
        self.render_thread_input_channel_sender = Some(render_thread_input_channel_sender);
        self.logic_thread_input_channel_sender = Some(logic_thread_input_channel_sender);

        // Thread handlers
        self.render_thread_handler = Some(render_thread_handler);
        self.logic_thread_handler = Some(logic_thread_handler);

        // Send signals for start event 
        self.logic_thread_input_channel_sender.as_ref().unwrap().send(LogicThreadInputSignal::Start).unwrap();
        self.render_thread_input_channel_sender.as_ref().unwrap().send(RenderThreadInputSignal::Start).unwrap();
    }

    /* fn suspended(&mut self, event_loop: &ActiveEventLoop) {
        
    }

    fn exiting(&mut self, event_loop: &ActiveEventLoop) {
        
    } */

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {  
        match event {
            WindowEvent::CloseRequested => { 
                // Graceful shutdown

                self
                .logic_thread_input_channel_sender
                .as_ref()
                .unwrap()
                .send(LogicThreadInputSignal::Shutdown)
                .unwrap();
                
                self
                .logic_thread_handler
                .take()
                .unwrap()
                .handle
                .join()
                .unwrap();


                self
                .render_thread_input_channel_sender
                .as_ref()
                .unwrap()
                .send(RenderThreadInputSignal::Shutdown)
                .unwrap();
                
                self
                .render_thread_handler
                .take()
                .unwrap()
                .handle
                .join()
                .unwrap();

                event_loop.exit();
            },
            _ => {
                self.winit_event_bus.as_ref().unwrap().push(event.into()); 
            }
        } 
    }

    fn new_events(&mut self, _event_loop: &ActiveEventLoop, cause: StartCause) {
        match cause { 
            StartCause::Poll => {
                let timer = self.timer.as_mut().unwrap();
                timer.update();
                if timer.check_logic_threshold() {
                    // Swap buffers in Buses
                    self
                    .winit_event_bus
                    .as_ref()
                    .unwrap()
                    .swap();

                    // Send signal to Logic thread
                    self
                    .logic_thread_input_channel_sender
                    .as_ref()
                    .unwrap()
                    .send(LogicThreadInputSignal::LogicTick)
                    .unwrap();
                }
                if timer.check_frame_threshold() {
                    // Swap buffers in Buses happens only in logic tick 
                    // Because time for logic bigger than frame time
                    
                    // Send signal to Render thread 
                    self
                    .render_thread_input_channel_sender
                    .as_ref()
                    .unwrap()
                    .send(RenderThreadInputSignal::FrameTick)
                    .unwrap();
                }
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
