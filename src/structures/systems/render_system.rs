use std::{
    collections::HashMap,
    sync::Arc,
};
use shipyard::{
    IntoIter, UniqueView, UniqueViewMut, View
};
use crate::{
    structures::{
        render_items::RenderItem, 
        components::{
            sprite_component::SpriteComponent,
            position_component::PositionComponent,
            size_component::SizeComponent,
        },
        uniques::{
            render_item_cache::RenderItemCache,
            material_manager_ref::MaterialManagerRef,
            logical_render_batches_cache::LogicalRenderBatchesCache,
        },
        buffers_layouts::InstanceVertex,
        material::Material,
        logical_render_batch::LogicalRenderBatch,
    }, 
};

pub fn prepare_render_items(
    position_component_view: View<PositionComponent>,
    size_component_view: View<SizeComponent>,
    sprite_component_view: View<SpriteComponent>,
    material_manager_view: UniqueView<MaterialManagerRef>,
    mut render_item_cache_view: UniqueViewMut<RenderItemCache>,
) {
    for (position, size, sprite) 
    in (
        &position_component_view, 
        &size_component_view, 
        &sprite_component_view
    )
    .iter() {
        let material = material_manager_view.material_manager.get_material(sprite.material_id).unwrap();

        let render_item = RenderItem {
            material,
            instance_size: [size.size_x, size.size_y, size.size_z],
            instance_position: [position.position_x, position.position_y, position.position_z]
        };

        render_item_cache_view.render_item_cache.push(render_item);
    }
}

pub fn prepare_logical_render_batches(
    mut render_item_cache_view: UniqueViewMut<RenderItemCache>,
    mut logical_render_batches_cache: UniqueViewMut<LogicalRenderBatchesCache> 
) {
    let mut render_groups: HashMap<*const Material, Vec<RenderItem>> = HashMap::new();  

        for render_item in render_item_cache_view.render_item_cache.drain(..) {
            let key = Arc::as_ptr(&render_item.material);
            render_groups.entry(key).or_default().push(render_item.clone());
        }

        for (_key, group) in render_groups {
            let material: Arc<Material> = group[0].material.clone();

            let instances: Vec<InstanceVertex> = group
            .iter()
            .map(|render_item| {
                
                //test 
                /* println!("from batcher uv_offset x = {}", render_item.material.uv_offset[0]);
                println!("from batcher uv_offset y = {}", render_item.material.uv_offset[1]);
                println!("from batcher uv_scale x = {}", render_item.material.uv_scale[0]);
                println!("from batcher uv_scale y = {}", render_item.material.uv_scale[1]); */

                InstanceVertex {
                    instance_position: render_item.instance_position,
                    instance_size: render_item.instance_size,
                    uv_offset: render_item.material.uv_offset,
                    uv_scale: render_item.material.uv_scale,
            }
            })
            .collect();
            
            let logical_render_batch = LogicalRenderBatch {
                material,
                instances
            };
            
            logical_render_batches_cache.logical_render_batches.push(logical_render_batch);
        }
}

