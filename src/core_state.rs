use std::sync::Arc;
use wgpu::{
    Instance, InstanceDescriptor, 
    Device, DeviceDescriptor, 
    Adapter, RequestAdapterOptions, 
    Queue, 
    TextureFormat,
    Surface, SurfaceConfiguration,
    TextureUsages, CompositeAlphaMode, PresentMode,
};
use winit::{
    window::Window,
};


pub struct CoreState{
    pub instance: Instance,
    pub window: Arc<Window>,
    pub device: Device,
    pub adapter: Adapter,
    pub queue: Queue,
    pub surface: Surface<'static>,
    pub surface_configuration: SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub surface_texture_format: TextureFormat,
}

impl CoreState {
    pub async fn new(window: Arc<Window>) -> Self {
        let instance = Instance::new(&InstanceDescriptor::default());
        let adapter = instance
            .request_adapter(&RequestAdapterOptions::default())
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(&DeviceDescriptor::default())
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
 

        Self {
            instance: instance,
            window: window,
            device: device,
            adapter: adapter,
            queue: queue,
            surface: surface,
            surface_configuration: surface_configuration,
            size: size,
            surface_texture_format: surface_texture_format,
        }
    }
}
