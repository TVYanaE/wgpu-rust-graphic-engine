use glam::{
    Mat4
};

pub struct CameraSystem {
    view_project_matrix: Mat4
}

impl CameraSystem {
    pub fn new() -> Self {
        Self {  }
    }

    pub fn run(&mut self) {
        let top = +10
        bottom = -10
        aspect = width / height
        right = top * aspect
        left = bottom * aspect        

    }
}



