use std::{
    sync::Arc,
    rc::Rc,
};
use wgpu::{
    Instance, InstanceDescriptor, 
    Device, DeviceDescriptor, Features,
    Adapter, RequestAdapterOptions, 
    Queue, 
    TextureFormat, TextureViewDescriptor, CommandEncoderDescriptor, Operations,
    Surface, SurfaceConfiguration, RenderPassDescriptor, RenderPassColorAttachment,
    TextureUsages, CompositeAlphaMode, PresentMode,
    RenderPipelineDescriptor, PipelineLayoutDescriptor, 
    RenderPipeline, VertexState, PipelineCompilationOptions, FragmentState, PrimitiveState, 
    PrimitiveTopology, MultisampleState, Buffer, BufferUsages, 
    util::{BufferInitDescriptor, DeviceExt}, 
};
use winit::{
    window::Window,
    dpi::PhysicalSize, 
};
use crate::{
    vertex::Vertex,
    shader_library::ShaderLibrary,
    managers::{
        texture_manager::TextureManager,
        sampler_manager::SamplerManager,
        bind_group_layout_manager::BindGroupLayoutManager,
        sprite_material_manager::SpriteMaterialManager,
    },
    enums::{
        bind_group_layout_name_enum::BindGroupLayoutName,
    },
    structures::{
        materials::sprite_material::SpriteMaterial,
    },
    functions::{
        bind_group_functions::create_bind_group_for_sprite_material,
    },
};

const VERTICES: &[Vertex] = &[
    Vertex { position: [0.0, 0.5, 0.0], tex_coords: [0.5, 0.0] },
    Vertex { position: [-0.5, -0.5, 0.0], tex_coords: [0.0, 1.0] },
    Vertex { position: [0.5, -0.5, 0.0], tex_coords: [1.0, 1.0] },
];

pub struct RenderState {
    pub instance: Instance,
    pub window: Arc<Window>,
    pub device: Device,
    pub adapter: Adapter,
    pub queue: Queue,
    pub surface: Surface<'static>,
    pub surface_configuration: SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub surface_texture_format: TextureFormat,
    pub shader_library: ShaderLibrary,
    pub texture_manager: TextureManager,
    pub sampler_manager: SamplerManager,
    pub bind_group_layout_manager: BindGroupLayoutManager,
    pub sprite_material_manager: SpriteMaterialManager,
    pub render_pipeline: RenderPipeline,
    pub vertex_buffer: Buffer,
    pub num_vertices: u32, 
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
        let size = window.inner_size();
        let surface_capabilities = surface.get_capabilities(&adapter);
        let surface_texture_format = surface_capabilities.formats[0];

        let surface_configuration = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_texture_format.clone(),
            // Request compatibility with the sRGB-format texture view we‘re going to create later.
            view_formats: vec![surface_texture_format.add_srgb_suffix()],
            alpha_mode: CompositeAlphaMode::Auto,
            width: size.width,
            height: size.height,
            desired_maximum_frame_latency: 2,
            present_mode: PresentMode::AutoVsync,
        };
        surface.configure(&device, &surface_configuration);
 
        let shader_library = ShaderLibrary::new(&device);
        
        let texture_manager = TextureManager::new();

        texture_manager.load_texture("assets/tree.ktx2", &device, &queue);

        let tree_texture_view = texture_manager.get_default_texture_view("tree").unwrap();

        let sampler_manager = SamplerManager::new(&device); 
        
        let default_sampler = sampler_manager.get_sampler("default_sampler").unwrap();

        let bind_group_layout_manager = BindGroupLayoutManager::new(&device);

        let default_sprite_material = Rc::new(SpriteMaterial::new(
            tree_texture_view,
            default_sampler,
        ));

        let default_bind_group_layout = bind_group_layout_manager.get_bind_group_layout(
            &BindGroupLayoutName::DefaultBindGroupLayout
        ).unwrap();

        let default_sprite_material_bind_group = create_bind_group_for_sprite_material(
            &device, 
            &default_bind_group_layout, 
            &default_sprite_material.get_texture_view(), 
            &default_sprite_material.get_sampler(), 
            Some("default_sprite_material_bind_group")
        );

        default_sprite_material.set_bind_group(default_sprite_material_bind_group);

        let sprite_material_manager = SpriteMaterialManager::new();

        sprite_material_manager.add_sprite_material("default_sprite_material", default_sprite_material);  

        // Pipeline config
        let pipe_line_layout_descriptor = PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[
                &default_bind_group_layout,
            ],
            push_constant_ranges: &[],
        };

        let pipe_line_layout = device.create_pipeline_layout(&pipe_line_layout_descriptor);

        let vertex_state = VertexState {
            module: &shader_library.basic_shader_module,
            entry_point: Some("vs_main"),
            buffers: &[Vertex::get_descriptor()],
            compilation_options: PipelineCompilationOptions::default(), 
        };

        let fragment_state = FragmentState {
            module: &shader_library.basic_shader_module,
            entry_point: Some("fs_main"),
            targets: &[Some(wgpu::ColorTargetState {
                format: surface_configuration.format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: PipelineCompilationOptions::default(),
        };

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

        let render_pipeline_description = RenderPipelineDescriptor {
            label: Some("Render pipeline"),
            layout: Some(&pipe_line_layout),
            vertex: vertex_state,
            fragment: Some(fragment_state),
            primitive: primitive_state,
            depth_stencil: None,
            multisample: multisample_state,
            multiview: None,
            cache: None,
        };

        let render_pipeline = device.create_render_pipeline(&render_pipeline_description);

        let buffer_init_descriptor = BufferInitDescriptor {
            label: Some("Vertex buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: BufferUsages::VERTEX,
        };

        let vertex_buffer = device.create_buffer_init(&buffer_init_descriptor);

        let num_vertices = VERTICES.len() as u32;

        let render_state = Self {
            device: device,
            surface_configuration: surface_configuration,
            size: size,
            queue: queue,
            window: window,
            adapter: adapter,
            surface: surface,
            instance: instance,
            surface_texture_format: surface_texture_format,
            shader_library: shader_library,
            texture_manager: texture_manager,
            sampler_manager: sampler_manager,
            bind_group_layout_manager: bind_group_layout_manager,
            sprite_material_manager: sprite_material_manager,
            render_pipeline: render_pipeline,
            vertex_buffer: vertex_buffer,
            num_vertices: num_vertices, 
        };

        return render_state; 
    }

    pub fn render(&mut self) {
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
      
        let default_bind_group = self.sprite_material_manager.get_sprite_material(
            "default_sprite_material"
        ).unwrap().get_bind_group();

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &*default_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.draw(0..self.num_vertices, 0..1);

        drop(render_pass);

        let command_buffer = command_encoder.finish();

        self.queue.submit([command_buffer]);

        surface_current_texture.present(); 
    }

    pub fn reconfigure_surface(&mut self, physical_size: PhysicalSize<u32>) {

    }
 
}
