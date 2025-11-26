use std::{
    sync::Arc,
    mem::size_of,
};
use winit::{window::Window};
use wgpu::{
    Device, Queue, Surface, Instance, InstanceDescriptor,
    RequestAdapterOptions, DeviceDescriptor, CommandEncoderDescriptor, RenderPassDescriptor,
    RenderPassColorAttachment, Operations, SurfaceConfiguration, RenderPipelineDescriptor,
    PipelineLayoutDescriptor, RenderPipeline, ShaderModuleDescriptor, 
    ShaderSource, VertexState, PipelineCompilationOptions, FragmentState, PrimitiveState, PrimitiveTopology, MultisampleState,
    Buffer, BufferUsages, VertexBufferLayout, VertexAttribute, VertexFormat, BufferAddress, VertexStepMode,
    Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages, TexelCopyTextureInfo, TexelCopyBufferLayout,
    Origin3d, TextureAspect, TextureViewDescriptor, SamplerDescriptor, AddressMode, FilterMode,
    BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, SamplerBindingType, TextureSampleType, TextureViewDimension,
    ShaderStages, BindGroupDescriptor, BindGroupEntry, BindingResource, BindGroup, BufferBindingType,
    util::{BufferInitDescriptor, DeviceExt}, 
};
use winit::{
    event_loop::ActiveEventLoop,
    keyboard::KeyCode
};
use image::{GenericImageView};
use cgmath::Vector3;
use crate::camera::{
   Camera, CameraUniform, CameraController, 
};


#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
}

impl Vertex {
    fn get_descriptor() -> VertexBufferLayout<'static>{
        const ATTRIBUTES: &[VertexAttribute] = &[ 
            VertexAttribute {
                offset: 0,
                shader_location: 0,
                format: VertexFormat::Float32x3,
            },
            VertexAttribute {
                offset: size_of::<[f32; 3]>() as BufferAddress,
                shader_location: 1,
                format: VertexFormat::Float32x2,
            },
        ];

        let vertext_buffer_layout = VertexBufferLayout {
            array_stride: size_of::<Vertex>() as BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: ATTRIBUTES
        };

        return vertext_buffer_layout;
    }
}

const VERTICES: &[Vertex] = &[
    Vertex { position: [0.0, 0.5, 0.0], tex_coords: [0.5, 0.0] },
    Vertex { position: [-0.5, -0.5, 0.0], tex_coords: [0.0, 1.0] },
    Vertex { position: [0.5, -0.5, 0.0], tex_coords: [1.0, 1.0] },
];




pub struct State {
    window: Arc<Window>,
    device: Device,
    queue: Queue,
    size: winit::dpi::PhysicalSize<u32>,
    surface: Surface<'static>,
    surface_format: TextureFormat,
    render_pipeline: RenderPipeline,
    vertex_buffer: Buffer,
    num_vertices: u32,
    bind_group: BindGroup,
    camera: Camera,
    camera_uniform: CameraUniform,
    camera_buffer: Buffer,
    camera_bind_group: BindGroup,
    camera_controller: CameraController,
}

impl State {
    pub async fn new(window: Arc<Window>) -> State {
        let instance = Instance::new(&InstanceDescriptor::default());
        let adapter = instance
            .request_adapter(&RequestAdapterOptions::default())
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(&DeviceDescriptor::default())
            .await
            .unwrap();

        let size = window.inner_size();

        let surface = instance.create_surface(window.clone()).unwrap();
        let cap = surface.get_capabilities(&adapter);
        let surface_format = cap.formats[0];

        let surface_config = SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            // Request compatibility with the sRGB-format texture view we‘re going to create later.
            view_formats: vec![surface_format.add_srgb_suffix()],
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            width: size.width,
            height: size.height,
            desired_maximum_frame_latency: 2,
            present_mode: wgpu::PresentMode::AutoVsync,
        };
        surface.configure(&device, &surface_config);

        let diffuse_bytes = include_bytes!("../tree.png");
        let diffuse_image = image::load_from_memory(diffuse_bytes).unwrap();
        let diffuse_rgba = diffuse_image.to_rgba8();
        
        let dimensions = diffuse_image.dimensions();

        let texture_size = Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        let texture_descriptor = TextureDescriptor {
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            usage: TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST,
            label: Some("diffuse_texture"),
            view_formats: &[],
        };

        let diffuse_texture = device.create_texture(&texture_descriptor);

        let texel_copy_texture_info = TexelCopyTextureInfo {
            texture: &diffuse_texture,
            mip_level: 0,
            origin: Origin3d::ZERO,
            aspect: TextureAspect::All
        }; 

        let texel_copy_buffer_layout = TexelCopyBufferLayout {
            offset: 0,
            bytes_per_row: Some(4 * dimensions.0),
            rows_per_image: Some(dimensions.1),
        };

        queue.write_texture(
            texel_copy_texture_info, 
            &diffuse_rgba, 
            texel_copy_buffer_layout, 
            texture_size
        );

        let diffuse_texture_view = diffuse_texture.create_view(&TextureViewDescriptor::default());

        let sampler_descriptor = SamplerDescriptor {
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            address_mode_w: AddressMode::ClampToEdge, 
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Nearest,
            mipmap_filter: FilterMode::Nearest,
            ..Default::default()
        };

        let diffuse_sampler = device.create_sampler(&sampler_descriptor);

