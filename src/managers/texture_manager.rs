use std::{
    collections::HashMap,
    fs::{read},
    path::Path,
    cell::RefCell,
    rc::Rc,
};
use wgpu::{
    Texture,
    Device, Queue,
    TextureFormat, TextureDescriptor, Extent3d, TexelCopyTextureInfo, TexelCopyBufferLayout, TextureViewDescriptor,
    TextureView,
};
use ktx2::{
    Format as KTXTextureFormat,
};



pub struct TextureManager {
    texture_storage: RefCell<HashMap<String, Texture>>,
    texture_view_storage: RefCell<HashMap<String, Rc<TextureView>>>,
}

impl TextureManager {
    pub fn new() -> Self {
        Self { 
            texture_storage: RefCell::new(HashMap::new()),
            texture_view_storage: RefCell::new(HashMap::new()),
        }
    }
    pub fn load_texture(&self, texture_path: impl AsRef<Path>, device: &Device, queue: &Queue) {
        let texture_name = texture_path.as_ref().file_stem().unwrap().to_str().unwrap().to_string();
        let texture_bytes = read(texture_path).unwrap();


        let ktx_texture = ktx2::Reader::new(texture_bytes).unwrap();

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
            label: Some(&texture_name),
            size: texture_size,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: texture_format,
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

        let texture_view = texture.create_view(&TextureViewDescriptor::default());

        self.texture_storage.borrow_mut().insert(texture_name.clone(), texture);
        self.texture_view_storage.borrow_mut().insert(texture_name, Rc::new(texture_view));
    }
    pub fn get_default_texture_view(&self, texture_name: &str) -> Option<Rc<TextureView>> {
        self.texture_view_storage.borrow().get(texture_name).cloned()
    }
}
