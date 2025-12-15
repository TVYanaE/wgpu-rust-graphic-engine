mod traits;
mod enums;
mod structures;
mod functions;
mod aliases;
mod consts;

use std::{
    sync::Arc,
};
use winit::{
    application::ApplicationHandler,
    event::{WindowEvent, KeyEvent, StartCause, },
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
    keyboard::{PhysicalKey},
};
use crate::{
    structures::{
        states::{
            app_state::AppState,
        },
        timer::Timer,
    },
};

#[derive(Default)]
struct App {
    app_state: Option<AppState>, 
    timer: Option<Timer>,
}

impl ApplicationHandler for App {

    // This event is triggered after StartCase::Init 
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // Initialization part 
        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );

        let app_state_ref = self.app_state.as_mut().unwrap();

        pollster::block_on(app_state_ref.init_render_state(window.clone())).unwrap();

        app_state_ref.init_logic_state().unwrap();

        app_state_ref.init_batcher().unwrap();

        // Render prepare part 

        let timer = self.timer.as_mut().unwrap();

        timer.update();

        window.request_redraw();
    }

    /* fn suspended(&mut self, event_loop: &ActiveEventLoop) {
        
    }

    fn exiting(&mut self, event_loop: &ActiveEventLoop) {
        
    } */

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {   
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                self.app_state.as_mut().unwrap().redraw_handle().unwrap();
            }
            WindowEvent::Resized(physical_size) => {
                // Reconfigures the size of the surface. We do not re-render
                // here as this event is always followed up by redraw request.
                self.app_state.as_mut().unwrap().resize_handle(physical_size).unwrap();
            },
            WindowEvent::KeyboardInput {  
                event: KeyEvent {
                    physical_key: PhysicalKey::Code(key_code),
                    state: key_state,
                    ..
                },
                ..
            } => {
                self.app_state.as_mut().unwrap().keyboard_input_handle(key_code, key_state.is_pressed()).unwrap();
            }
            _ => (),
        }
    }

    fn new_events(&mut self, _event_loop: &ActiveEventLoop, cause: StartCause) {
        match cause {
            StartCause::Init => {
                let mut app_state = AppState::default();

                let timer = Timer::new();
                
                self.app_state = Some(app_state); 
                self.timer = Some(timer);
            },
            StartCause::Poll => {
                let timer = self.timer.as_mut().unwrap();
                let app_state_ref = self.app_state.as_mut().unwrap();

                timer.update(); 

                app_state_ref.get_window().request_redraw();

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
