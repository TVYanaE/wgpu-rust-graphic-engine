use shipyard::{
    Unique
};
use crate::{
    structures::{
        render_items::RenderItem,
    },
};

#[derive(Unique)]
pub struct RenderItemCache {
    pub render_item_cache: Vec<RenderItem>
}

impl RenderItemCache {
    pub fn new() -> Self {
        Self { render_item_cache: Vec::new() }
    } 
}
