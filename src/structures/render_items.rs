use std::{
    rc::Rc,
};
use crate::{
    structures::material::Material,
};

#[derive(Debug, Clone)]
pub struct RenderItem {
    pub instance_position: [f32; 2],
    pub instance_size: [f32; 2],
    pub material: Rc<Material>,
}


