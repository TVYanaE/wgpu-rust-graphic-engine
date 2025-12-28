use std::{
    collections::HashMap,
    path::Path,
    cell::RefCell,
    rc::Rc,
};
use wgpu::{
    Device, Queue,
    TextureView,
};
use crate::{
    aliases::{TextureAtlasID, InternalTextureID},
    structures::{
        texture_atlas::TextureAtlas,
        texture_info::TextureInfo,
    },
};

pub struct TextureAtlasManager {
    texture_atlas_storage: RefCell<HashMap<TextureAtlasID, TextureAtlas>>,
}

impl TextureAtlasManager {
    pub fn new() -> Self {
        Self { 
            texture_atlas_storage: RefCell::new(HashMap::new()),
        }
    }
    pub fn load_texture_atlas(
        &self, 
        texture_atlas_id: TextureAtlasID,
        texture_atlas_file_path: impl AsRef<Path>,
        texture_atlas_meta_path: impl AsRef<Path>,
        device: &Device,
        queue: &Queue,
    ) {
        let texture_atlas = TextureAtlas::new(
            texture_atlas_file_path, 
            texture_atlas_meta_path, 
            device, 
            queue
        );

        self.texture_atlas_storage.borrow_mut().insert(texture_atlas_id, texture_atlas);
    }

    pub fn get_texture_view(&self, texture_atlas_id: TextureAtlasID) -> Option<Rc<TextureView>> {
        if let Some(texture_atlas) = self.texture_atlas_storage.borrow().get(&texture_atlas_id) {
            return Some(texture_atlas.get_texture_view());
        }
        else {
            return None;
        }
    }

    pub fn get_texture_info(
        &self, 
        texture_atlas_id: TextureAtlasID, 
        internal_texture_id: InternalTextureID
    ) -> Option<TextureInfo> {
        if let Some(texture_atlas) = self.texture_atlas_storage.borrow().get(&texture_atlas_id) {
            return texture_atlas.get_texture_info(internal_texture_id);
        }
        else {
            return None
        }
    }
}
