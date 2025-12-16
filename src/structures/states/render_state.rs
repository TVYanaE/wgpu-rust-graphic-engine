use std::{
    path::PathBuf, sync::Arc
};
use wgpu::{
    Instance, InstanceDescriptor, 
    Device, DeviceDescriptor, Features,
    Adapter, RequestAdapterOptions, 
    Queue, 
    TextureFormat, TextureViewDescriptor, CommandEncoderDescriptor, Operations,
    Surface, SurfaceConfiguration, RenderPassDescriptor, RenderPassColorAttachment,
    TextureUsages, CompositeAlphaMode, PresentMode,
    PrimitiveState, IndexFormat,
    PrimitiveTopology, MultisampleState, 
    Buffer, BufferUsages, BindGroupDescriptor, BindGroupEntry,
    util::{BufferInitDescriptor, DeviceExt}, 
};
use winit::{
    window::Window,
    dpi::PhysicalSize, 
};
use crate::{ 
    enums::{
        bind_group_layout_name_enum::BindGroupLayoutName,
        bind_group_name_enum::BindGroupName,
        render_pipeline_name_enum::RenderPipelineName,
        sampler_name_enum::SamplerName,
    },
    structures::{
        shader_library::ShaderLibrary, 
        material::Material,
        render_batch::RenderBatch,
        camera::{CameraUniformMatrix, CameraStorage},
        shapes::square::{SQUARE_INDEX, SQUARE_VERTICES},
        managers::{
            texture_atlas_manager::TextureAtlasManager,
            sampler_manager::SamplerManager,
            bind_group_layout_manager::BindGroupLayoutManager,
            bind_group_manager::BindGroupManager,
            render_pipeline_manager::RenderPipelineManager,
            material_manager::MaterialManager,
        },
    },
};

#[allow(dead_code)]
pub struct RenderState {
    pub instance: Instance,
    pub window: Arc<Window>,
    pub device: Device,
    pub adapter: Adapter,
    pub queue: Queue,
    pub surface: Surface<'static>,
    pub surface_configuration: SurfaceConfiguration,
    pub window_size: PhysicalSize<u32>,
    pub surface_texture_format: TextureFormat,
    pub shader_library: ShaderLibrary,
    pub texture_atlas_manager: TextureAtlasManager,
    pub sampler_manager: SamplerManager,
    pub bind_group_layout_manager: BindGroupLayoutManager,
    pub bind_group_manager: BindGroupManager,
    pub material_manager: Arc<MaterialManager>,
    pub render_pipeline_manager: RenderPipelineManager,
    pub square_vertex_buffer: Buffer,
    pub square_index_buffer: Buffer,
    pub index_format: IndexFormat,
    pub camera_storage: CameraStorage,
}

impl RenderState {
    pub async fn new(window: Arc<Window>) -> Self {
        let instance = Instance::new(&InstanceDescriptor::default());
        let adapter = instance
            .request_adapter(&RequestAdapterOptions::default())
            .await
            .unwrap();

        let required_features = Features::TEXTURE_COMPRESSION_BC; 

        let (device, queue) = adapter
            .request_device(&DeviceDescriptor {
                required_features: required_features,
                ..Default::default()
            })
            .await
            .unwrap();
        let surface = instance.create_surface(window.clone()).unwrap(); 

        // Surface config
        let window_size = window.inner_size();
        let surface_capabilities = surface.get_capabilities(&adapter);
        let surface_texture_format = surface_capabilities.formats[0];

        let surface_configuration = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_texture_format.clone(),
            // Request compatibility with the sRGB-format texture view we‘re going to create later.
            view_formats: vec![surface_texture_format.add_srgb_suffix()],
            alpha_mode: CompositeAlphaMode::Auto,
            width: window_size.width,
            height: window_size.height,
            desired_maximum_frame_latency: 2,
            present_mode: PresentMode::AutoVsync,
        };
        surface.configure(&device, &surface_configuration);
 
        let shader_library = ShaderLibrary::new(&device);
        
