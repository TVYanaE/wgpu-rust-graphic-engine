use std::{
    sync::Arc,
};
use glam::{
    Mat4,
};
use wgpu::{
    Device,
};
use crate::{
    modules::{
        shared::{
            double_buffer_bus::DoubleBufferBus,
            render_state::{
                RenderState,
                ActivieCamera,
            },
        },
    },
};
use super::{
    super::{
        states::{
            gpu_state::{
                managers::{
                    material_manager::MaterialManager,
                },
                camera::ViewProjectionUniformMatrixCache,
            }
        },
        batcher::batching,
        render_batch::RenderBatch,
    },
};

pub fn render_prepare_phase(
    render_state_bus: Arc<DoubleBufferBus<RenderState>>,
    device: &Device,
    render_batches_cache: &mut Vec<RenderBatch>,
    material_manager: &MaterialManager,
    view_projection_uniform_matrix_cache: &mut ViewProjectionUniformMatrixCache,
    width: f32,
    height: f32,
) {
    let read_buffer = render_state_bus.get_read_buffer().as_ref().clone();

    let mut render_state = read_buffer[0].clone();

    batching(device, render_state.render_item_cache.as_mut().unwrap(), render_batches_cache, material_manager);

    refresh_view_project_uniform_matrix_cache(
        render_state.activie_camera.as_mut().unwrap(),
        width, 
        height,
        view_projection_uniform_matrix_cache);
}

pub fn refresh_view_project_uniform_matrix_cache(
    activie_camera: &mut ActivieCamera,
    width: f32,
    height: f32,
    view_projection_uniform_matrix_cache: &mut ViewProjectionUniformMatrixCache
) {
    let view = Mat4::look_at_rh(
        activie_camera.camera_position,
        activie_camera.view_target,
        activie_camera.up,
    );

    let top = activie_camera.top_bound;
    let bottom = activie_camera.bottom_bound;

    let aspect = width / height;

    let horizontal_half = (top - bottom) * aspect * 0.5;

    let left = - horizontal_half;
    let right = horizontal_half;
     
    let projection = Mat4::orthographic_rh(
        left,
        right,
        bottom,
        top,
        activie_camera.near,
        activie_camera.far,
    );

    let view_projection_matrix = projection * view;

    view_projection_uniform_matrix_cache.view_projection_matrix = view_projection_matrix.to_cols_array_2d();
}
