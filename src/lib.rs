mod camera;
mod vertex;
mod shapes;
mod shader_library;
mod traits;
mod managers;
mod enums;
mod structures;
mod functions;
mod aliases;
mod components;
mod systems;

use std::{
    sync::Arc,
};
use winit::{
    application::ApplicationHandler,
    event::{WindowEvent, KeyEvent, StartCause},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
    keyboard::{PhysicalKey},
};
use crate::{
    structures::{
        states::{
            ecs_state::ECSState,
            render_state::RenderState,
        },
        timer::Timer,
    },
};

#[derive(Default)]
struct App {
    ecs_state: Option<ECSState>,
    render_state: Option<RenderState>,
    timer: Option<Timer>,
}

impl ApplicationHandler for App {

    // This event is triggered after StartCase::Init 
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // Create window object
        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );

        let render_state = pollster::block_on(RenderState::new(window.clone()));
        self.render_state = Some(render_state);

        let timer = self.timer.as_mut().unwrap();

        timer.update();

        // methods for run_fixed, run_variable, prepare and after that request redraw.

        window.request_redraw();
    }

    /* fn suspended(&mut self, event_loop: &ActiveEventLoop) {
        
    }

    fn exiting(&mut self, event_loop: &ActiveEventLoop) {
        
    } */

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let render_state = self.render_state.as_mut().unwrap();
        let ecs_state = self.ecs_state.as_mut().unwrap();
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                render_state.render();
            }
            WindowEvent::Resized(physical_size) => {
                // Reconfigures the size of the surface. We do not re-render
                // here as this event is always followed up by redraw request.
                render_state.reconfigure_surface(physical_size);
            },
            WindowEvent::KeyboardInput {  
                event: KeyEvent {
                    physical_key: PhysicalKey::Code(key_code),
                    state: key_state,
                    ..
                }, 
                .. 
            } => ecs_state.handle_key(event_loop, key_code, key_state.is_pressed()),
            _ => (),
        }
    }

    fn new_events(&mut self, _event_loop: &ActiveEventLoop, cause: StartCause) {
        match cause {
            StartCause::Init => {
                let ecs_state = ECSState::new();
                let timer = Timer::new();
                self.ecs_state = Some(ecs_state);
                self.timer = Some(timer);
            },
            StartCause::Poll => {
                let timer = self.timer.as_mut().unwrap();

                timer.update();

                // methods for run_fixed, run_variable, prepare and after that trigger RedrawRequested. 
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
