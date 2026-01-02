use shipyard::{
    IntoIter, UniqueViewMut, View,
    Workload,
};
use crate::{
    modules::{ 
        logic_thread::{ 
            components::{
                sprite_component::SpriteComponent,
                position_component::PositionComponent,
                size_component::SizeComponent,
                camera_component::CameraComponent,
            }, 
        },
        shared::{
            render_state::{RenderItem, RenderState, ActivieCamera},
        },
    },        
};


pub fn get_prepare_render_state_phase_workloads() -> Vec<Workload> {
    let prepare_render_items_workload = Workload::new("PrepareRenderState")
        .with_system(prepare_render_state);

    return vec![prepare_render_items_workload];
}

fn prepare_render_state(
    position_component_view: View<PositionComponent>,
    size_component_view: View<SizeComponent>,
    sprite_component_view: View<SpriteComponent>,
    camera_component_view: View<CameraComponent>,
    mut render_state_mut_view: UniqueViewMut<RenderState>,
) {
    let mut render_items = Vec::new();

    for (position, size, sprite) 
    in (
        &position_component_view, 
        &size_component_view, 
        &sprite_component_view
    )
    .iter() {

        let render_item = RenderItem {
            material_id: sprite.material_id,
            instance_size: [size.size_x, size.size_y, size.size_z],
            instance_position: [position.position.x, position.position.y, position.position.z]
        };

        render_items.push(render_item); 
    }
    
    render_state_mut_view.render_item_cache = Some(render_items); 

    // TODO!: Choosing activie Camera 
    camera_component_view.iter().for_each(|camera_component|{
        let activie_camera = ActivieCamera {
            camera_position: camera_component.camera_position,
            up: camera_component.up,
            far: camera_component.far,
            near: camera_component.near,
            top_bound: camera_component.top_bound,
            bottom_bound: camera_component.bottom_bound,
            view_target: camera_component.view_target,
        };

        render_state_mut_view.activie_camera = Some(activie_camera);
    });
}


