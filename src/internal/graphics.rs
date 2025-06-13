mod assets;
mod pipeline;
mod sprite;
mod texture;

use std::collections::HashMap;

use super::window::Window;
use assets::AssetsManager;
use pipeline::RenderPipeline;

struct GraphicsInternal {
    surface: wgpu::Surface<'static>,
    config: wgpu::SurfaceConfiguration,
    device: wgpu::Device,
    queue: wgpu::Queue,
}

impl GraphicsInternal {
    pub fn new(window: &Window) -> Self {
        let instance_desc = wgpu::InstanceDescriptor {
            // the backend that wgpu will use, like Vulkan, Metal, or DX12
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        };

        // WGPU lib entry point, first thing to do when using wgpu
        let instance = wgpu::Instance::new(&instance_desc);

        // Presentable surface, used to draw to the screen. eg: window
        let surface = instance.create_surface(window.instance().unwrap()).unwrap();

        let adapter_desc = wgpu::RequestAdapterOptionsBase {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        };

        // Handle to a physical graphics. Used to connect to device
        let adapter = pollster::block_on(instance.request_adapter(&adapter_desc)).unwrap();

        let device_desc = wgpu::DeviceDescriptor {
            label: Some("device"),
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::default(),
            memory_hints: wgpu::MemoryHints::default(),
            trace: wgpu::Trace::Off,
        };

        // Device - connection to a graphic device
        // Queue - Handle to command queue on a device
        let (device, queue) = pollster::block_on(adapter.request_device(&device_desc)).unwrap();

        let surface_capabilities = surface.get_capabilities(&adapter);

        // List of supported formats to use in adapter. The first item is preferred. Here we get
        // the first SRGB format.
        let surface_format = surface_capabilities
            .formats
            .iter()
            .copied()
            .filter(|f| f.is_srgb())
            .next()
            .unwrap();

        let size = window.size();
        let config = wgpu::SurfaceConfiguration {
            // Only usage supported for surface
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width(),
            height: size.height(),
            present_mode: surface_capabilities.present_modes[0],
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
            // Desired maximum number of frames that presentation engine should enqueue in advance
            // Typical values range from 3 to 1, but higher values can be used
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        return Self {
            config,
            surface,
            device,
            queue,
        };
    }
}

pub struct Graphics {
    internal: Option<GraphicsInternal>,
    assets_manager: AssetsManager,
    pipelines: HashMap<String, Box<dyn RenderPipeline>>,
}

impl Graphics {
    pub fn new() -> Self {
        return Self {
            internal: None,
            assets_manager: AssetsManager::new(),
            pipelines: HashMap::new(),
        };
    }

    pub fn run(&mut self, window: &Window) {
        let internal = Some(GraphicsInternal::new(window));
        self.internal = internal;
    }

    pub fn render(&mut self) {
        if self.internal.is_none() {
            panic!("Graphics not initialized");
        }

        let internal = self.internal.as_ref().unwrap();

        // The next texture to be presented to the surface
        let drawable = internal.surface.get_current_texture().unwrap();
        let image_view_desc = wgpu::TextureViewDescriptor::default();
        let image_view = drawable.texture.create_view(&image_view_desc);

        let encoder_desc = wgpu::CommandEncoderDescriptor {
            label: Some("encoder"),
        };

        // Encodes GPU operations. Can record RenderPass and ComputePass, and transfer operations
        // between driver-managed resources like Buffers and Textures
        // When finished recording, call `finish()` obtain a `CommandBuffer` that can be submitted to the queue
        let mut encoder = internal.device.create_command_encoder(&encoder_desc);

        let color_attachment = wgpu::RenderPassColorAttachment {
            view: &image_view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
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

        {
            let mut render_pass = encoder.begin_render_pass(&render_pass_desc);

            for pipeline in self.pipelines.values() {
                pipeline.render(&mut render_pass);
            }
        }

        internal.queue.submit(std::iter::once(encoder.finish()));

        drawable.present();
    }

    pub fn add_pipeline(&mut self, name: &str, pipeline: Box<dyn RenderPipeline>) {
        self.pipelines.insert(name.to_string(), pipeline);
    }
}

// fn build_pipeline(
//     device: &wgpu::Device,
//     config: &wgpu::SurfaceConfiguration,
//     bind_groups_layouts: &[&wgpu::BindGroupLayout],
// ) -> wgpu::RenderPipeline {
//     let shader_desc = wgpu::include_wgsl!("../shaders/tile.wgsl");
//     let shader = device.create_shader_module(shader_desc);
//
//     let layout_desc = wgpu::PipelineLayoutDescriptor {
//         label: Some("pipeline_layout"),
//         bind_group_layouts: bind_groups_layouts,
//         push_constant_ranges: &[],
//     };
//
//     let layout = device.create_pipeline_layout(&layout_desc);
//
//     let color_state = &[Some(wgpu::ColorTargetState {
//         format: config.format,
//         blend: Some(wgpu::BlendState::ALPHA_BLENDING),
//         write_mask: wgpu::ColorWrites::ALL,
//     })];
//
//     let desc = wgpu::RenderPipelineDescriptor {
//         label: Some("pipeline"),
//         layout: Some(&layout),
//         vertex: wgpu::VertexState {
//             module: &shader,
//             entry_point: Some("vs_main"),
//             buffers: &[render::vertex::Vertex::desc(), TileDrawCommand::desc()],
//             compilation_options: wgpu::PipelineCompilationOptions::default(),
//         },
//         fragment: Some(wgpu::FragmentState {
//             module: &shader,
//             entry_point: Some("fs_main"),
//             targets: color_state,
//             compilation_options: wgpu::PipelineCompilationOptions::default(),
//         }),
//         primitive: wgpu::PrimitiveState {
//             // How to interpret the vertices. Here each set of 3 vertices composes a new triangle
//             topology: wgpu::PrimitiveTopology::TriangleList,
//             strip_index_format: None,
//             // Triangles drawn in clockwise order is considered to be front-facing. So is not discarded by the culling
//             front_face: wgpu::FrontFace::Ccw,
//             // Determines whether certain faces of 3D objects should be rendered or discarded.
//             cull_mode: None,
//             polygon_mode: wgpu::PolygonMode::Fill,
//             unclipped_depth: false,
//             conservative: false,
//         },
//         depth_stencil: None,
//         // Is used to avoid aliasing
//         multisample: wgpu::MultisampleState {
//             count: 1,
//             mask: !0,
//             alpha_to_coverage_enabled: false,
//         },
//         multiview: None,
//         cache: None,
//     };
//
//     let pipeline = device.create_render_pipeline(&desc);
//     return pipeline;
// }

pub struct GraphicsAPI<'a> {
    instance: &'a Graphics,
}

impl<'a> GraphicsAPI<'a> {
    pub fn with(i: &'a Graphics) -> GraphicsAPI<'a> {
        return Self { instance: i };
    }
}
