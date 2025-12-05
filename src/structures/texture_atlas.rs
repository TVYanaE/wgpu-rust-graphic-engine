use std::{
    collections::HashMap,
    rc::Rc,
    path::Path,
    fs::read as read_file,
};
use wgpu::{
    Texture, TextureView, TextureFormat, TextureDescriptor, TextureViewDescriptor, 
    Device, Queue, 
    Extent3d,
    TexelCopyTextureInfo, TexelCopyBufferLayout,
};
use ktx2::{
    Reader,
    Format as KTXTextureFormat,
};
use serde::{Serialize, Deserialize};
use crate::{
    aliases::InternalTextureID,
    structures::texture_info::{self, TextureInfo},
};

// for parsing JSON meta 

#[derive(Debug, Serialize, Deserialize)]
struct TextureAtlasSize {
    pub w: u32,
    pub h: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Meta {
    pub image: String,
    pub size: TextureAtlasSize,
}

#[derive(Debug, Serialize, Deserialize)]
struct Frame {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Image {
    pub filename: String,
    pub frame: Frame 
}

#[derive(Debug, Serialize, Deserialize)]
struct TexturesInfoRaw {
    pub frames: Vec<Image>,
    pub meta: Meta,
}

#[allow(dead_code)]
pub struct TextureAtlas {
    data: Texture,
    texture_view: Rc<TextureView>,
    texture_info_storage: HashMap<InternalTextureID, TextureInfo>  
}

impl TextureAtlas {
    pub fn new(
        texture_atlas_file_path: impl AsRef<Path>, 
        texture_atlas_meta_path: impl AsRef<Path>, 
        device: &Device, 
        queue: &Queue
    ) -> Self {
        let mut texture_info_storage: HashMap<InternalTextureID, TextureInfo> = HashMap::new(); 

        // reading texture atlas file as raw bytes
        let texture_bytes = read_file(texture_atlas_file_path).unwrap();

        // reading texture atlas meta file
        let textures_info_raw: TexturesInfoRaw = serde_json::from_str(
            &std::fs::read_to_string(texture_atlas_meta_path).unwrap()
        )
        .unwrap(); 
        
        let texture_atlas_meta = textures_info_raw.meta;

        let texture_atlas_size_x = texture_atlas_meta.size.w as f32;
        let texture_atlas_size_y = texture_atlas_meta.size.h as f32;

        for (texture_number, image) in textures_info_raw.frames.into_iter().enumerate() {
            let internal_texture_id = texture_number as InternalTextureID;


            let texture_position_x = image.frame.x as f32;
            let texture_position_y = image.frame.y as f32;
            let texture_size_x = image.frame.w as f32;
            let texture_size_y = image.frame.h as f32;

            let uv_offset_x = texture_position_x / texture_atlas_size_x;
            let uv_offset_y = texture_position_y / texture_atlas_size_y;
            let uv_scale_x = texture_size_x / texture_atlas_size_x;
            let uv_scale_y = texture_size_y / texture_atlas_size_y;

            let texture_info = TextureInfo {
                uv_offset: [uv_offset_x, uv_offset_y],
                uv_scale: [uv_scale_x, uv_scale_y],
            };

            texture_info_storage.insert(internal_texture_id, texture_info);
 
        }  

        let ktx_texture = Reader::new(texture_bytes).unwrap();
         
        let ktx_texture_header = ktx_texture.header();
        let ktx_texture_level = ktx_texture.levels();

        let ktx_texture_width = ktx_texture_header.pixel_width;
        let ktx_texture_height = ktx_texture_header.pixel_height;

        let texture_format = match ktx_texture_header.format.unwrap() {
            KTXTextureFormat::R8G8B8A8_SRGB => TextureFormat::Rgba8UnormSrgb,
            KTXTextureFormat::R8G8B8A8_UNORM => TextureFormat::Rgba8Unorm,
            KTXTextureFormat::BC7_SRGB_BLOCK => TextureFormat::Bc7RgbaUnormSrgb,
            other => {panic!("Unsuported texture format {:?}", other)}
        };
        
        let texture_size = Extent3d {
            width: ktx_texture_width,
            height: ktx_texture_height,
            depth_or_array_layers: 1,
        };

        let texture_descriptor = TextureDescriptor {
            label: Some(&texture_atlas_meta.image),
            size: texture_size,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: texture_format.clone(),
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
            mip_level_count: ktx_texture_header.level_count,
        };

        let texture = device.create_texture(&texture_descriptor);

        for (i, level) in ktx_texture_level.into_iter().enumerate() {
            let texture_data = level.data;

            let texture_copy_info = TexelCopyTextureInfo {
                texture: &texture,
                mip_level: i as u32,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All
            };

            /*let mip_width  = (ktx_texture_width  >> i).max(1);
            let mip_height = (ktx_texture_height >> i).max(1);

            let pixel_size = 4; // RGBA8 = 4 bytes per pixel

            let bytes_per_row = pixel_size * mip_width;
            let rows_per_image = mip_height; */

            let mip_width  = ktx_texture_width  >> i;
            let mip_height = ktx_texture_height >> i;

            let blocks_x = (mip_width  + 3) / 4;
            let blocks_y = (mip_height + 3) / 4;

            let bytes_per_row = blocks_x * 16;
            let rows_per_image = blocks_y;

            let data_layout = TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(bytes_per_row),
                rows_per_image: Some(rows_per_image)
            };
            
            let mip_size = Extent3d {
                width: (ktx_texture_width >> i).max(1),
                height: (ktx_texture_height >> i).max(1),
                depth_or_array_layers: 1,
            };

            queue.write_texture(
                texture_copy_info, 
                texture_data, 
                data_layout, 
                mip_size
            );
        }

        let texture_view_descriptor = TextureViewDescriptor {
            label: Some("texture_vie"),
            format: Some(texture_format),
            dimension: Some(wgpu::TextureViewDimension::D2),
            usage: Some(wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST),
            aspect: wgpu::TextureAspect::All,
            base_mip_level: 0,
            mip_level_count: Some(ktx_texture_header.level_count),
            base_array_layer: 0,
            array_layer_count: None,
        };

        let texture_view = texture.create_view(&texture_view_descriptor); 

        Self { 
            data: texture, 
            texture_view: Rc::new(texture_view), 
            texture_info_storage: texture_info_storage, 
        }
    }

    pub fn get_texture_view(&self) -> Rc<TextureView> {
        self.texture_view.clone()
    }

    pub fn get_texture_info(&self, internal_texture_id: InternalTextureID) -> Option<TextureInfo> {
        self.texture_info_storage.get(&internal_texture_id).copied()
    }
}
