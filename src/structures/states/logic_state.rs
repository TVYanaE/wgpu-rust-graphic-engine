use std::{
    sync::{Arc},
};
use shipyard::{
    World,
    IntoWorkload,
    Workload,
};
use crate::{
    structures::{
        managers::{
            material_manager::MaterialManager
        },
        uniques::{
            material_manager_ref::MaterialManagerRef,
            render_item_cache::RenderItemCache,
            logical_render_batches_cache::LogicalRenderBatchesCache,
        },
        components::{
            position_component::PositionComponent,
            size_component::SizeComponent,
            sprite_component::SpriteComponent,
        },
        systems::{
            render_system::{prepare_render_items, prepare_logical_render_batches},
        },
    },
};

pub struct LogicState {
    pub world: World,
}

impl LogicState {
    pub fn new(material_manager: Arc<MaterialManager>) -> Self {
        let mut world = World::default();

        let logical_render_batches_cache = LogicalRenderBatchesCache::new();
        let render_item_cache = RenderItemCache::new();
        let material_manager_ref = MaterialManagerRef::new(material_manager);

        world.add_unique(material_manager_ref);
        world.add_unique(render_item_cache);
        world.add_unique(logical_render_batches_cache);

        let _entity = world.add_entity((PositionComponent::default(), SizeComponent::default(), SpriteComponent::default()));

        world.add_workload(Self::workload);

        Self { 
            world, 
        }
    }

    fn workload() -> Workload {
        (prepare_render_items, prepare_logical_render_batches).into_workload()
    }

    pub fn run_tact(&mut self) { 
        self.world.run_workload(Self::workload).unwrap();
    }
}