        const BIND_GROUP_LAYOUT_DESCRIPTOR_ENTRIES: &[BindGroupLayoutEntry] = &[
            BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::FRAGMENT,
                count: None,
                ty: BindingType::Texture { 
                    sample_type: TextureSampleType::Float { filterable: true }, 
                    view_dimension: TextureViewDimension::D2, 
                    multisampled: false, 
                },
            },
            BindGroupLayoutEntry {
                binding: 1,
                visibility: ShaderStages::FRAGMENT,
                count: None,
                ty: BindingType::Sampler(SamplerBindingType::Filtering)
            },
        ];

        let bind_group_layout_descriptor = BindGroupLayoutDescriptor {
            label: Some("texture_bind_group_layout"),
            entries: BIND_GROUP_LAYOUT_DESCRIPTOR_ENTRIES,
        };

        let texture_bind_group_layout = device.create_bind_group_layout(&bind_group_layout_descriptor);

        let diffuse_bind_group = device.create_bind_group(
            &BindGroupDescriptor {
                layout: &texture_bind_group_layout,
                entries: &[
                    BindGroupEntry {
                        binding: 0,
                        resource: BindingResource::TextureView(&diffuse_texture_view)
                    },
                    BindGroupEntry {
                        binding: 1,
                        resource: BindingResource::Sampler(&diffuse_sampler)
                    },
                ],
                label: Some("diffuse_bind_group")
            }
        );

        let camera = Camera {
            eye: (0.0, 1.0, 2.0).into(),
            target: (0.0, 0.0, 0.0).into(),
            up: Vector3::unit_y(),
            aspect: surface_config.width as f32 / surface_config.height as f32,
            fovy: 45.0,
            z_near: 0.1,
            z_far: 100.0
        };

        let mut camera_uniform = CameraUniform::new();
        camera_uniform.update_view_projection(&camera);

        let camera_buffer = device.create_buffer_init(
            &BufferInitDescriptor { 
                label: Some("Camera Buffer"), 
                contents: bytemuck::cast_slice(&[camera_uniform]), 
                usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST, 
            }
        );

        let camera_bind_group_layout = device.create_bind_group_layout(
            &BindGroupLayoutDescriptor { 
                label: Some("camera_bind_group_layout"), 
                entries: &[
                    BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStages::VERTEX,
                        count: None,
                        ty: BindingType::Buffer { 
                            ty: BufferBindingType::Uniform, 
                            has_dynamic_offset: false,
                            min_binding_size: None 
                        },
                    },
                ],  
            },
        );

        let camera_bind_group = device.create_bind_group(
            &BindGroupDescriptor { 
                label: Some("camera_bind_group"), 
                layout: &camera_bind_group_layout, 
                entries: &[
                    BindGroupEntry {
                        binding: 0,
                        resource: camera_buffer.as_entire_binding(),
                    },
                ], 
            }
        );

        let shader_module_descriptor = ShaderModuleDescriptor {
            label: Some("Shader"),
            source: ShaderSource::Wgsl(include_str!("shader.wgsl").into()), 
        };

        let shader_module = device.create_shader_module(shader_module_descriptor);

        let pipe_line_layout_descriptor = PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[
                &texture_bind_group_layout,
                &camera_bind_group_layout,
            ],
            push_constant_ranges: &[],
        };

        let pipe_line_layout = device.create_pipeline_layout(&pipe_line_layout_descriptor);

        let vertex_state = VertexState {
            module: &shader_module,
            entry_point: Some("vs_main"),
            buffers: &[Vertex::get_descriptor()],
            compilation_options: PipelineCompilationOptions::default(), 
        };

        let fragment_state = FragmentState {
            module: &shader_module,
            entry_point: Some("fs_main"),
            targets: &[Some(wgpu::ColorTargetState {
                format: surface_config.format,
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

        let camera_controller = CameraController::new(0.2);

        let state = State {
            window,
            device,
            queue,
            size,
            surface,
            surface_format,
            render_pipeline,
            vertex_buffer: vertex_buffer,
            num_vertices: num_vertices,
            bind_group: diffuse_bind_group,
            camera: camera,
            camera_bind_group: camera_bind_group,
            camera_buffer: camera_buffer,
            camera_uniform: camera_uniform,
            camera_controller: camera_controller,
        }; 

        return state;
    }

    pub fn handle_key(&mut self, event_loop: &ActiveEventLoop, code: KeyCode, is_pressed: bool) {
        if code == KeyCode::Escape && is_pressed {
            event_loop.exit();
        }
        else {
            self.camera_controller.handle_key(code, is_pressed);
        }
    }

    pub fn update(&mut self) {
        self.camera_controller.update_camera(&mut self.camera);
        self.camera_uniform.update_view_projection(&self.camera);
        self.queue.write_buffer(&self.camera_buffer, 0, bytemuck::cast_slice(&[self.camera_uniform]));
    }

    pub fn get_window(&self) -> &Window {
        &self.window
    }

    pub fn configure_surface(&self) {
        
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;

        // reconfigure the surface
        self.configure_surface();
    }

    pub fn render(&mut self) {
        let surface_current_texture = self.surface.get_current_texture().unwrap();

        let current_texture =  surface_current_texture.texture.clone(); 

        let texture_view_descriptor = TextureViewDescriptor {
            format: Some(self.surface_format.add_srgb_suffix()),
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
       
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.set_bind_group(1, &self.camera_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.draw(0..self.num_vertices, 0..1);

        drop(render_pass);

        let command_buffer = command_encoder.finish();

        self.queue.submit([command_buffer]);

        surface_current_texture.present(); 
    }
}


