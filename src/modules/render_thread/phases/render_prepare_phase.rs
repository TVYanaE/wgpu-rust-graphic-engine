use std::{
    time::{
        Instant,
    },
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
            render_state::{
                ActivieCamera, RenderState, RenderItem,
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
            },
            render_state_snapshot::RenderStateSnapshot,
        },
        batcher::batching,
        render_batch::RenderBatch,
    },
};

pub fn render_prepare_phase(
    device: &Device,
    render_batches_cache: &mut Vec<RenderBatch>,
    material_manager: &MaterialManager,
    view_projection_uniform_matrix_cache: &mut ViewProjectionUniformMatrixCache,
    width: f32,
    height: f32,
    render_state_snapshot: &RenderStateSnapshot,
) {
    let render_state = if let Some(alpha) = calc_alpha(render_state_snapshot) {
        interpolation(alpha, render_state_snapshot)
    } else {
        render_state_snapshot.curr_render_state_snapshot.as_ref().unwrap().clone()
    };

    batching(device, &mut render_state.render_item_cache.unwrap().clone(), render_batches_cache, material_manager);

    refresh_view_project_uniform_matrix_cache(
        &mut render_state.activie_camera.unwrap().clone(),
        width, 
        height,
        view_projection_uniform_matrix_cache);
}

fn calc_alpha(render_state_snapshot: &RenderStateSnapshot) -> Option<f32> {
    if let Some(_prev_render_state) = &render_state_snapshot.prev_render_state_snapshot {
        let t_curr = render_state_snapshot.t_curr.as_ref().unwrap();
        let dt_logic = render_state_snapshot.delta_time_logic.unwrap().as_secs_f32();

        let time_now = Instant::now();
        let elapsed = time_now.duration_since(*t_curr);
        let raw_alpha = elapsed.as_secs_f32() / dt_logic;

        let alpha = raw_alpha.clamp(0.0, 1.0);

        return Some(alpha);
    } 
    else {
        return None;
    } 
} 


fn interpolation(alpha: f32, render_state_snapshot: &RenderStateSnapshot) -> RenderState {
    let prev = render_state_snapshot.prev_render_state_snapshot.as_ref().unwrap();
    let curr = render_state_snapshot.curr_render_state_snapshot.as_ref().unwrap();

    // Camera interpolation
    let interpolated_camera = match (prev.activie_camera, curr.activie_camera) {
        (Some(prev_cam), Some(curr_cam)) => Some(ActivieCamera {
            camera_position: prev_cam.camera_position.lerp(curr_cam.camera_position, alpha),
            view_target: prev_cam.view_target.lerp(curr_cam.view_target, alpha),
            up: prev_cam.up.lerp(curr_cam.up, alpha),
            bottom_bound: prev_cam.bottom_bound * (1.0 - alpha) + curr_cam.bottom_bound * alpha,
            top_bound: prev_cam.top_bound * (1.0 - alpha) + curr_cam.top_bound * alpha,
            near: prev_cam.near * (1.0 - alpha) + curr_cam.near * alpha,
            far: prev_cam.far * (1.0 - alpha) + curr_cam.far * alpha,
        }),
        _ => curr.activie_camera, 
    };

    // RenderItems interpolation
    let interpolated_render_items = match (&prev.render_item_cache, &curr.render_item_cache) {
        (Some(prev_items), Some(curr_items)) => {
            let mut items = Vec::with_capacity(curr_items.len());
            for (p, c) in prev_items.iter().zip(curr_items.iter()) {
                items.push(RenderItem {
                    instance_position: [
                        p.instance_position[0] * (1.0 - alpha) + c.instance_position[0] * alpha,
                        p.instance_position[1] * (1.0 - alpha) + c.instance_position[1] * alpha,
                        p.instance_position[2] * (1.0 - alpha) + c.instance_position[2] * alpha,
                    ],
                    instance_size: [
                        p.instance_size[0] * (1.0 - alpha) + c.instance_size[0] * alpha,
                        p.instance_size[1] * (1.0 - alpha) + c.instance_size[1] * alpha,
                        p.instance_size[2] * (1.0 - alpha) + c.instance_size[2] * alpha,
                    ],
                    material_id: c.material_id,  
                });
            }
            Some(items)
        }
        _ => curr.render_item_cache.clone(),
    };

    RenderState {
        render_item_cache: interpolated_render_items,
        activie_camera: interpolated_camera,
    }
}

fn refresh_view_project_uniform_matrix_cache(
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
