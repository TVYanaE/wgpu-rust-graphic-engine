use shipyard::{
    EntityId,
};
use super::super::{
    components::{
        position_component::PositionComponent,
        size_component::SizeComponent,
        sprite_component::SpriteComponent,
    },
};

#[derive(Debug)]
pub struct MapObject {
    pub entity_id: EntityId, 
}

#[derive(Debug)]
pub struct MapObjectDescriptor {
    pub position_component: PositionComponent,
    pub size_component: SizeComponent,
    pub sprite_component: SpriteComponent,
}
