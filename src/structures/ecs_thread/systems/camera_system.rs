use glam::{
    Mat4
};
use shipyard::{
    UniqueView,
    UniqueViewMut
};
use crate::{
    structures::camera::{Camera, CameraUniformMatrix}
};

pub fn calc_view_project_matrix(camera: UniqueView<Camera>, mut camera_uniform_matrix: UniqueViewMut<CameraUniformMatrix>) {
    let view = Mat4::look_at_rh(
        camera.camera_position,
        camera.view_target,
        camera.up,
    );

    let top = camera.top_bound;
    let bottom = camera.bottom_bound;

    let half_width = (top - bottom) * camera.aspect * 0.5;

    let left = -half_width;
    let right = half_width;

    let projection = Mat4::orthographic_rh(
        left,
        right,
        bottom,
        top,
        camera.near,
        camera.far,
    );

    let view_projection_matrix = projection * view;

    let camera_uniform_matrix_new = CameraUniformMatrix::from_mat4(&view_projection_matrix);

    camera_uniform_matrix.view_projection_matrix = camera_uniform_matrix_new.view_projection_matrix;
}


