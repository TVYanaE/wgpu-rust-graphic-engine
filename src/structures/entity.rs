use crate::{
    aliases::{EntityID, GenerationID},
};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Entity {
    pub entity_id: EntityID,
    pub generation: GenerationID,
}