        let texture_atlas_manager= TextureAtlasManager::new();

        //let execute_dir = current_exe().unwrap().parent().unwrap().to_path_buf();

        let texture_atlas_file_path = PathBuf::from("assets/texture_atlases/texture_atlas_1/texture_atlas_1.ktx2"); 

        let texture_atlas_meta_path = PathBuf::from("assets/texture_atlases/texture_atlas_1/texture_atlas_1.json");

        texture_atlas_manager.load_texture_atlas(
            1, 
            texture_atlas_file_path, 
            texture_atlas_meta_path, 
            &device, 
            &queue
        );

        let sampler_manager = SamplerManager::new(&device); 
        
        let default_sampler = sampler_manager.get_sampler(SamplerName::DefaultSampler).unwrap();

        let bind_group_layout_manager = BindGroupLayoutManager::new(&device);

        let bind_group_manager = BindGroupManager::new();

        bind_group_manager.create_bind_group(
            &device, 
            &bind_group_layout_manager.get_bind_group_layout(&BindGroupLayoutName::DefaultBindGroupLayout).unwrap(), 
            &texture_atlas_manager.get_texture_view(1).unwrap(), 
            &default_sampler, 
            Some("Texture atlas 1 bind group"), 
            BindGroupName::TextureAtlas1BindGroup,
        );

        let camera_uniform_matrix = CameraUniformMatrix::new(); 

        let camera_uniform_buffer = device.create_buffer_init(&
            BufferInitDescriptor {
                label: Some("Camera Uniform Buffer"),
                usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
                contents: bytemuck::cast_slice(&[camera_uniform_matrix]),
            }
        );

        let camera_bind_group_layout = bind_group_layout_manager
            .get_bind_group_layout(&BindGroupLayoutName::CameraBindGroupLayout)
            .unwrap();

        let camera_bind_group = device.create_bind_group(
            &BindGroupDescriptor {
                layout: camera_bind_group_layout.as_ref(),
                entries: &[
                    BindGroupEntry {
                        binding: 0,
                        resource: camera_uniform_buffer.as_entire_binding(), 
                    }, 
                ],
                label: Some("Camera Bind Group") 
            }
        ); 

        let camera_storage = CameraStorage {
            camera_bind_group: camera_bind_group,
            camera_uniform_buffer: camera_uniform_buffer,
        };

        let render_pipeline_manager = RenderPipelineManager::new();

        let primitive_state = PrimitiveState {
            topology: PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        };

