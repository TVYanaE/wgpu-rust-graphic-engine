use cgmath::{Point3, Vector3, Vector4, Matrix4, SquareMatrix, InnerSpace, perspective};
use winit::{
    keyboard::{KeyCode},
};

pub struct CameraController {
    speed: f32,
    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_left_pressed: bool,
    is_right_pressed: bool,
}

impl CameraController {
    pub fn new(speed: f32) -> Self {
        Self { 
            speed, 
            is_forward_pressed: false,
            is_backward_pressed: false,
            is_left_pressed: false,
            is_right_pressed: false
        }
    }

    pub fn handle_key(&mut self, code: KeyCode, is_pressed: bool) -> bool {
        match code {
            KeyCode::ArrowUp => {
                self.is_forward_pressed = is_pressed;
                true
            },
            KeyCode::ArrowLeft => {
                self.is_left_pressed = is_pressed;
                true
            },
            KeyCode::ArrowDown => {
                self.is_backward_pressed = is_pressed;
                true
            },
            KeyCode::ArrowRight => {
                self.is_right_pressed = is_pressed;
                true
            },
            _ => false
        }
    }

    pub fn update_camera(&self, camera: &mut Camera) {
        let forward = camera.target - camera.eye;
        let forward_norm = forward.normalize();
        let forward_mag = forward.magnitude();

        if self.is_forward_pressed && forward_mag > self.speed {
            camera.eye += forward_norm * self.speed;
        }

        if self.is_backward_pressed {
            camera.eye -= forward_norm * self.speed;
        }

        let right = forward_norm.cross(camera.up);

        let forward = camera.target - camera.eye;
        let forward_mag = forward.magnitude();

        if self.is_right_pressed {
            camera.eye = camera.target - (forward + right * self.speed).normalize() * forward_mag;
        }
        if self.is_left_pressed {
            camera.eye = camera.target - (forward - right * self.speed).normalize() * forward_mag;
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_projection: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        Self { 
            view_projection: Matrix4::identity().into(),  
        }
    }

    pub fn update_view_projection(&mut self, camera: &Camera) {
        self.view_projection = camera.build_view_projection_matrix().into();
    }
}

pub struct Camera {
    pub eye: Point3<f32>,
    pub target: Point3<f32>,
    pub up: Vector3<f32>,
    pub aspect: f32,
    pub fovy: f32,
    pub z_near: f32,
    pub z_far: f32,
}

pub const OPENGL_TO_WGPU_MATRIX: Matrix4<f32> = Matrix4::from_cols(
    Vector4::new(1.0, 0.0, 0.0, 0.0), 
    Vector4::new(0.0, 1.0, 0.0, 0.0), 
    Vector4::new(0.0, 0.0, 0.5, 0.0), 
    Vector4::new(0.0, 0.0, 0.5, 1.0),
);

impl Camera {
    pub fn build_view_projection_matrix(&self) -> Matrix4<f32> {
        let view_matrix = Matrix4::look_at_rh(self.eye, self.target, self.up);
        let projection_matrix = perspective(cgmath::Deg(self.fovy), self.aspect, self.z_near, self.z_far);

        return OPENGL_TO_WGPU_MATRIX * projection_matrix * view_matrix;
    }
}

