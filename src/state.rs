use std::{
    sync::Arc,
    rc::Rc,
};
use winit::{window::Window};
use wgpu::{
    CommandEncoderDescriptor, RenderPassDescriptor,
    RenderPassColorAttachment, Operations, RenderPipelineDescriptor, PipelineLayoutDescriptor, 
    RenderPipeline, VertexState, PipelineCompilationOptions, FragmentState, PrimitiveState, 
    PrimitiveTopology, MultisampleState, Buffer, BufferUsages, TextureViewDescriptor,  BindGroupLayoutDescriptor, 
    BindGroupLayoutEntry, BindingType, ShaderStages, BindGroupDescriptor, BindGroupEntry, BindGroup, BufferBindingType,
    util::{BufferInitDescriptor, DeviceExt}, 
};
use winit::{
    event_loop::ActiveEventLoop,
    keyboard::KeyCode
};
use cgmath::Vector3;
use crate::{
    camera::{
        Camera, CameraUniform, CameraController, 
    },
    vertex::Vertex,
    core_state::CoreState,
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

#[allow(dead_code)]
pub struct State {
    core_state: CoreState,
    shader_library: ShaderLibrary,
    texture_manager: TextureManager,
    sampler_manager: SamplerManager,
    bind_group_layout_manager: BindGroupLayoutManager,
    sprite_material_manager: SpriteMaterialManager,
    render_pipeline: RenderPipeline,
    vertex_buffer: Buffer,
    num_vertices: u32, 
    camera: Camera,
    camera_uniform: CameraUniform,
    camera_buffer: Buffer,
    camera_bind_group: BindGroup,
    camera_controller: CameraController,
}

impl State {
    pub async fn new(window: Arc<Window>) -> State {
        let core_state = CoreState::new(window.clone()).await; 
        
        let shader_library = ShaderLibrary::new(&core_state);
        
        let texture_manager = TextureManager::new();

        texture_manager.load_texture("assets/tree.ktx2", &core_state.device, &core_state.queue);

        let tree_texture_view = texture_manager.get_default_texture_view("tree").unwrap();

        let sampler_manager = SamplerManager::new(&core_state.device); 
        
        let default_sampler = sampler_manager.get_sampler("default_sampler").unwrap();

        let bind_group_layout_manager = BindGroupLayoutManager::new(&core_state.device);

        let default_sprite_material = Rc::new(SpriteMaterial::new(
            tree_texture_view,
            default_sampler,
        ));

        let default_bind_group_layout = bind_group_layout_manager.get_bind_group_layout(
            &BindGroupLayoutName::DefaultBindGroupLayout
        ).unwrap();

        let default_sprite_material_bind_group = create_bind_group_for_sprite_material(
            &core_state.device, 
            &default_bind_group_layout, 
            &default_sprite_material.get_texture_view(), 
            &default_sprite_material.get_sampler(), 
            Some("default_sprite_material_bind_group")
        );

        default_sprite_material.set_bind_group(default_sprite_material_bind_group);

        let sprite_material_manager = SpriteMaterialManager::new();

        sprite_material_manager.add_sprite_material("default_sprite_material", default_sprite_material); 

        // Camera config
        let camera = Camera {
            eye: (0.0, 1.0, 2.0).into(),
            target: (0.0, 0.0, 0.0).into(),
            up: Vector3::unit_y(),
            aspect: core_state.surface_configuration.width as f32 / core_state.surface_configuration.height as f32,
            fovy: 45.0,
            z_near: 0.1,
            z_far: 100.0
        };

        let mut camera_uniform = CameraUniform::new();
        camera_uniform.update_view_projection(&camera);

        let camera_buffer = core_state.device.create_buffer_init(
            &BufferInitDescriptor { 
                label: Some("Camera Buffer"), 
                contents: bytemuck::cast_slice(&[camera_uniform]), 
                usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST, 
            }
        );

        let camera_bind_group_layout = core_state.device.create_bind_group_layout(
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

        let camera_bind_group = core_state.device.create_bind_group(
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

        let camera_controller = CameraController::new(0.2); 

        // Pipeline config
        let pipe_line_layout_descriptor = PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[
                &default_bind_group_layout,
                &camera_bind_group_layout,
            ],
            push_constant_ranges: &[],
        };

        let pipe_line_layout = core_state.device.create_pipeline_layout(&pipe_line_layout_descriptor);

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
                format: core_state.surface_configuration.format,
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

        let render_pipeline = core_state.device.create_render_pipeline(&render_pipeline_description);

        let buffer_init_descriptor = BufferInitDescriptor {
            label: Some("Vertex buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: BufferUsages::VERTEX,
        };

        let vertex_buffer = core_state.device.create_buffer_init(&buffer_init_descriptor);

        let num_vertices = VERTICES.len() as u32;

        let state = Self {
            core_state: core_state,
            shader_library: shader_library,
            texture_manager: texture_manager,
            sampler_manager: sampler_manager,
            bind_group_layout_manager: bind_group_layout_manager,
            sprite_material_manager: sprite_material_manager,
            render_pipeline: render_pipeline,
            vertex_buffer: vertex_buffer,
            num_vertices: num_vertices,
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
        self.core_state.queue.write_buffer(&self.camera_buffer, 0, bytemuck::cast_slice(&[self.camera_uniform]));
    }

    pub fn get_window(&self) -> &Window {
        &self.core_state.window
    }

    pub fn configure_surface(&self) {
        
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.core_state.size = new_size;

        // reconfigure the surface
        self.configure_surface();
    }

    pub fn render(&mut self) {
        let surface_current_texture = self.core_state.surface.get_current_texture().unwrap();

        let current_texture =  surface_current_texture.texture.clone(); 

        let texture_view_descriptor = TextureViewDescriptor {
            format: Some(self.core_state.surface_texture_format.add_srgb_suffix()),
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

        let mut command_encoder = self.core_state.device.create_command_encoder(&command_encoder_description); 
        
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
        render_pass.set_bind_group(1, &self.camera_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.draw(0..self.num_vertices, 0..1);

        drop(render_pass);

        let command_buffer = command_encoder.finish();

        self.core_state.queue.submit([command_buffer]);

        surface_current_texture.present(); 
    }
}


