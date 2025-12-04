use std::{
    sync::Arc,
    rc::Rc,
    env::current_exe,
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
    PrimitiveTopology, MultisampleState, Buffer, BufferUsages,
    util::{BufferInitDescriptor, DeviceExt}, 
};
use winit::{
    window::Window,
    dpi::PhysicalSize, 
};
use crate::{
    shapes::square::{SQUARE_INDEX, SQUARE_VERTICES},
    shader_library::ShaderLibrary,
    managers::{
        texture_atlas_manager::TextureAtlasManager,
        sampler_manager::SamplerManager,
        bind_group_layout_manager::BindGroupLayoutManager,
        bind_group_manager::BindGroupManager,
        render_pipeline_manager::RenderPipelineManager,
        material_manager::MaterialManager,
    },
    enums::{
        bind_group_layout_name_enum::BindGroupLayoutName,
        bind_group_name_enum::BindGroupName,
        render_pipeline_name_enum::RenderPipelineName,
        sampler_name_enum::SamplerName,
    },
    structures::{
        material::Material,
        render_batch::RenderBatch,
    },
};

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
    pub material_manager: MaterialManager,
    pub render_pipeline_manager: RenderPipelineManager,
    pub square_vertex_buffer: Buffer,
    pub square_index_buffer: Buffer,
    pub index_format: IndexFormat,
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

        let execute_dir = current_exe().unwrap().parent().unwrap().to_path_buf();

        let texture_atlas_file_path = execute_dir.clone().join("assets/texture_atlases/texture_atlas_1/texture_atlas_1.ktx2"); 

        let texture_atlas_meta_path = execute_dir.join("assets/texture_atlases/texture_atlas_1/texture_atlas_1.json");

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
            &[&bind_group_layout_manager.get_bind_group_layout(&BindGroupLayoutName::DefaultBindGroupLayout).unwrap()], 
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
    
        let texture_for_default_material = texture_atlas_manager.get_texture_info(1, 1).unwrap();

        let default_material = Material {
            uv_scale: texture_for_default_material.uv_scale,
            uv_offset: texture_for_default_material.uv_offset,
            bind_group: bind_group_manager.get_bind_group(BindGroupName::TextureAtlas1BindGroup).unwrap(),
            render_pipeline: render_pipeline_manager.get_render_pipeline(RenderPipelineName::DefaultRenderPipeline).unwrap(),
        };

        material_manager.add_material(0, Rc::new(default_material));

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
            material_manager: material_manager,
            square_vertex_buffer: square_vertex_buffer,
            square_index_buffer: square_index_buffer,
            index_format: IndexFormat::Uint16,
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
        
        for render_batch in render_batches {
            render_pass.set_pipeline(&render_batch.render_pipeline);
            render_pass.set_bind_group(0, render_batch.bind_group.as_ref(), &[]);
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

    }
 
}
