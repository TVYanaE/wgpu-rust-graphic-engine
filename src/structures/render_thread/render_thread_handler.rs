use std::{
    cell::RefCell,
    sync::Arc,
};
use winit::{
    window::Window,
};
use crate::{
    structures::{
        states::{
            render_thread_state::RenderThreadState,
        },
        managers::{
            material_manager::MaterialManager
        },
    },
};

pub struct RenderThread {
    state: Box<RefCell<RenderThreadState>>
}

impl RenderThread {
    pub async fn new(window: Arc<Window>) -> Self {
        let state = RenderThreadState::new(window).await;

        Self {state: Box::new(RefCell::new(state))}
    }
    pub fn get_material_manager(&self) -> Arc<MaterialManager> {
        self.state.borrow().material_manager.clone()
    }
}
