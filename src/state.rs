use std::sync::Arc;

use wgpu::util::DeviceExt;
use winit::window::Window;

use crate::{game, render, world::camera::Camera};

pub struct State<'a> {
    game: game::Game,
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub camera_controller: render::camera::CameraController,
    pub camera_buffer: wgpu::Buffer,
    grass_bind_group: wgpu::BindGroup,
    grass_bind_group_layout: wgpu::BindGroupLayout,
}

impl<'a> State<'a> {
    pub async fn new(window: Arc<Window>) -> Self {
        let game = game::Game::new();

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

        // List of supported formats to use in adapter. The first item is preferred. Here we get
        // the first SRGB format.
        let surface_format = surface_capabilities
            .formats
            .iter()
            .copied()
            .filter(|f| f.is_srgb())
            .next()
            .unwrap();

        let config = wgpu::SurfaceConfiguration {
            // Only usage supported for surface
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_capabilities.present_modes[0],
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
            // Desired maximum number of frames that presentation engine should enqueue in advance
            // Typical valuues range from 3 to 1, but higher values can be used
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        let camera_controller = render::camera::CameraController::new();
        let camera_buffer = build_camera_buffer(&device, game.camera_ref());

        let (grass_bind_group, grass_bind_group_layout) = build_grass_texture(&device, &queue);

        return Self {
            game,
            camera_buffer,
            camera_controller,
            surface,
            device,
            queue,
            config,
            size,
            grass_bind_group,
            grass_bind_group_layout,
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
        // The next texture to be presented to the surface
        let drawable = self.surface.get_current_texture()?;
        let image_view_desc = wgpu::TextureViewDescriptor::default();
        let image_view = drawable.texture.create_view(&image_view_desc);

        let encoder_desc = wgpu::CommandEncoderDescriptor {
            label: Some("encoder"),
        };

        // Encodes GPU operations. Can record RenderPass and ComputePass, and transfer operations
        // between driver-managed resources like Buffers and Textures
        // When finished recording, call `finish()` obtain a `CommandBuffer` that can be submitted to the queue
        let mut encoder = self.device.create_command_encoder(&encoder_desc);

        let (screen_bind_group, screen_bind_group_layout) =
            build_screen_bind_group(&self.device, &self.size);

        let (camera_bind_group, camera_bind_group_layout) =
            build_camera_bind_group(&self.device, &self.camera_buffer);

        let pipeline = build_pipeline(
            &self.device,
            &self.config,
            &[
                &screen_bind_group_layout,
                &camera_bind_group_layout,
                &self.grass_bind_group_layout,
            ],
        );

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

        let vertices = &render::tiles::TileRender::vertices();
        {
            let vertex_buffer_desc = wgpu::util::BufferInitDescriptor {
                label: Some("vertex_buffer"),
                contents: bytemuck::cast_slice(vertices),
                usage: wgpu::BufferUsages::VERTEX,
            };
            let vertex_buffer = self.device.create_buffer_init(&vertex_buffer_desc);

            let world_render =
                render::world::WorldRender::new(self.game.world_ref(), self.game.camera_ref());
            let instances = world_render.tiles();

            let instance_buffer_desc = wgpu::util::BufferInitDescriptor {
                label: Some("instance_buffer"),
                contents: bytemuck::cast_slice(&instances),
                usage: wgpu::BufferUsages::VERTEX,
            };
            let instance_buffer = self.device.create_buffer_init(&instance_buffer_desc);

            // List of render commands in a command encoder. A render pass may contain any number of
            // drawing commands, and before/between each command the render state may be updated
            // however you wish
            let mut render_pass = encoder.begin_render_pass(&render_pass_desc);

            render_pass.set_pipeline(&pipeline);

            render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
            render_pass.set_vertex_buffer(1, instance_buffer.slice(..));

            render_pass.set_bind_group(0, &screen_bind_group, &[]);
            render_pass.set_bind_group(1, &camera_bind_group, &[]);
            render_pass.set_bind_group(2, &self.grass_bind_group, &[]);

            render_pass.draw(0..vertices.len() as u32, 0..instances.len() as u32);
        }
        self.queue.submit(std::iter::once(encoder.finish()));

        drawable.present();

        return Ok(());
    }

    pub fn update_camera(&mut self) {
        let intention = self.camera_controller.build_walk_intention();
        let new_coords = self.game.perform_walk(intention);

        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[new_coords.to_array()]),
        );
    }
}

