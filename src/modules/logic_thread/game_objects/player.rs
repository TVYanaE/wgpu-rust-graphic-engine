use shipyard::{
    EntityId,
};
use super::super::{
    components::{
        position_component::PositionComponent,
        size_component::SizeComponent,
        sprite_component::SpriteComponent,
        player_component::PlayerComponent,
    },
};

#[derive(Debug)]
pub struct Player {
    pub entity_id: EntityId,
}

#[derive(Debug)]
pub struct PlayerDescriptor {
    pub position_component: PositionComponent,
    pub size_component: SizeComponent,
    pub sprite_component: SpriteComponent,
    pub player_component: PlayerComponent,
}
