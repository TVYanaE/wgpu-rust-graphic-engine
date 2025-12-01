use std::{
    rc::Rc,
    cell::RefCell,
};
use wgpu::{
    TextureView, Sampler, BindGroup,
};

#[derive(Clone, Debug)]
pub struct SpriteMaterial {
    texture_view: Rc<TextureView>,
    sampler: Rc<Sampler>,
    bind_group: RefCell<Option<Rc<BindGroup>>>, 
}

impl SpriteMaterial {
    pub fn new(texture_view: Rc<TextureView>, sampler: Rc<Sampler>) -> Self {
        Self { 
            texture_view: texture_view, 
            sampler: sampler,
            bind_group: RefCell::new(None) 
        }
    }

    pub fn set_bind_group(&self, bind_group: BindGroup) {
        *self.bind_group.borrow_mut() = Some(Rc::new(bind_group));
    }
    pub fn get_texture_view(&self) -> Rc<TextureView> {
        self.texture_view.clone()
    }
    pub fn get_sampler(&self) -> Rc<Sampler> {
        self.sampler.clone()
    }
    pub fn get_bind_group(&self) -> Rc<BindGroup> {
        self.bind_group.borrow().clone().unwrap()
    }
}