        let multisample_state = MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false
        };

        render_pipeline_manager.create_render_pipeline(
            RenderPipelineName::DefaultRenderPipeline, 
            Some("Default Render Pipeline"), 
            &device, 
            &surface_configuration, 
            &[
                &camera_bind_group_layout,
                &bind_group_layout_manager.get_bind_group_layout(&BindGroupLayoutName::DefaultBindGroupLayout).unwrap(), 
            ], 
            &shader_library.basic_shader_module, 
            primitive_state, 
            multisample_state);

        let square_vertex_buffer_init_descriptor = BufferInitDescriptor {
            label: Some("Square Vertex buffer"),
            contents: bytemuck::cast_slice(SQUARE_VERTICES),
            usage: BufferUsages::VERTEX,
        };

        let square_vertex_buffer = device.create_buffer_init(&square_vertex_buffer_init_descriptor);

        let square_index_buffer_init_descriptor = BufferInitDescriptor {
            label: Some("Square index buffer"),
            contents: bytemuck::cast_slice(SQUARE_INDEX),
            usage: BufferUsages::INDEX,
        };

        let square_index_buffer = device.create_buffer_init(&square_index_buffer_init_descriptor);

        let material_manager = MaterialManager::new();
   
        // ant material
        let texture_for_ant_material = texture_atlas_manager.get_texture_info(1, 0).unwrap(); 

        let ant_material = Material {
            uv_scale: texture_for_ant_material.uv_scale,
            uv_offset: texture_for_ant_material.uv_offset,
            bind_group: bind_group_manager.get_bind_group(BindGroupName::TextureAtlas1BindGroup).unwrap(),
            render_pipeline: render_pipeline_manager.get_render_pipeline(RenderPipelineName::DefaultRenderPipeline).unwrap(),
        };

        // pig material
        let texture_for_pig_material = texture_atlas_manager.get_texture_info(1, 1).unwrap(); 

        let pig_material = Material {
            uv_scale: texture_for_pig_material.uv_scale,
            uv_offset: texture_for_pig_material.uv_offset,
            bind_group: bind_group_manager.get_bind_group(BindGroupName::TextureAtlas1BindGroup).unwrap(),
            render_pipeline: render_pipeline_manager.get_render_pipeline(RenderPipelineName::DefaultRenderPipeline).unwrap(),
        };

        material_manager.add_material(0, Arc::new(ant_material));
        material_manager.add_material(1, Arc::new(pig_material));

        let render_state = Self {
            device: device,
            surface_configuration: surface_configuration,
            window_size: window_size,
            queue: queue,
            window: window,
            adapter: adapter,
            surface: surface,
            instance: instance,
            surface_texture_format: surface_texture_format,
            shader_library: shader_library,
            sampler_manager: sampler_manager,
            bind_group_layout_manager: bind_group_layout_manager,
            bind_group_manager: bind_group_manager,
            texture_atlas_manager: texture_atlas_manager,
            render_pipeline_manager: render_pipeline_manager,
            material_manager: Arc::new(material_manager),
            square_vertex_buffer: square_vertex_buffer,
            square_index_buffer: square_index_buffer,
            index_format: IndexFormat::Uint16,
            camera_storage: camera_storage,
        };

        return render_state; 
    }

    pub fn draw_call(&mut self, render_batches: &[RenderBatch]) {
        let surface_current_texture = self.surface.get_current_texture().unwrap();

        let current_texture =  surface_current_texture.texture.clone(); 

        let texture_view_descriptor = TextureViewDescriptor {
            format: Some(self.surface_texture_format.add_srgb_suffix()),
            ..Default::default()
        };

        let current_texture_view = current_texture.create_view(&texture_view_descriptor);

        let command_encoder_description = CommandEncoderDescriptor::default(); 
        
        let operations = Operations {
            store: wgpu::StoreOp::Store,
            load: wgpu::LoadOp::Clear(wgpu::Color::BLUE),
        }; 

        let collor_attachments = RenderPassColorAttachment {
            view: &current_texture_view,
            depth_slice: None,
            resolve_target: None,
            ops: operations, 
        }; 

        let mut command_encoder = self.device.create_command_encoder(&command_encoder_description); 
        
        let render_pass_description = RenderPassDescriptor {
            label: None,
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            color_attachments: &[Some(collor_attachments)],
        };

        let mut render_pass = command_encoder.begin_render_pass(&render_pass_description);
        
        render_pass.set_bind_group(0, &self.camera_storage.camera_bind_group, &[]);

        for render_batch in render_batches {
            render_pass.set_pipeline(&render_batch.render_pipeline);
            render_pass.set_bind_group(1, render_batch.bind_group.as_ref(), &[]);
            render_pass.set_vertex_buffer(0, self.square_vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, render_batch.instance_buffer.slice(..));
            render_pass.set_index_buffer(self.square_index_buffer.slice(..), self.index_format);
            render_pass.draw_indexed(0..6, 0, 0..render_batch.instance_count);      
        }
        
        drop(render_pass);

        let command_buffer = command_encoder.finish();

        self.queue.submit([command_buffer]);

        surface_current_texture.present(); 
    }
 
    pub fn reconfigure_surface(&mut self, physical_size: PhysicalSize<u32>) {
        if physical_size.width == 0 || physical_size.height == 0 {
            return; 
        }

        self.surface_configuration.width = physical_size.width;
        self.surface_configuration.height = physical_size.height;

        self.surface.configure(&self.device, &self.surface_configuration); 
    }
 
}
