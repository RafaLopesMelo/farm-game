use std::sync::Arc;

use winit::window::Window;

pub struct State<'a> {
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
}

impl<'a> State<'a> {
    pub async fn new(window: Arc<Window>) -> Self {
        let size = window.inner_size();

        let instance_desc = wgpu::InstanceDescriptor {
            // the backend that wgpu will use, like Vulkan, Metal, or DX12
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        };

        // WGPU lib entry point, first thing to do when using wgpu
        let instance = wgpu::Instance::new(instance_desc);

        // Presentable surface, used to draw to the screen. eg: window
        let surface = instance.create_surface(window).unwrap();

        let adapter_desc = wgpu::RequestAdapterOptionsBase {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        };

        // Handle to a physical graphics. Used to connect to device
        let adapter = instance.request_adapter(&adapter_desc).await.unwrap();

        let device_desc = wgpu::DeviceDescriptor {
            label: Some("device"),
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::default(),
            memory_hints: wgpu::MemoryHints::default(),
        };

        // Device - connection to a graphic device
        // Queue - Handle to command queue on a device
        let (device, queue) = adapter.request_device(&device_desc, None).await.unwrap();

        let surface_capabilities = surface.get_capabilities(&adapter);
        let surface_format = surface_capabilities
            .formats
            .iter()
            .copied()
            .filter(|f| f.is_srgb())
            .next()
            .unwrap();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_capabilities.present_modes[0],
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        return Self {
            surface,
            device,
            queue,
            config,
            size,
        };
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let drawable = self.surface.get_current_texture()?;
        let image_view_desc = wgpu::TextureViewDescriptor::default();
        let image_view = drawable.texture.create_view(&image_view_desc);

        let encoder_desc = wgpu::CommandEncoderDescriptor {
            label: Some("encoder"),
        };
        let mut encoder = self.device.create_command_encoder(&encoder_desc);

        let color_attachment = wgpu::RenderPassColorAttachment {
            view: &image_view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color {
                    r: 0.1,
                    g: 0.2,
                    b: 0.3,
                    a: 1.0,
                }),
                store: wgpu::StoreOp::Store,
            },
        };

        let render_pass_desc = wgpu::RenderPassDescriptor {
            label: Some("render_pass"),
            color_attachments: &[Some(color_attachment)],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        };

        encoder.begin_render_pass(&render_pass_desc);
        self.queue.submit(std::iter::once(encoder.finish()));

        drawable.present();

        return Ok(());
    }
}