fn build_pipeline(
    device: &wgpu::Device,
    config: &wgpu::SurfaceConfiguration,
    bind_groups_layouts: &[&wgpu::BindGroupLayout],
) -> wgpu::RenderPipeline {
    let shader_desc = wgpu::include_wgsl!("shaders/tile.wgsl");
    let shader = device.create_shader_module(shader_desc);

    let layout_desc = wgpu::PipelineLayoutDescriptor {
        label: Some("pipeline_layout"),
        bind_group_layouts: bind_groups_layouts,
        push_constant_ranges: &[],
    };

    let layout = device.create_pipeline_layout(&layout_desc);

    let color_state = &[Some(wgpu::ColorTargetState {
        format: config.format,
        blend: Some(wgpu::BlendState::REPLACE),
        write_mask: wgpu::ColorWrites::ALL,
    })];

    let desc = wgpu::RenderPipelineDescriptor {
        label: Some("pipeline"),
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: &[
                render::vertex::Vertex::desc(),
                render::tiles::TileRender::desc(),
            ],
            compilation_options: wgpu::PipelineCompilationOptions::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: color_state,
            compilation_options: wgpu::PipelineCompilationOptions::default(),
        }),
        primitive: wgpu::PrimitiveState {
            // How to interpret the vertices. Here each set of 3 vertices composes a new triangle
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            // Triangles drawn in clockwise order is considered to be front-facing. So is not discarded by the culling
            front_face: wgpu::FrontFace::Ccw,
            // Determines whether certain faces of 3D objects should be rendered or discarded.
            cull_mode: None,
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        },
        depth_stencil: None,
        // Is used to avoid aliasing
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
        cache: None,
    };

    let pipeline = device.create_render_pipeline(&desc);
    return pipeline;
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct ScreenUniform {
    size: [u32; 2],
}

fn build_screen_bind_group(
    device: &wgpu::Device,
    size: &winit::dpi::PhysicalSize<u32>,
) -> (wgpu::BindGroup, wgpu::BindGroupLayout) {
    let content = &[ScreenUniform {
        size: [size.width as u32, size.height as u32],
    }];

    let buffer_desc = wgpu::util::BufferInitDescriptor {
        label: Some("screen_buffer"),
        contents: bytemuck::cast_slice(content),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    };
    let buffer = device.create_buffer_init(&buffer_desc);

    let layout_desc = wgpu::BindGroupLayoutDescriptor {
        label: Some("screen_bind_group_layout"),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
    };
    let layout = device.create_bind_group_layout(&layout_desc);

    let bind_group_desc = wgpu::BindGroupDescriptor {
        label: Some("screen_bind_group"),
        layout: &layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: buffer.as_entire_binding(),
        }],
    };
    let bind_group = device.create_bind_group(&bind_group_desc);

    return (bind_group, layout);
}

fn build_camera_buffer(device: &wgpu::Device, camera: &Camera) -> wgpu::Buffer {
    let content = &camera.to_tuple();

    let buffer_desc = wgpu::util::BufferInitDescriptor {
        label: Some("camera_buffer"),
        contents: bytemuck::cast_slice(content),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    };
    let buffer = device.create_buffer_init(&buffer_desc);

    return buffer;
}

fn build_camera_bind_group(
    device: &wgpu::Device,
    camera: &wgpu::Buffer,
) -> (wgpu::BindGroup, wgpu::BindGroupLayout) {
    let layout_desc = wgpu::BindGroupLayoutDescriptor {
        label: Some("camera_bind_group_layout"),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
    };
    let layout = device.create_bind_group_layout(&layout_desc);

    let bind_group_desc = wgpu::BindGroupDescriptor {
        label: Some("camera_bind_group"),
        layout: &layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: camera.as_entire_binding(),
        }],
    };
    let bind_group = device.create_bind_group(&bind_group_desc);

    return (bind_group, layout);
}

fn build_grass_texture(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
) -> (wgpu::BindGroup, wgpu::BindGroupLayout) {
    let buffer = include_bytes!("../assets/grass.jpg");
    let img = image::load_from_memory(buffer).unwrap();
    let rgba = img.to_rgba8();

    use image::GenericImageView;
    let dimensions = img.dimensions();

    let size = wgpu::Extent3d {
        width: dimensions.0,
        height: dimensions.1,
        depth_or_array_layers: 1,
    };

    let texture_desc = wgpu::TextureDescriptor {
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        label: Some("grass_texture"),
        view_formats: &[],
    };

    let texture = device.create_texture(&texture_desc);

    queue.write_texture(
        wgpu::ImageCopyTexture {
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        &rgba,
        wgpu::ImageDataLayout {
            offset: 0,
            bytes_per_row: Some(4 * dimensions.0), // 4 bytes per pixel
            rows_per_image: Some(dimensions.1),
        },
        size,
    );

    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

    let sampler_desc = wgpu::SamplerDescriptor {
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Nearest,
        mipmap_filter: wgpu::FilterMode::Nearest,
        ..Default::default()
    };
    let sampler = device.create_sampler(&sampler_desc);

    let bind_group_layout_desc = wgpu::BindGroupLayoutDescriptor {
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    multisampled: false,
                    view_dimension: wgpu::TextureViewDimension::D2,
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
        ],
        label: Some("grass_bind_group_layout"),
    };
    let bind_group_layout = device.create_bind_group_layout(&bind_group_layout_desc);

    let bind_group_desc = wgpu::BindGroupDescriptor {
        layout: &bind_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::TextureView(&view),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::Sampler(&sampler),
            },
        ],
        label: Some("grass_bind_group"),
    };

    let bind_group = device.create_bind_group(&bind_group_desc);
    return (bind_group, bind_group_layout);
}
