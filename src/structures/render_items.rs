use std::{
    rc::Rc,
};
use crate::{
    structures::material::Material,
};

#[derive(Debug, Clone)]
pub struct RenderItem {
    pub instance_position: [f32; 3],
    pub instance_size: [f32; 3],
    pub material: Rc<Material>,
}


